/*
* A Router, Controller and/or EndDevice.
*
* Design:
*   The concept is implemented as a global in C 'esp_zigbee_sdk'. We wrap them in a more object-
*   oriented way.
*
* Note:
*   C code supports both native and RCP (radio co-processor) implementations. We only native.
*/
use embassy_time::{Timer, Duration};

use log;

use esp_zb_raw::{
    esp_zb_cfg_t,
    esp_zb_nwk_device_type_t,
    esp_zb_cfg_s__bindgen_ty_1,
    esp_zb_zczr_cfg_t,
    esp_zb_init,
    esp_zb_start,
    esp_zb_stack_main_loop_iteration,
    esp_zb_app_signal_t,
    //esp_zb_app_signal_type_t,
};

use esp_idf_sys::EspError;

use crate::Signal;

#[allow(non_upper_case_globals)]
static mut G_nwk_cfg: Option<esp_zb_cfg_t> = None;
    //
    // note: could use 'OnceLock' but it's 'std'. Also this works.

pub trait Node {
    /**
    * Take ownership of the Zigbee C stack. May only be called once.
    */
    fn take_stack(nwk_cfg: esp_zb_cfg_t) -> Result<(),&'static str> {

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
                return Err("Zigbee already in use");
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
    const AUTO_START: bool = true;

    /**
    * Process Zigbee messages.
    */
    // Ah, the 'async' within trait.
    //
    async fn roll(self) -> ! where Self: Sized {
        log::debug!("1");
        unsafe {
            esp_zb_start(Self::AUTO_START);     // tbd. C examples have 'false'?  Why?
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
}

/**
* Zigbee router.
*/
pub struct Router{
    _private: ()    // prevent creation from outside (even if we don't store the state)
}

impl Router {
    pub fn new(max_children: u8) -> Result<Self, &'static str> {

        // tbd. For INITIAL DEMOS, have this as 'false' (as was in C example)
        //      - move to 'true' (even for demos); heading for the secure pairing time
        //
        const INSTALLCODE_NOT_YET: bool = false;

        //typedef struct esp_zb_cfg_s {
        //    esp_zb_nwk_device_type_t esp_zb_role; /*!< The nwk device type */
        //    bool install_code_policy;             /*!< Allow install code security policy or not */
        //    union {
        //        esp_zb_zczr_cfg_t zczr_cfg; /*!< The Zigbee zc/zr device configuration */
        //        esp_zb_zed_cfg_t zed_cfg;   /*!< The Zigbee zed device configuration */
        //    } nwk_cfg;                      /*!< Union of the network configuration */
        //} esp_zb_cfg_t;
        //
        //typedef struct {
        //    uint8_t max_children; /*!< Max number of the children */
        //} esp_zb_zczr_cfg_t;
        //
        let tmp = esp_zb_cfg_t {
            esp_zb_role: esp_zb_nwk_device_type_t::ESP_ZB_DEVICE_TYPE_ROUTER,
            install_code_policy: INSTALLCODE_NOT_YET,
            nwk_cfg: esp_zb_cfg_s__bindgen_ty_1 {
                zczr_cfg: esp_zb_zczr_cfg_t {
                    max_children
                }
            }
        };
        <Self as Node>::take_stack(tmp)?;

        Ok( Self{ _private: () } )
    }

    pub async fn roll(self) -> ! {
        <Self as Node>::roll(self) .await
    }
}

impl Node for Router {
    // enables '.roll()'
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
