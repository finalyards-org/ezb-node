
use core::ffi::c_void;
use std::collections::BTreeMap;

use esp_idf_svc::sys::EspError;

use ezb_node_config::{
    CommonFields,
    Config,
    Endpoint,
    DeviceType,
    NodeType::*,
};

use ezb_node_raw::{
    ezb_af_device_add_endpoint_desc,
    ezb_af_device_desc_t,
    ezb_af_endpoint_get_cluster_desc,
    ezb_nwk_device_type_t::*,
    ezb_zcl_basic_cluster_desc_add_attr,
    ezb_zcl_cluster_desc_t,
    ezb_zcl_basic_server_attr_t,
    ezb_zcl_cluster_id_e,
    ezb_af_profile_id_e,
    ezb_zha_device_id_e,
    ezb_af_ep_desc_t,
    ezb_zha_color_dimmer_switch_config_t,
    ezb_zha_create_color_dimmer_switch,
    ezb_zcl_role_e,
    ezb_af_ep_config_t,
};

use crate::{
    utils::PascalString,
};

/**
* A view to a 'Config' struct used in creating end points.
*
* Note: It is perfectly okay for a node not to have an endpoint (i.e. just be a network repeater).
*
* Covers TOML sections '[endpoint.default]', '[endpoint.{id}]'.
*/
pub(crate) struct EndpointCreator(&'static BTreeMap<u8, Endpoint>);

impl EndpointCreator {
    pub(crate) fn create_all(self, dev: ezb_af_device_desc_t) -> Result<(), EspError> {
        for (&ep_id, entry) in self.0 {
            create_one(dev, ep_id, entry.clone())?;
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

/// Create an end point, in the C library level.
//
fn create_one(dev: ezb_af_device_desc_t, ep_id: u8, ab: Endpoint) -> Result<(), EspError> {
    use DeviceType::*;

    let Endpoint {
        common_fields,
        device_type,
        additional_server_clusters
    } = ab;

    // Note: It's perfectly possible to have a node without an endpoint (= a repeater). That means
    //      this code would not be called - but it still is being built. Using '.clone()' tricks
    //      the Rust compiler - it would not like matching a reference to an empty enum (which
    //      cannot be instantiated).
    //
    //      An alternative is to opt-out the whole function, but that needs a '_dt_any' feature that
    //      gets automatically set. Didn't bother with that.
    //
    let ep_desc = match device_type .clone() {
        #[cfg(feature = "dt_color_dimmable_light")]
        DeviceType::ColorDimmableLight => create_color_dimmable_light(ep_id),

        #[cfg(feature = "dt_color_dimmer_switch")]
        DeviceType::ColorDimmerSwitch => create_color_dimmer_switch(ep_id),

        #[cfg(feature = "dt_ias_cie")]
        DeviceType::IasCie => create_ias_cie(ep_id),
    };

    // Add common fields.
    //
    // ezb_zcl_cluster_desc_t basic_desc = {0};
    // basic_desc = ezb_af_endpoint_get_cluster_desc(ep_desc, EZB_ZCL_CLUSTER_ID_BASIC, EZB_ZCL_CLUSTER_SERVER);
    // ezb_zcl_basic_cluster_desc_add_attr(basic_desc, EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID, (void *)ESP_MANUFACTURER_NAME);
    // ezb_zcl_basic_cluster_desc_add_attr(basic_desc, EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID, (void *)ESP_MODEL_IDENTIFIER);
    {
        use ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_BASIC; // 0
        use ezb_zcl_role_e::CLUSTER_SERVER;
        use ezb_zcl_basic_server_attr_t::{
            EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID,
            EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID,
        };

        let CommonFields {
            manufacturer_name,
            model_identifier,
        } = common_fields;

        let basic_desc: ezb_zcl_cluster_desc_t = unsafe {
            ezb_af_endpoint_get_cluster_desc(ep_desc, EZB_ZCL_CLUSTER_ID_BASIC, CLUSTER_SERVER)
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
    }

    // Additional clusters.
    {
        for cl in additional_server_clusters {
            todo!();
        }
    }

    // ESP_ERROR_CHECK(ezb_af_device_add_endpoint_desc(dev_desc, ep_desc));
    //
    let err = unsafe { ezb_af_device_add_endpoint_desc(dev, ep_desc) };

    match EspError::from(err) {
        None => Ok(()),
        Some(e) => Err(e),
    }
}

#[cfg(feature = "dt_color_dimmable_light")]
fn create_color_dimmable_light(ep_id: u8) -> ezb_af_ep_desc_t {
    use ezb_node_raw::{
        ezb_zha_color_dimmable_light_config_t,
        ezb_zha_create_color_dimmable_light
    };

    let mut cfg = ezb_zha_color_dimmable_light_config_t::default();
    unsafe { ezb_zha_create_color_dimmable_light(ep_id, &cfg) }
}

#[cfg(feature = "dt_color_dimmer_switch")]
fn create_color_dimmer_switch(ep_id: u8) -> ezb_af_ep_desc_t {
    use ezb_node_raw::{
        ezb_zha_color_dimmer_switch_config_t,
        ezb_zha_create_color_dimmer_switch
    };

    let mut cfg = ezb_zha_color_dimmer_switch_config_t::default();
    unsafe { ezb_zha_create_color_dimmer_switch(ep_id, &cfg) }
}

// This variant is what 'esp-zigbee-sdk' PEOPLE suggest |1| - building from scratch.
//  |1[ -> https://github.com/espressif/esp-zigbee-sdk/issues/897
#[cfg(feature = "dt_ias_cie")]
fn create_ias_cie(ep_id: u8) -> ezb_af_ep_desc_t {
    use ezb_af_profile_id_e::EZB_AF_HA_PROFILE_ID;

    let cfg = ezb_af_ep_config_t::new( ep_id,
        EZB_AF_HA_PROFILE_ID,
        EZB_ZHA_IAS_CONTROL_INDICATING_EQUIPMENT_ID,
        0   // app device version
    );

    let ep_desc: ezb_af_ep_desc_t = ezb_af_create_endpoint_desc(&cfg);

    //ezb_af_endpoint_add_cluster_desc(ep_desc, ezb_zcl_ias_ace_create_cluster_desc(ias_ace_cfg, EZB_ZCL_CLUSTER_SERVER));
    //ezb_af_endpoint_add_cluster_desc(ep_desc, ezb_zcl_identify_create_cluster_desc(NULL, EZB_ZCL_CLUSTER_CLIENT));
    //ezb_af_endpoint_add_cluster_desc(ep_desc, ezb_zcl_ias_zone_create_cluster_desc(NULL, EZB_ZCL_CLUSTER_CLIENT));
    //ezb_af_endpoint_add_cluster_desc(ep_desc, ezb_zcl_ias_wd_create_cluster_desc(NULL, EZB_ZCL_CLUSTER_CLIENT));

    return ep_desc;
}

// This variant is what GOOGLE.AI suggested - building on top of a harmless device type.
#[cfg(false)]
#[cfg(feature = "dt_ias_cie")]
fn create_ias_cie(ep_id: u8) -> ezb_af_ep_desc_t {
    use ezb_node_raw::{
        ezb_zha_configuration_tool_config_t,
        ezb_zha_create_configuration_tool,
        ezb_zcl_ias_zone_create_cluster_desc,
        ezb_zcl_role_e::CLUSTER_CLIENT,
        ezb_zcl_ias_zone_cluster_client_init,
        ezb_af_endpoint_add_cluster_desc,
    };

    // Once (if!) 'esp-zigbee-sdk' starts supporting 'ias-cie', we can revert to similar setup as
    // with the other device types.
    #[cfg(false)]
    {
        let mut cfg = ezb_zha_color_ias_cie_config_t::default();
        unsafe { ezb_zha_create_ias_cie(ep_id, &cfg) }
    }

    // Until then..
    {
        // Create *some* endpoint that we can piggy-pack on.
        //
        let cfg = ezb_zha_configuration_tool_config_t::default();
        let ep_desc = unsafe { ezb_zha_create_configuration_tool(ep_id, &cfg) };

        let cl_desc = unsafe {
            ezb_zcl_ias_zone_create_cluster_desc(
                std::ptr::null(),       // NULL uses defaults for a client
                CLUSTER_CLIENT as u8
            )
        };

        // Attach the cluster to the end point description.
        let err = unsafe {
            ezb_af_endpoint_add_cluster_desc(ep_desc, cl_desc)
        };
        // IF we start getting these, consider what should be done. This only occurs during node
        // initialization, so panic is good-enough-for-now.
        //
        if err != 0 /*EZB_ERR_NONE*/ {
            panic!("ezb_af_endpoint_add_cluster_desc failed: {}", err);
        }

        // tbd. Q: should this be here? Ask around.
        unsafe {
            ezb_zcl_ias_zone_cluster_client_init(ep_id);
        }
        ep_desc
    }
}
