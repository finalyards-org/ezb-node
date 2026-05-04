/*
* A Router device.
*/
#[allow(unused_imports)]
use log;

use crate::raw::{
    ezb_cfg_t,
    ezb_nwk_device_type_t,
    ezb_cfg_s__bindgen_ty_1,
    ezb_zczr_cfg_t,
};
use crate::AppSignal;
use crate::node::{Node, NodeConfig};

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

    fn init(cfg: NodeConfig) -> Result<(), crate::Error> {

        let max_children = cfg.max_children.unwrap_or(MAX_CHILDREN_DEFAULT);

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
        };  // tbd. move crafting the 'tmp' (above) inside '::take_stack'; can have router-conditional code (then it handles all raw level)
        <Self as Node>::take_stack(tmp, cfg)?;

        Ok(())
    }

    fn on_app_signal(&self, sig: AppSignal) /*-> Result<(), Self::Error>*/;
}
