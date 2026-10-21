
use core::ffi::c_void;
use std::collections::BTreeMap;

use ezb_node_config::{
    CommonFields,
    Config,
    Endpoint,
    DeviceType,
    NodeType::*,
    Cluster,
};

use ezb_node_raw::{
    ezb_af_device_add_endpoint_desc,
    ezb_af_device_desc_t,
    ezb_af_endpoint_get_cluster_desc,
    ezb_nwk_device_type_t::*,
    ezb_zcl_basic_cluster_desc_add_attr,
    ezb_zcl_cluster_desc_t,
    ezb_zcl_basic_server_attr_t::{
        EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID,
        EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID,
    },
    ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_BASIC,
    ezb_af_profile_id_e,
    ezb_zha_device_id_e,
    ezb_af_ep_desc_t,
    ezb_zha_color_dimmer_switch_config_t,
    ezb_zha_create_color_dimmer_switch,
    ezb_zcl_role_e::CLUSTER_SERVER,
    ezb_af_ep_config_t,
    ezb_af_create_endpoint_desc,
    ezb_zcl_power_config_create_cluster_desc,
    ezb_af_endpoint_add_cluster_desc,
    ezb_zcl_power_config_cluster_client_init,
    ezb_zcl_role_e::CLUSTER_CLIENT,
    ezb_af_create_device_desc,
    ezb_af_device_desc_register,
    ezb_zcl_core_action_handler_register
};

use crate::{
    utils::PascalString,
    EspError,
    EzbEspError,
    EzbError,
};

/**
* A view to a 'Config' struct used in creating -and initializing- end points.
*
* Note:
*   For a router node, it is perfectly okay not to have an endpoint (i.e. just be a repeater).
*   For coordinator and end device, without an end point there would be nothing.
*
* Covers TOML sections '[endpoint.default]', '[endpoint.{id}]'.
*/
pub(crate) struct EndpointCreator(&'static BTreeMap<u8, Endpoint>);

impl EndpointCreator {
    /// Create the configured endpoints.
    ///
    pub(crate) fn create_all_c(&self, dev_desc: ezb_af_device_desc_t) -> Result<(), EzbEspError> {

        for (&ep_id, entry) in self.0 {
            log::debug!("Creating endpoint: {} {:?}", ep_id, entry);

            let ep_desc = create_one_c(ep_id, entry)?;

            // Attach to the device.
            //
            // ESP_ERROR_CHECK(ezb_af_device_add_endpoint_desc(dev_desc, ep_desc));
            //
            EspError::wrap(unsafe {
                ezb_af_device_add_endpoint_desc(dev_desc, ep_desc)
            })?;

            // maybe - depends on the allowed order of C level calls.
            //init_one_c(ep_id, &entry.additional_client_clusters);
        }
        Ok(())
    }

    /// Initialize additional clusters.
    ///
    /// This needs to be called:
    ///     - after the device has been registered
    ///         ^-- tbd. IS THAT EVEN TRUE?
    ///
    /// @note There is no parameter. The 'esp_zigbee_lib' affects the changes based on endpoint
    ///     indices, which are available in the config itself.
    ///
    // #feedback 'esp_zigbee_lib' could be more specific, in its function documentation,
    //     about the valid context of each function call. Like when they *may not* and when the
    //     *can* be called.
    //
    pub(crate) fn init_additional_clusters_c(&self) {
        for (&ep_id, entry) in self.0 {
            log::debug!("Initating endpoint: {}", ep_id);

            init_one_c(ep_id, &entry.additional_client_clusters);
        }
    }
}

// Allow an app to provide a static '&Config' where the needing party only needs a view.
impl From<&'static Config> for EndpointCreator {
    fn from(c: &'static Config) -> Self {
        Self(&c.endpoints)
    }
}

/// Create an end point, in the C library level.
///
/// Add it to the device descriptor 'dev'.
fn create_one_c(ep_id: u8, entry: &Endpoint) -> Result<ezb_af_ep_desc_t, EzbEspError> {
    use DeviceType::*;

    let Endpoint {
        common_fields,
        device_type,
        additional_client_clusters,
        #[cfg(not(feature = "coordinator"))]
        discover_server_clusters,
    } = entry;

    // Note: It is possible to build 'config' with no 'DeviceType' support (not intended for any
    //      endpoints; router only). This would provide us with an "empty enum" (that cannot exist,
    //      and cannot be matched against, even if the code were never to get run, anyways)...
    //
    //      There are ways around this (helpful feature as it otherwise is!):
    //          - add a '_Dummy' clause to the enum in question.   WE DO THIS
    //          - conditionally compile the whole function (based on all 'dt_...' features)
    //          - using 'device_type.clone()' in the match; this tricks the Rust compiler to
    //            allow such matches to exist.
    //
    //      The '.clone()' is arguably the easiest, cleanest, is it..? Note we only need it for router!
    //
    #[cfg(feature = "router")]
    let device_type = device_type.clone();

    let ep_desc = match device_type {
        #[cfg(feature = "dt_color_dimmable_light")]
        DeviceType::ColorDimmableLight => create_color_dimmable_light(ep_id),

        #[cfg(feature = "dt_color_dimmer_switch")]
        DeviceType::ColorDimmerSwitch => create_color_dimmer_switch(ep_id),

        #[cfg(feature = "dt_ias_cie")]
        DeviceType::IasCie => create_ias_cie(ep_id),

        //r _Dummy => unreachable!()
    };

    // ezb_zcl_cluster_desc_t basic_desc = {0};
    // basic_desc = ezb_af_endpoint_get_cluster_desc(ep_desc, EZB_ZCL_CLUSTER_ID_BASIC, EZB_ZCL_CLUSTER_SERVER);
    //
    // basic cluster: id 0
    let basic_desc: ezb_zcl_cluster_desc_t = unsafe {
        ezb_af_endpoint_get_cluster_desc(ep_desc, EZB_ZCL_CLUSTER_ID_BASIC, CLUSTER_SERVER)
    };

    // Add common fields.
    //
    // ezb_zcl_basic_cluster_desc_add_attr(basic_desc, EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID, (void *)ESP_MANUFACTURER_NAME);
    // ezb_zcl_basic_cluster_desc_add_attr(basic_desc, EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID, (void *)ESP_MODEL_IDENTIFIER);
    {
        let CommonFields {
            manufacturer_name,
            model_identifier,
        } = *common_fields;

        for (s_opt, id) in [
            (manufacturer_name, EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID),
            (model_identifier, EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID),
        ] {
            if s_opt.is_none() { continue; }

            let s = s_opt.unwrap();

            let ps: &'static [u8] = PascalString::from(s).into_leaked();
            EzbError::wrap( unsafe {
                ezb_zcl_basic_cluster_desc_add_attr(basic_desc, id, ps.as_ptr() as *const c_void)
            })?;
        }
    }

    // Additional clusters.
    {
        use ezb_node_config::Cluster;

        for cl in additional_client_clusters {
            let cluster_desc = match cl {
                // Implicit for device types we support - should not come about as '.additional_client_clusters'
                Cluster::OnOff |
                Cluster::LevelControl |
                Cluster::IasZone => {
                    unimplemented!()
                },

                Cluster::PowerConfig => {
                    unsafe {
                        ezb_zcl_power_config_create_cluster_desc(
                            core::ptr::null_mut(),
                            CLUSTER_CLIENT as u8
                        )
                    }
                },
            };

            EzbError::wrap(unsafe {
                ezb_af_endpoint_add_cluster_desc(ep_desc, cluster_desc)
            })?;
        }
    }

    #[cfg(not(feature = "coordinator"))]
    {
        discover_server_clusters;
        unimplemented!()
    }

    Ok(ep_desc)
}

/// Initialization places the necessary event listeners on the (already registered) endpoint.
///
fn init_one_c(ep_id: u8, additional_client_clusters: &Vec<Cluster>) {

    for cl in additional_client_clusters {
        match cl {
            // Implicit for device types we support - should not come about as '.additional_client_clusters'
            Cluster::OnOff |
            Cluster::LevelControl |
            Cluster::IasZone => {
                unimplemented!()
            },

            Cluster::PowerConfig => {
                let () = unsafe {
                    ezb_zcl_power_config_cluster_client_init(ep_id)
                };
            },
        };
    }
}


#[cfg(feature = "dt_color_dimmable_light")]
fn create_color_dimmable_light(ep_id: u8) -> ezb_af_ep_desc_t {
    use ezb_node_raw::{
        ezb_zha_color_dimmable_light_config_t,
        ezb_zha_create_color_dimmable_light
    };

    let cfg = ezb_zha_color_dimmable_light_config_t::default();
    unsafe { ezb_zha_create_color_dimmable_light(ep_id, &cfg) }
}

#[cfg(feature = "dt_color_dimmer_switch")]
fn create_color_dimmer_switch(ep_id: u8) -> ezb_af_ep_desc_t {
    use ezb_node_raw::{
        ezb_zha_color_dimmer_switch_config_t,
        ezb_zha_create_color_dimmer_switch
    };

    let cfg = ezb_zha_color_dimmer_switch_config_t::default();
    unsafe { ezb_zha_create_color_dimmer_switch(ep_id, &cfg) }
}

// This variant is based on advice at: https://github.com/espressif/esp-zigbee-sdk/issues/897
#[cfg(feature = "dt_ias_cie")]
fn create_ias_cie(ep_id: u8) -> ezb_af_ep_desc_t {
    use ezb_af_profile_id_e::HA_PROFILE_ID;
    use ezb_zha_device_id_e::IAS_CONTROL_INDICATING_EQUIPMENT_ID;
    use ezb_node_raw::{
        ezb_zcl_ias_ace_create_cluster_desc,
        ezb_zcl_identify_create_cluster_desc,
        ezb_zcl_ias_zone_create_cluster_desc,
        ezb_zcl_ias_wd_create_cluster_desc,
        ezb_af_endpoint_add_cluster_desc,
    };

    let cfg = ezb_af_ep_config_t::new( ep_id,
        HA_PROFILE_ID,
        IAS_CONTROL_INDICATING_EQUIPMENT_ID,
        0   // app device version
    );

    let desc: ezb_af_ep_desc_t = unsafe {
        ezb_af_create_endpoint_desc(&cfg)
    };
    assert!( !desc.is_null(), "Creating a generic endpoint descriptor failed");    // bad config?

    const NULL: *const c_void = core::ptr::null();

    let clusters: [ezb_zcl_cluster_desc_t;4] = [
        unsafe { ezb_zcl_ias_ace_create_cluster_desc(NULL, CLUSTER_SERVER as u8) },
        unsafe { ezb_zcl_identify_create_cluster_desc(NULL, CLUSTER_CLIENT as u8) },
        unsafe { ezb_zcl_ias_zone_create_cluster_desc(NULL, CLUSTER_CLIENT as u8) },
        unsafe { ezb_zcl_ias_wd_create_cluster_desc(NULL, CLUSTER_CLIENT as u8) },
    ];
    assert!( !clusters.iter().any(|c| c.is_null()), "Failed to create some cluster(s): IAS_CIE");

    for cl in clusters {
        EzbError::wrap( unsafe {
            ezb_af_endpoint_add_cluster_desc(desc, cl)
        }).expect("Failed to add cluster: IAS_ACE");
    }

    desc
}

//r This variant is what GOOGLE.AI suggested - building on top of a harmless device type.
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
