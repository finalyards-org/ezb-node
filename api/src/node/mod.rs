/*
* Design:
*   - Singleton; there's only one Node in a running system
*   - Methods, not globals. Helps e.g. in IDE auto-completion: seeing only meaningful methods, based on context.
*       In comparison, C API has everything as global functions, which is overwhelming.
*   - Application RTOS task does not know about the underlying Zigbee task. The application is based
*       on Embassy's 'async' for its own concurrency.
*/
mod zigbee_task;

use std::sync::atomic::{AtomicBool, Ordering};
use zigbee_task::zigbee_task;

mod zigbee_dev;
pub(crate) use zigbee_dev::ZigbeeDev;

mod access;
#[cfg(feature = "_remote_any")]
pub use access::*;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel
};

use ezb_node_raw::{
    esp_zigbee_lock_acquire,
    esp_zigbee_lock_release,
    ezb_bdb_is_factory_new,
    ezb_bdb_open_network,
    ezb_bdb_start_top_level_commissioning,
    ezb_nwk_get_current_channel,
    ezb_nwk_get_extended_panid,
    ezb_nwk_get_extended_address,
    ezb_nwk_get_panid,
    ezb_nwk_get_short_address,
};

use crate::{
    AppSignal,
    BdbMode,
    Config,
    Error,
    EzbError,
    IeeeAddr,
    ZclError,
    ZclEvent,
};

// Channel:
//  - fed by the Zigbee task (or ISR, not sure); lock not needed within an ISR
//  - consumed by the _application task_; async; lock for every call
//
pub(self) static CHANNEL: Channel<CriticalSectionRawMutex, Payload, 10> = Channel::new();
    // tx: in 'zigbee_task'
    // rx: us, application task

const ZIGBEE_TASK_NAME: &str = "Zigbee_main";

//const ZIGBEE_TASK_STACK_SIZE: usize = 10 * 1024;   // note: C example uses 4k, Rust may need more
//  Let's just use the default, unless there's a specific reason to be ...specific.

#[derive(Debug, Clone)]
enum Payload {
    AppSignal(AppSignal),
    ZclEvent(ZclEvent)
}

/**
* Provides access to the radio. The struct implementing this decides the role (Coordinator/Router/EndDevice)
* that this has.
*
* tbd. We can provide 'Coordinator: Node' etc. traits, later, as a building block for custom nodes.
*
* A **singleton** - you can create only one and it has "endless" (static) lifespan.
*
* @thread Application RTOS thread, throughout.
*/
#[allow(async_fn_in_trait)]
pub trait Node {
    // Provided by the upper trait:
    async fn on_app_signal(&mut self, app_signal: AppSignal);
    fn on_zcl_event(&mut self, zcl_event: ZclEvent);

    /**
    * Initialize the Zigbee side of things.
    *
    * Launches the task that receives Zigbee events, converts them to Rust-friendly structs, and
    * sends them over to us, the application task.
    */
    fn init(cfg: &'static Config) -> Result<(), crate::Error> where Self: Sized {

        let _handle = std::thread::Builder::new()
            .name(ZIGBEE_TASK_NAME.to_string())    // visible e.g. in FreeRTOS monitoring
            //.stack_size(ZIGBEE_TASK_STACK_SIZE)

            .spawn(move || zigbee_task(cfg))
            .map_err(|e| { Error::SpawnFailed(e) })?;

        Ok(())
    }

    /**
    * Run the event loop, passing Zigbee events from the channel (an implementation detail) to the
    * handlers.
    */
    async fn run_loop(mut self) -> ! where Self: Sized {
        let rx = CHANNEL.receiver();

        loop {
            let x = rx.receive().await;
            log::info!("Received: {:?}", &x);  // DEBUG

            match x {
                Payload::AppSignal(x) =>  self.on_app_signal(x) .await,
                Payload::ZclEvent(x) => self.on_zcl_event(x)
            }
        }
    }
}

/// Provide applications' event handlers access to (parts of) the C level API.
///
impl<T: Node> NodeApi for T {}

/// Access to methods of the 'esp_zigbee_lib' from a context (application RTOS thread) that
/// requires locking.
///
/// @design
///     We could provide a '.guarded(|x| { x.aaa(); x.bbb() } )' interface that has the benefit
///     of allowing multiple calls per one lock. Currently, we don't. Everything is guarded,
///     but the application developer _does not need_ to know about it.
///
pub trait NodeApi {
    /**
    * Get the PAN ID of the network.
    */
    fn get_panid(&self) -> u16 {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            ezb_nwk_get_panid()
        }
    }

    /**
    * Get the extended PAN ID of the network.
    */
    fn get_extended_panid(&self) -> IeeeAddr {
        let _guard = ZigbeeGuard::acquire();
        let v = unsafe {
            ezb_nwk_get_extended_panid()
        };
        IeeeAddr(v)
    }

    /**
    * Get the extended (IEEE) address of the device.
    */
    fn get_extended_address(&self) -> IeeeAddr {
        let _guard = ZigbeeGuard::acquire();
        let v = unsafe {
            ezb_nwk_get_extended_address()
        };
        IeeeAddr(v)
    }

    /**
    * Get the network (short) address of the device.
    */
    fn get_short_address(&self) -> u16 {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            ezb_nwk_get_short_address()
        }
    }

    /**
    * Get the currently used channel.
    */
    fn get_current_channel(&self) -> u8 {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            ezb_nwk_get_current_channel()
        }
    }

    /**
    * @brief  Start top level commissioning procedure with specified mode mask.
    *
    * @note This function is intended to be a lower level, common tool. Applications should likely use
    *       node type -specific helper methods (and not directly the modes).
    *
    * Note: Some of the modes apply only to certain node types (NETWORK_FORMATION only to Coordinator
    *       role).
    *
    * Note: Modes are bit patterns.
    *
    * @note It's a little bit strange that the C library only lists 'EZB_ERR_NONE' as a return value.
    *       Does it always succeed? Is this a claim that future version might give errors? #feedback
    */
    fn start_top_level_commissioning(&self, mask: BdbMode) -> Result<(),EzbError> {
        let _guard = ZigbeeGuard::acquire();

        EzbError::wrap(unsafe {
            ezb_bdb_start_top_level_commissioning(mask.into())
        })
    }

    /**
    * Get the "factory new" status.
    *
    * A factory new device:
    *   - is not part of any network
    *   - has no stored keys
    *   - all settings (bindings, intervals) are at their defaults
    */
    fn is_factory_new(&self) -> bool {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            ezb_bdb_is_factory_new()
        }
    }

    /**
    * Open the network for joining.
    *
    * @param for_how_long After this time, permit expires.
    *
    * @note Since the call is synchronous (it only starts the join), error situations are not
    *       well understood: the C side can err on "not memory" (panic might be justified?) and
    *       "on failed" - which is inappropriately vague (again, panic).
    */
    fn open_network(&self, for_how_long: embassy_time::Duration) -> () {
        let secs: u8 = for_how_long.as_secs() as u8;

        let _guard = ZigbeeGuard::acquire();
        EzbError::wrap( unsafe {
            ezb_bdb_open_network(secs)
                // docs don't care to mention the unit (2.0.5), but examples use seconds. #feedback

        }).unwrap_or_else(|e| {
            panic!("'ezb_bdb_open_network' failed with: {}", e);
        });
    }

    /**
    * @brief Perform "factory reset" procedure
    * @note The device will completely erase the `zb_storage` partition and then restart
    */
    #[cfg(false)]
    fn factory_reset(&self) {
        let _guard = ZigbeeGuard::acquire();
        unsafe {
            esp_zb_factory_reset()
        }
    }
}

/// All 'Node' methods are outside of the 'esp_zigbee_lib' callbacks, since they operate in another
/// RTOS thread and received the information via a channel. Thus, they need locks when accessing the
/// 'esp_zigbee_lib' API.
///
#[must_use = "Please store the guard in a variable, e.g. '_guard = ...'"]
struct ZigbeeGuard;

// Just a safety check to see if anyone tries to guard repeatedly.
static GUARDED: AtomicBool = AtomicBool::new(false);

impl ZigbeeGuard {
    fn acquire() -> Self {
        let claimed = GUARDED.swap(true, Ordering::Relaxed);
        assert!(!claimed, "INTERNAL: Guard re-entered"); // if we coded correctly, a double claim won't happen. Application cannot cause it.

        unsafe {
            let got_it = esp_zigbee_lock_acquire(99999);    // tbd. change to a tick count matching, say, 9s

            // Note: 'got_it' being 'false' is technically possible: it means our huge window timed out.
            if !got_it { panic!("Timed out waiting for Zigbee lock") };
        };
        Self
    }
}

impl Drop for ZigbeeGuard {
    fn drop(&mut self) {
        let claimed= GUARDED.swap(false, Ordering::Relaxed);
        assert!(claimed, "INTERNAL: Guard already dropped!");

        unsafe {
            esp_zigbee_lock_release();
        }
    }
}
