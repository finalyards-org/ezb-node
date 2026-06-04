use std::collections::BTreeMap;
use esp_idf_svc::sys::EspError;
use ezb_node_config::{
    CommonFields,
    Config,
    EndpointConfig,
    Specific,
    NodeType::*
};

use ezb_node_raw::{
    ezb_af_device_add_endpoint_desc,
    ezb_af_device_desc_t,
    ezb_af_endpoint_get_cluster_desc,
    ezb_nwk_device_type_t::*,
    ezb_zcl_basic_cluster_desc_add_attr,
    ezb_zcl_cluster_desc_t,
    ezb_zha_create_color_dimmable_light,
    EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG,
    ezb_zcl_basic_server_attr_t,
    ezb_zcl_cluster_id_e,
};
use crate::utils::PascalString;

/**
* A view to a 'Config' struct used in creating end points.
*
* Covers TOML sections '[endpoint.default]', '[endpoint.{id}]'.
*/
pub struct EndpointCreator(&'static BTreeMap<u8, EndpointConfig>);

impl EndpointCreator {

    pub(crate) fn create_all(&self, dev: ezb_af_device_desc_t) {
        for (ep_id, entry) in self.0 {
            self.create_one(dev, ep_id, entry);
        }
    }

    // Create each end point. Their types are different, but much of the logic is common.
    //
    fn create_one(dev: ezb_af_device_desc_t, ep_id: u8, ab: &EndpointConfig) -> Result<(), EspError> {
        use Specific::*;

        let EndpointConfig(common_fields, specific) = ab;

        let ep_desc = match specific {
            #[cfg(feature = "ep_color_dimmable_light")]
            // ezb_zha_color_dimmable_light_config_t light_cfg = EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG();
            // ezb_af_ep_desc_t       ep_desc = ezb_zha_create_color_dimmable_light(ESP_ZIGBEE_HA_COLOR_DIMMABLE_LIGHT_EP_ID, &light_cfg);
            //
            Specific::ColorDimmableLightEPC => {
                let light_cfg = unsafe { EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG };
                unsafe { ezb_zha_create_color_dimmable_light(ep_id, &light_cfg) }
            }
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
            } = common_fields;

            let manufacturer_name = PascalString::from(manufacturer_name).into_leaked();
            let model_identifier = PascalString::from(model_identifier).into_leaked();

            let basic_desc: ezb_zcl_cluster_desc_t = unsafe {
                ezb_af_endpoint_get_cluster_desc(ep_desc, EZB_ZCL_CLUSTER_ID_BASIC, ClusterRole::Server)
            };

            unsafe { ezb_zcl_basic_cluster_desc_add_attr(basic_desc, EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID, manufacturer_name); }
            unsafe { ezb_zcl_basic_cluster_desc_add_attr(basic_desc, EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID, model_identifier); }

            let err = unsafe { ezb_af_device_add_endpoint_desc(dev, ep_desc) };
            EspError::from(err).map_or(Ok(()), Err)?;
        }
        Ok(())
    }
}

// This allows an app to provide a static '&Config' where the needing party only needs a view.
impl From<&'static Config> for EndpointCreator {
    fn from(c: &'static Config) -> Self {
        Self(&c.endpoints)
    }
}

