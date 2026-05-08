/*
* A Router device.
*/
#[allow(unused_imports)]
use log;

use crate::raw::{
    esp_zigbee_device_config_t,
    ezb_nwk_device_type_t,
    esp_zigbee_zczr_config_s,
    esp_zigbee_device_config_s__bindgen_ty_1,
};
use crate::AppSignal;
use crate::node::{Node, TOMLConfig};

#[cfg(any(feature = "controller", feature = "router"))]
const MAX_CHILDREN_DEFAULT: u8 = 10;
#[cfg(not(any(feature = "controller", feature = "router")))]
const MAX_CHILDREN_DEFAULT: u8 = 0; // end device

/**
* Zigbee router.
*
* Example:
*   ```
*   struct MyRouter [...];
*   impl Router for MyRouter {
*       type Error = ...;
*       fn onAppSignal(&self, sig: Signal) {
*           ...
*           }
*   }
*   ```
*/
pub trait Router where Self: Node {
    type Error;

    fn init(cfg: TOMLConfig) -> Result<(), crate::Error> {

        let max_children = cfg.max_children.unwrap_or(MAX_CHILDREN_DEFAULT);

        // tbd. For INITIAL DEMOS, have this as 'false' (as was in C example)
        //      - move to 'true' (even for demos); heading for the secure pairing time
        //
        const INSTALLCODE_NOT_YET: bool = false;

        // 2.0
        //typedef struct esp_zigbee_device_config_s {
        //     ezb_nwk_device_type_t device_type;          /*!< The nwk device type, @ref ezb_nwk_device_type_t */
        //     bool install_code_policy;                   /*!< Allow install code security policy or not */
        //     union {
        //         struct esp_zigbee_zczr_config_s zczr_config; /*!< The Zigbee zc/zr device configuration */
        //         struct esp_zigbee_zed_config_s  zed_config;  /*!< The Zigbee zed device configuration */
        //     };
        // } esp_zigbee_device_config_t;
        //
        //struct esp_zigbee_zczr_config_s {
        //     uint8_t max_children; /*!< Max number of the children */
        // };
        //
        let tmp = esp_zigbee_device_config_t {
            device_type: ezb_nwk_device_type_t::EZB_NWK_DEVICE_TYPE_ROUTER,
            install_code_policy: INSTALLCODE_NOT_YET,
            __bindgen_anon_1: esp_zigbee_device_config_s__bindgen_ty_1 {
                zczr_cfg: esp_zb_zczr_cfg_t {
                    max_children
                }
            }
        };  // tbd. move crafting the 'tmp' (above) inside '::take_stack'; can have router-conditional code (then it handles all raw level)
        <Self as Node>::take_stack(tmp, cfg)?;

        Ok(())
    }

    fn on_app_signal(&self, sig: AppSignal) /*-> Result<(), Self::Error>*/;
}
