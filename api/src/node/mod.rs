/*
* Design:
*   - Singleton; there's only one Node in a running system
*   - Methods, not globals. Helps e.g. in IDE auto-completion (and generally not being able to see weird stuff
*       outside their context). In comparison, C API has everything as global functions, which is overwhelming.
*   - Application task does not (need to) know about the underlying Zigbee task; it's an Embassy application
*     and can use 'async' for its own concurrency.
*
* Note:
*   - Native mode only; no RCP (radio co-processor) implementation. C API has both.
*/

mod zigbee_task;
use zigbee_task::zigbee_spawn;

use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel,
};
use esp_idf_svc::{
    sys::EspError
};
use log;

use ezb_node_raw::{
    ezb_bdb_start_top_level_commissioning,
    ezb_bdb_is_factory_new,
    ezb_nwk_get_panid,
    ezb_nwk_get_extended_panid,
    ezb_nwk_get_short_address,
    ezb_nwk_get_current_channel,
    esp_zigbee_lock_acquire,
    esp_zigbee_lock_release,
};

use crate::{
    AppSignal,
    Config,
    BdbMode,
    IeeeAddr,
    Error,
    ZclEvent
};

// Channel:
//  - fed by the Zigbee task; blocking
//  - consumed by the application task; async
//
pub(self) static CHANNEL: Channel<CriticalSectionRawMutex, Payload, 10> = Channel::new();
    // tx: in 'zigbee_task'
    // rx: us, passing to the application (application task)

#[derive(Debug, Clone)]
enum Payload {
    AppSignal(AppSignal),
    ZclEvent(ZclEvent)
}

/**
* Provides access to the radio. The application struct implementing this decides the role (Coordinator/Router/EndDevice)
* that this has.
*
* A **singleton** - you can create only one and it has "endless" (static) lifespan.
*
* @thread Call from application RTOS thread.
*/
pub trait Node {
    /**
    * Initialize the Zigbee side of things.
    *
    * This launches the task that receives Zigbee events, converts them to Rust-friendly structs, and sends them over
    * to the application task.
    */
    // 'auto_start': we might get rid of this parameter. It has to do with the application initialization logic.
    //      C example uses delayed hardware init. If the value is 'true', the Zigbee network needs to be later
    //      activated by a call to '...'.
    //
    fn init(cfg: &'static Config, auto_start: bool) -> Result<(), crate::Error> {
        let () = zigbee_spawn(cfg, auto_start)
            .map_err(|e| { Error::SpawnFailed(e) })?;
        Ok(())
    }

    /**
    * Run the event loop, passing Zigbee events to the application task (that calls us).
    *
    * The closures/functions get us (an application struct implementing 'Node' as a parameter, allowing them to
    * access the Zigbee APIs, via 'Node' methods. We ensure that locking is in place for such methods (see ZigbeeGuard).
    *
    * @note: The 'this' is mutable, in case the application has fields in there (state) that the handlers want to
    *       change. 'ezb_node' itself carries no state in 'T' (it couldn't, since the 'struct' is application defined).
    */
    async fn run<T: Node, F1, F2>(mut this: /*move*/ T, on_app_signal: F1, on_zcl_event: F2) -> !
    where
        F1: Fn(&mut T, AppSignal),
        F2: Fn(&mut T, Result<ZclEvent, ZclError>)
    {
        let rx = CHANNEL.receiver();

        loop {
            let x = rx.receive().await;
            match x {
                Payload::AppSignal(x) => on_app_signal(&mut this, x),
                Payload::ZclEvent(x) => on_zcl_event(&mut this, x)
            }
        }
    }

    //---
    // The rest of the methods are for the application task to call the Zigbee C API.
    // They _must_ all have the guarding mechanism in place!!!

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
        IeeeAddr::from(v)
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
    */
    fn start_top_level_commissioning(&self, mask: BdbMode) -> Option<EspError> {
        let _guard = ZigbeeGuard::acquire();
        let err= unsafe {
            ezb_bdb_start_top_level_commissioning(mask.into())
        };
        EspError::from(err) // provides 'Option'
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

#[must_use = "Please store the guard in a variable, e.g. '_guard = ...'; otherwise it drops right away."]
struct ZigbeeGuard;

impl ZigbeeGuard {
    /**
    * It's mandatory to acquire the lock before calling any Zigbee SDK APIs, except that the call site is in Zigbee
    * callbacks.
    */
    fn acquire() -> Self {
        unsafe {
            let got_it = esp_zigbee_lock_acquire(u32::MAX);
            assert!(got_it);
        };
        Self
    }
}

impl Drop for ZigbeeGuard {
    fn drop(&mut self) {
        unsafe {
            esp_zigbee_lock_release();
        }
    }
}
