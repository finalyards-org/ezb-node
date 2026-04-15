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

use esp_zb_raw::{
    esp_zb_cfg_t,
    esp_zb_nwk_device_type_t,
    esp_zb_cfg_s__bindgen_ty_1,
    esp_zb_zczr_cfg_t,
    esp_zb_init,
    esp_zb_start,
    esp_zb_stack_main_loop_iteration
};

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
        unsafe {
            esp_zb_start(Self::AUTO_START);     // tbd. C examples have 'false'?  Why?
        }

        #[cfg(true)]
        loop {
            unsafe { esp_zb_stack_main_loop_iteration() };
            Timer::after(Duration::from_ticks(1)).await;
        }

        // An optimization, if we know the idle/busy state of Zigbee library. #later
        #[cfg(false)]
        loop {
            unsafe { esp_zb_stack_main_loop_iteration() };

            // 'yield_now()' consumes 100% CPU, but means there's no gap between Zigbee processing
            // its messages. We fall asleep only once Zigbee is idle.
            //
            if unsafe { esp_zb_scheduler_can_sleep() } {
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
