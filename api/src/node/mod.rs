/*
* Node
*
* A Router, Controller or EndDevice. They are global in C 'esp_zigbee_sdk'. We wrap them in a more
* object-oriented manner.
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

// tbd. these as a config from the caller
const MAX_CHILDREN: usize = 10;             // max number of connected devices
const INSTALLCODE_POLICY: bool = false;     // install code policy for security
const HA_COLOR_DIMMABLE_LIGHT_ENDPOINT: u8 = 10;

pub trait Node {
    /*
    * Once a node is set up (in the C library), starting and running it are the same for all
    * node types.
    *
    * Autostart:
    *   true: "Loads parameters from NVRAM and immediately proceeds with [...] joining, forming or rejoining [...]"
    *   false: "half-start": initializes the Zigbee framework but does not initiate network operations.
    *
    *       Autostart 'false' is used e.g. if you have unfinished hardware initialization that should be carried
    *       out before "opening shop" on the Zigbee. ((Why would one build an app that way?)).
    *
    *       tbd. If we need to expose autostart, let's wrap it in some two modes (methods/trait parameter of the Node?).
    *
    *       nb. Autostart 'false' needs to call 'esp_zb_bdb_start_top_level_commissioning()', for manually starting
    *           that network stuff.
    */
    async fn ready_and_roll(&self) -> ! {
        unsafe {
            esp_zb_start(true);     // Note: C examples have 'false'?  Why?
        }

        loop {
            unsafe { esp_zb_stack_main_loop_iteration() };
            Timer::after(Duration::from_ticks(1)).await;    // tbd. try 0
        }
    }

}

// tbd. Router, End under the same trait, e.g. 'Node'.

struct Router{
    // no state, since the state is global!
}

impl Router {
    pub fn new() -> Self {
        let nwk_cfg = esp_zb_cfg_t {
            esp_zb_role: esp_zb_nwk_device_type_t::ESP_ZB_DEVICE_TYPE_ROUTER,
            install_code_policy: INSTALLCODE_POLICY,
            nwk_cfg: esp_zb_cfg_s__bindgen_ty_1 {
                zczr_cfg: esp_zb_zczr_cfg_t {
                    max_children: MAX_CHILDREN as _
                }
            }
        };

        todo!();
        #[cfg(false)]
        unsafe { esp_zb_init(nwk_cfg.into()) };

        Self{}
    }
}

impl Node for Router {}
