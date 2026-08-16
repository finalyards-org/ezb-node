
use core::ffi::c_void;
use std::collections::BTreeMap;

use esp_idf_svc::sys::EspError;
use ezb_node_config::{
    CommonFields,
    Config,
    EndpointConfig,
    Specific,
    NodeType::*,
};

use ezb_node_raw::{
    ezb_af_device_add_endpoint_desc,
    ezb_af_device_desc_t,
    ezb_af_endpoint_get_cluster_desc,
    ezb_nwk_device_type_t::*,
    ezb_zcl_basic_cluster_desc_add_attr,
    ezb_zcl_cluster_desc_t,
    ezb_zha_create_color_dimmable_light,
    ezb_zcl_basic_server_attr_t,
    ezb_zcl_cluster_id_e,
    ezb_zha_color_dimmable_light_config_t,
    ezb_zha_color_dimmer_switch_config_t,
    EZB_ZCL_CLUSTER_SERVER,
    ezb_zha_create_color_dimmer_switch
};

use crate::{
    utils::PascalString,
};

/**
* A view to a 'Config' struct used in creating end points.
*
* Note: It is perfectly okay for a node not to have any endpoints.
*
* Covers TOML sections '[endpoint.default]', '[endpoint.{id}]'.
*/
pub(crate) struct EndpointCreator(&'static BTreeMap<u8, EndpointConfig>);

impl EndpointCreator {
    pub(crate) fn create_all(self, dev: ezb_af_device_desc_t) -> Result<(), EspError> {
        for (&ep_id, entry) in self.0 {
            create_one(dev, ep_id, entry)?;
        }
        Ok(())
    }
}

// Allow an app to provide a static '&Config' where the needing party only needs a view.
impl From<&'static Config> for EndpointCreator {
    fn from(c: &'static Config) -> Self {
        Self(&c.endpoints)
    }
}

// Create each end point. Their types are different, but much of the logic is common.
//
fn create_one(dev: ezb_af_device_desc_t, ep_id: u8, ab: &EndpointConfig) -> Result<(), EspError> {
    use Specific::*;

    let EndpointConfig(common_fields, specific) = ab;

    // Without '.clone()', this code would break a build when there are no endpoints enabled.
    //
    // The reason: by Rust rules, a reference to an enum "is always inhabited" (enum cannot be empty).
    //      However, if we have no end points enabled, that's exactly what 'Specific' is - an empty
    //      enum that can not be instantiated. The '.clone()' allows the code to build, though this
    //      particular function will not get called.
    //
    // Note: To cfg-out the whole function is a possibility, but did not want to take that route.
    //      It'd need tracking all endpoints; here it kind of happens naturally.

    let ep_desc = match specific.clone() {
        #[cfg(feature = "ep_color_dimmable_light")]
        // ezb_zha_color_dimmable_light_config_t light_cfg = EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG();
        // ezb_af_ep_desc_t       ep_desc = ezb_zha_create_color_dimmable_light(ESP_ZIGBEE_HA_COLOR_DIMMABLE_LIGHT_EP_ID, &light_cfg);
        //
        Specific::ColorDimmableLight => {
            let mut cfg = ezb_zha_color_dimmable_light_config_t::default();
            unsafe { ezb_zha_create_color_dimmable_light(ep_id, &cfg) }
        }

        #[cfg(feature = "ep_color_dimmer_switch")]
        // ezb_zha_color_dimmer_switch_config_t switch_cfg = EZB_ZHA_COLOR_DIMMER_SWITCH_CONFIG();
        // ezb_af_ep_desc_t       ep_desc = ezb_zha_create_color_dimmer_switch(ESP_ZIGBEE_HA_COLOR_DIMMER_SWITCH_EP_ID, &switch_cfg);
        //
        Specific::ColorDimmerSwitch => {
            let mut cfg = ezb_zha_color_dimmer_switch_config_t::default();
            unsafe { ezb_zha_create_color_dimmer_switch(ep_id, &cfg) }
        },
    };

    // ezb_zcl_cluster_desc_t basic_desc = {0};
    // basic_desc = ezb_af_endpoint_get_cluster_desc(ep_desc, EZB_ZCL_CLUSTER_ID_BASIC, EZB_ZCL_CLUSTER_SERVER);
    // ezb_zcl_basic_cluster_desc_add_attr(basic_desc, EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID, (void *)ESP_MANUFACTURER_NAME);
    // ezb_zcl_basic_cluster_desc_add_attr(basic_desc, EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID, (void *)ESP_MODEL_IDENTIFIER);
    // ESP_ERROR_CHECK(ezb_af_device_add_endpoint_desc(dev_desc, ep_desc));
    {
        use ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_BASIC; // 0
        use ezb_zcl_basic_server_attr_t::{
            EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID,
            EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID,
        };

        let CommonFields {
            manufacturer_name,
            model_identifier,
        } = *common_fields;

        let basic_desc: ezb_zcl_cluster_desc_t = unsafe {
            ezb_af_endpoint_get_cluster_desc(ep_desc, EZB_ZCL_CLUSTER_ID_BASIC as u16, EZB_ZCL_CLUSTER_SERVER as u8)
            // basic cluster: id 0
        };

        [
            (manufacturer_name, EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID),
            (model_identifier, EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID),
        ]
        .into_iter()
        .filter_map(|(opt,id)| {
            let s = opt?; // break on 'None' of .0
            Some((s, id))
        })    // ones actually having a string
        .for_each(|(s, which)| {
            let ps: &'static [u8] = PascalString::from(s).into_leaked();
            unsafe { ezb_zcl_basic_cluster_desc_add_attr(basic_desc, which, ps.as_ptr() as *const c_void); }
        });

        let err = unsafe { ezb_af_device_add_endpoint_desc(dev, ep_desc) };
        EspError::from(err).map_or(Ok(()), Err)?;
    }
    Ok(())
}

