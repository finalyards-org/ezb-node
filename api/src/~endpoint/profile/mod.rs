/*
*
*/
#![feature(adt_const_params)] // worry #later

use crate::raw::{
    esp_zb_cluster_list_t,
    esp_zb_endpoint_config_t,
    esp_zb_af_profile_id_t,
    esp_zb_ha_standard_devices_t,
};

#[cfg(feature = "ep_color_dimmable_light")]
mod color_dimmable_light;
pub use color_dimmable_light::*;

// Specific profiles must implement this.
//
trait Profile<const PROFILE_ID: esp_zb_af_profile_id_t, const DEVICE_ID: esp_zb_ha_standard_devices_t> {
    /**
    * Create the 'esp-zigbee-lib' structs for a certain Zigbee profile.
    */
    fn expand(self, id: u8) -> (*mut esp_zb_cluster_list_t, esp_zb_endpoint_config_t) {


    }
}
