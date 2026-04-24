/*
* A Router device.
*/
#[allow(unused_imports)]
use log;

use esp_zb_raw::{
    esp_zb_cfg_t,
    esp_zb_nwk_device_type_t,
    esp_zb_cfg_s__bindgen_ty_1,
    esp_zb_zczr_cfg_t,
};
use crate::Signal;
use crate::node::Node;

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

    fn init(max_children: u8) -> Result<(), crate::Error> {

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
        <Self as Node>::take_stack(tmp)
    }

    fn on_app_signal(&self, sig: Signal) /*-> Result<(), Self::Error>*/;
}
