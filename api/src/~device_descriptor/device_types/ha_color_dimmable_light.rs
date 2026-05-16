#![cfg(feature = "ep_color_dimmable_light")]

use crate::raw::{
    ezb_zha_color_dimmable_light_config_t,
    ezb_af_ep_desc_t,
    ezb_zha_create_color_dimmable_light,
    ezb_zcl_cluster_id_e::{
        EZB_ZCL_CLUSTER_ID_BASIC,
    },
    ezb_zcl_cluster_desc_t,
    ezb_af_endpoint_get_cluster_desc,
    ezb_zcl_basic_server_attr_t::{
        EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID,
        EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID,
    },
    ezb_af_device_add_endpoint_desc,
    ezb_af_device_desc_register,
    ezb_zcl_core_action_handler_register,
    ezb_af_create_device_desc,
    ezb_af_device_desc_t,
};
use alloc::string::String;
use esp_zb_raw::ezb_zcl_basic_cluster_desc_add_attr;
use crate::node::{
    EndpointConfig::ColorDimmableLightConfig,
    Node,
};

trait ColorDimmableLight {
    /**
    * Create an endpoint for "color dimmable light" profile.
    */
    fn from_config(ep_id: u8, _cfg: ColorDimmableLightConfig) -> Endpoint {

        // C note: Based on 'esp_zigbee_create_zha_color_dimmable_light_device()' in C examples.

        //r let dev_desc: ezb_af_device_desc_t = unsafe {
        //r     ezb_af_create_device_desc()
        //r };

        type MyEndpoint = Endpoint<ezb_zha_color_dimmable_light_config_t>
        let cfg: ezb_zha_color_dimmable_light_config_t = ezb_zha_color_dimmable_light_config_t::default();
            // C: 'EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG'

        let ep: Endpoint = Endpoint>::new(ep_id, &cfg, )

        ezb_af_device_add_endpoint_desc(dev_desc, ep_desc);
        ezb_af_device_desc_register(dev_desc);

        ezb_zcl_core_action_handler_register(todo!());
    }
}
