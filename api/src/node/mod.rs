/*
* A Router, Controller and/or EndDevice.
*
* Node type specific methods and constants are defined each in their own source file.
* General (all nodes) are under 'Node' trait.
*
* Design:
*   The C API implements all these as _global_ functions and constants. We are more specific,
*   which should help e.g. in IDE auto-completion (and generally not using weird stuff when it
*   does not make sense!).
*
* Note:
*   C code supports both native and RCP (radio co-processor) implementations. We only native.
*/
use embassy_time::{Timer, Duration};

use bitflags::bitflags;
use log;

use esp_zb_raw::{esp_zb_cfg_t, esp_zb_init, esp_zb_start, esp_zb_stack_main_loop_iteration, esp_zb_app_signal_t, esp_zb_get_pan_id, esp_zb_get_current_channel, esp_zb_bdb_start_top_level_commissioning, esp_zb_bdb_commissioning_mode_t, esp_zb_get_extended_pan_id, esp_zb_bdb_is_factory_new, esp_zb_get_short_address};

use esp_idf_sys::EspError;

use crate::{Error, IeeeAddr, Signal};

mod router;
pub use router::Router;

#[allow(non_upper_case_globals)]
static mut G_nwk_cfg: Option<esp_zb_cfg_t> = None;
    //
    // note: could use 'OnceLock' but it's 'std'. Also this works.

pub trait Node {
    /**
    * Take ownership of the Zigbee C stack. May only be called once.
    */
    fn take_stack(nwk_cfg: esp_zb_cfg_t) -> Result<(),crate::Error> {

        // If 'G_nwk_cfg' already used, fail.
        // Note: This does not need to be atomic, since all access happens within the same
        //      FreeRTOS task (anything between '.await's is atomic).
        //
        // NOTE: IF THERE ARE PROBLEMS, have a look at how to use "raw borrow" instead of the
        //      intermediate '&mut' we now have. |1|
        //
        //      |1|: "Raw pointers" (The Rust Edition Guide)
        //          https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html#raw-pointers
        //
        #[allow(static_mut_refs)]
        let ptr = unsafe {
            if G_nwk_cfg.is_some() {
                return Err(Error::AlreadyInUse);
            }
            G_nwk_cfg.insert(nwk_cfg)  // eats 'nwk_cfg'
        };
        nwk_cfg.install_code_policy;    // TEMP; should NOT PASS the compiler

        // void esp_zb_init(esp_zb_cfg_t *nwk_cfg);
        //
        unsafe { esp_zb_init(ptr) };
            //
            // 'esp_zb_init()' takes a pointer, so we must assume it can read that memory, later.
            // Providing it a 'static', non-changing struct is safe.
            //
            // NOTE: It actually takes a _non-const_ pointer. Thus, we need to pass it a '*mut'.
            //
            // tbd. If we know how to avoid 'esp_zb_cfg_t' from being 'Copy' (moveable across
            //      memory) in the 'bindgen' state, that'd be sweet..

        Ok(())
    }

    /*
    * Once a node is set up in the C code, starting and running it are the same for all node types.
    *
    * Autostart:
    *   true: "Loads parameters from NVRAM and immediately proceeds with [...] joining, forming or rejoining [...]"
    *   false: "half-start": initializes the Zigbee framework but does not initiate network operations.
    *
    *       Autostart 'false' is used e.g. if you have unfinished hardware initialization that should be carried
    *       out before "opening shop" on the Zigbee. (Why would one build an app that way?)
    *
    *       note. Autostart 'false' needs to call 'esp_zb_bdb_start_top_level_commissioning()', for manually starting
    *           the network stuff.
    */
    //const AUTO_START: bool = true;

    /**
    * Process Zigbee messages.
    *
    * @param auto_start
    *   'true' for automatic start of the Zigbee stack
    *   'false' for delayed start, needing a call to '.start_top_level_commissioning()' at a later stage.
    */
    #[allow(async_fn_in_trait)] // "you can suppress this lint if you plan to use the trait only in your own code"
    async fn roll(self: Self, auto_start: bool) -> ! where Self: Sized {

        unsafe {
            esp_zb_start(auto_start);
        }

        loop {
            unsafe { esp_zb_stack_main_loop_iteration() };

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
    * Get the Zigbee network PAN ID.
    */
    fn get_pan_id(&self) -> u16 {
        unsafe {
            esp_zb_get_pan_id()
        }
    }

    /**
    * Get extended PAN ID.
    */
    fn get_extended_pan_id(&self) -> IeeeAddr {
        let mut buf: [u8;8] = [0;_];
        unsafe {
            esp_zb_get_extended_pan_id(buf.as_mut_ptr());
        }
        IeeeAddr::from(buf)
    }

    /**
    * Get the short address.
    */
    fn get_short_address(&self) -> u16 {
        unsafe {
            esp_zb_get_short_address()
        }
    }

    /**
    * Get the currently used channel.
    */
    fn get_current_channel(&self) -> u8 {
        unsafe {
            esp_zb_get_current_channel()
        }
    }

    /**
    * Start commissioning.
    *
    * This function is intended to be a lower level, common tool. Applications should likely use
    * node type -specific helper methods (and not directly the modes).
    *
    * Note: Some of the modes apply only to certain node types (NETWORK_FORMATION only to Coordinator
    *       role).
    *
    * Note: Modes are bit patterns.
    */
    fn start_top_level_commissioning(&self, mask: CommissioningModesMask) -> Option<EspError> {
        let err= unsafe {
            esp_zb_bdb_start_top_level_commissioning(mask.bits())
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
            esp_zb_bdb_is_factory_new()
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

type BdbMode = esp_zb_bdb_commissioning_mode_t;
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CommissioningModesMask: u8 {
        // 0 should ideally not be used as 'bitflags' (see bitflags docs); use '::empty()' instead.
        //const INITIALIZATION = ESP_ZB_BDB_MODE_INITIALIZATION; // 0

        #[cfg(feature = "touchlink")]
        const TOUCHLINK = BdbMode::ESP_ZB_BDB_MODE_TOUCHLINK.0 as u8; // 1
        const NETWORK_STEERING = BdbMode::ESP_ZB_BDB_MODE_NETWORK_STEERING.0 as u8; // 2
        #[cfg(feature = "controller")]
        const NETWORK_FORMATION = BdbMode::ESP_ZB_BDB_MODE_NETWORK_FORMATION.0 as u8; // 4
        #[cfg(feature = "touchlink")]
        const TOUCHLINK_TARGET = BdbMode::ESP_ZB_BDB_MODE_TOUCHLINK_TARGET.0 as u8; // 64

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
extern "C" fn esp_zb_app_signal_handler(ss: *mut esp_zb_app_signal_t) {
    let ss: *const esp_zb_app_signal_t = ss;  // un-mut

    let esp_zb_app_signal_t{ p_app_signal, esp_err_status: err_st } = unsafe { *ss };

    Signal::from(p_app_signal as *const _, err_st)
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

