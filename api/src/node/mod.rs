/*
* A Node.
*
* Provides access to the radio, and decides the role (Coordinator/Router/EndDevice) of the ... Node.
*
* Design:
*   - Singleton; there's only one Node in a running system
*   - Methods, not globals. Helps e.g. in IDE auto-completion (and generally not being able to see weird stuff
*       outside their context). In comparison, C API has everything as global functions, which is overwhelming.
*
* Note:
*   - Native mode only; no RCP (radio co-processor) implementation. C API has both.
*/
use core::mem::MaybeUninit;
use embassy_time::{Timer, Duration};

use bitflags::bitflags;
use log;

use crate::raw::{
    esp_zigbee_device_config_t,
    esp_zigbee_init,
    ezb_app_signal_t,
    ezb_bdb_start_top_level_commissioning,
    ezb_bdb_is_factory_new,
    ezb_nwk_get_panid,
    ezb_nwk_get_extended_panid,
    ezb_extpanid_t,
    ezb_nwk_get_short_address,
    ezb_nwk_get_current_channel,
    ezb_bdb_comm_mode_t,
    esp_zigbee_config_t,
    ezb_bdb_set_primary_channel_set,
    ezb_bdb_set_secondary_channel_set,
    esp_zigbee_launch_mainloop,
    esp_zigbee_start,
};

use esp_idf_sys::EspError;

use crate::{Error, IeeeAddr, AppSignal};

mod router;
pub use router::Router;

mod channel_mask;
pub use channel_mask::ChannelMask;
use esp_zb_raw::esp_zigbee_platform_config_t;

use alloc::boxed::Box;
use once_cell::sync::OnceCell;

use crate::config::{
    PlatformDeviceNodeView,
    NodeType
};

static SINGLETON_CHECK: OnceCell<()> = OnceCell::new();

/**
* Provides access to the radio, and decides the role (Coordinator/Router/EndDevice) of the ... Node.
*
* Configuration-wise this includes: network, platform and node categories.
*
* A **singleton** - you can create only one and it has "endless" (static) lifespan.
*/
pub struct Node {
    _cfg: &'static esp_zigbee_config_t,
        // anchored for the C side to use it; it's leaked so whether it's here or not does not really matter.
}

impl Node {
    /**
    * Initialize a 'Node' from a given configuration.
    */
    pub fn from_config(cv: &PlatformDeviceNodeView) -> Self {
        let (cc, channel_masks) = cv.expand();

        SINGLETON_CHECK.set(()).unwrap_or_else(|_| {
            panic!("node already in use.");
        });

        // Move the C-side configuration structure to heap (from stack), and leak it. Note: we wouldn't need to leak,
        // if we just place it as a member in 'Node', but this also works. We are singleton, after all.
        //
        // The point is to keep the contents from being moved around: 'esp_zigbee_init()' might read it, even after the
        // initial call.
        //
        let cc: &'static esp_zigbee_config_t = Box::leak(Box::new(cc));

        // esp_err_t esp_zigbee_init(const esp_zigbee_config_t *config);
        //
        let err = unsafe { esp_zigbee_init(cc) };
            //
            // 'esp_zigbee_init()' takes a pointer, so we must assume it can read that memory, later.
            // Providing it a 'static', non-changing struct is safe.

        EspError::from(err).unwrap_or_else(|e| {
            panic!("Initializing node failed: {}", e);
        });

        set_channel_sets(&channel_masks);

        Self { _cfg: cc }
    }

    /**
    * Process Zigbee messages.
    *
    * @param auto_start
    *   'true' for automatic start of the Zigbee stack
    *   'false' for delayed start, needing a call to '.start_top_level_commissioning()' at a later stage.
    */
    fn launch(self: Self, auto_start: bool) -> Result<!,EspError> {

        let err = unsafe {
            esp_zigbee_start(auto_start)
        };
        EspError::from(err).map_or(Ok(()), Err)?;

        let err = unsafe {
            esp_zigbee_launch_mainloop()
        };
        EspError::from(err).map_or(Ok(()), Err)?;

        unreachable!();

        #[cfg(false)]   // v 1.x; this will no longer work
        loop {
            unsafe { esp_zigbee_stack_main_loop_iteration() };

            // Note: Optimizing what shall be here is not trivial. We would ideally both:
            //  - process Zigbee events without delay (call e.g. 'yield_now().await' instead of waiting 1 tick)
            //  - sleep if there's nothing happening
            //
            //      Also, we must consider the Zigbee event loop. For NOW, it's safest to just
            //      always have a small nap. 'esp_zigbee_lib' should be fine. So should Embassy
            //      async code.
            //
            Timer::after(Duration::from_ticks(1)).await;
        }

        #[cfg(false)]
        loop {
            unsafe { esp_zb_stack_main_loop_iteration() };

            // 'yield_now()' consumes 100% CPU, but means there's no gap between Zigbee processing
            // its messages. We fall asleep only once Zigbee is idle.
            //
            if unsafe { esp_zb_scheduler_can_sleep() } {    // <-- no such function
                Timer::after(Duration::from_ticks(1)).await;
            } else {
                yield_now().await;
            }
        }
    }

    /**
    * Get the PAN ID of the network.
    */
    fn nwk_get_panid(&self) -> u16 {
        unsafe {
            ezb_nwk_get_panid()
        }
    }

    /**
    * Get the extended PAN ID of the network.
    */
    fn nwk_get_extended_panid(&self) -> IeeeAddr {
        let mut buf: ezb_extpanid_t = ezb_extpanid_t::empty();
        unsafe {
            ezb_nwk_get_extended_panid(&mut buf);
        }
        IeeeAddr::from(buf)
    }

    /**
    * Get the network (short) address of the device.
    */
    fn nwk_get_short_address(&self) -> u16 {
        unsafe {
            ezb_nwk_get_short_address()
        }
    }

    /**
    * Get the currently used channel.
    */
    fn nwk_get_current_channel(&self) -> u8 {
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
    fn start_top_level_commissioning(&self, mask: CommissioningModesMask) -> Option<EspError> {
        let err= unsafe {
            ezb_bdb_start_top_level_commissioning(mask.bits())
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
        unsafe {
            esp_zb_factory_reset()
        }
    }
}

/**
* Set the primary and secondary channel mask, on the C library side.
*
* @note This (for primary) "should be called [...] after 'ezb_core_init()' and before 'ezb_dev_start()'".
*       "If function is not called, by default it will scan all channels or read from zb_fct NVRAM zone if available." (1.x docs)
*       -- but we call it every time.
*/
fn set_channel_sets(channel_masks: &[ChannelMask;2]) {
    let mut primary = true;

    for cm in channel_masks {
        let err = unsafe {
            if primary {
                ezb_bdb_set_primary_channel_set(cm.into())
            } else {
                ezb_bdb_set_secondary_channel_set(cm.into())
            }
        };
        // EZB_ERR_NONE
        // EZB_ERR_INVALID_ARG  should not happen: 'ChannelMask'
        assert!((err == 0));

        primary = false;
    }
}

type BdbMode = ezb_bdb_comm_mode_t;
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CommissioningModesMask: u8 {
        const INITIALIZATION = BdbMode::EZB_BDB_MODE_INITIALIZATION.0 as u8; // 1
        #[cfg(feature = "touchlink")]
        const TOUCHLINK_INITIATOR = BdbMode::EZB_BDB_MODE_TOUCHLINK_INITIATOR.0 as u8; // 2
        const NETWORK_STEERING = BdbMode::EZB_BDB_MODE_NETWORK_STEERING.0 as u8; // 4
        #[cfg(feature = "controller")]
        const NETWORK_FORMATION = BdbMode::EZB_BDB_MODE_NETWORK_FORMATION.0 as u8; // 8
        const FINDING_N_BINDING = BdbMode::EZB_BDB_MODE_FINDING_N_BINDING.0 as u8; // 16
        #[cfg(feature = "touchlink")]
        const TOUCHLINK_TARGET = BdbMode::EZB_BDB_MODE_TOUCHLINK_TARGET.0 as u8; // 32

        // Declare all bits as "known". Recommended for 'bitflags', when working with C library APIs.
        const _ = !0;
    }
}

/*
* Handler for Zigbee APP signals.
*/
//  typedef struct esp_zb_app_signal_s {
//      uint32_t *p_app_signal;   /*!< Application pointer signal type, refer to esp_zb_app_signal_type_t */
//      esp_err_t esp_err_status; /*!< The error status of the each signal event, refer to esp_err_t */
//  } esp_zb_app_signal_t;
//
// NOTE: In addition to pointing to the signal type, 'p_app_signal' can be given to 'esp_zb_app_signal_get_params()',
//      in order to fetch more, signal specific, information. We bake those into a single value
//      Rust enum, below, before providing to the application.
//
#[unsafe(no_mangle)]
extern "C" fn esp_zb_app_signal_handler(ss: *mut ezb_app_signal_t) {
    let ss: *const ezb_app_signal_t = ss;  // un-mut

    let ezb_app_signal_t{ p_app_signal, esp_err_status: err_st } = unsafe { *ss };

    assert!(err_st == 0);  //?? mitä sillä pitäisi tehdä?

    AppSignal::from(p_app_signal as *const _)
        .map(|sig| {
            log::info!("Received: {}", sig);
        })
        .unwrap_or_else(|| {
            let sig_type = unsafe { *p_app_signal };

            // 'EspError' is 'Display': we can use it to give a wording for the error code.
            //  Note: Handling became a bit elaborate: 'Option<EspError>' is not 'Display'.
            //
            let ee = EspError::from(err_st);
            let display_ee: &dyn core::fmt::Display = match ee {
                Some(ref e) => e,
                None => &"ESP_OK",
            };

            log::error!("Unexpected app signal: {}, {}", sig_type, display_ee);
        });
}

/**
* Internal helper.
*
* Collects about-to-be-registered endpoints, and passes them to the C 'esp-zigbee-lib', all at once.
*/
#[cfg(false)]   // is it needed in 2.0?
struct RegState(*mut esp_zb_ep_list_t);

#[cfg(false)]
impl RegState {
    fn new() -> Self {
        let l = unsafe { esp_zb_ep_list_create() };
        Self(l)
    }

    fn add(&self, cluster_list: *mut esp_zb_cluster_list_t, ep_cfg: esp_zb_endpoint_config_t) {
        unsafe {
            esp_zb_ep_list_add_ep(self.0, cluster_list, ep_cfg);   // tbd. handle error
        }
    }

    fn register(self) {
        unsafe {
            esp_zb_device_register(self.0);    // tbd. errors
        }
    }
}
