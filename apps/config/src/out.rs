extern crate alloc;

use alloc::{
    collections::BTreeMap,
    vec::Vec
};
use core::ops::RangeInclusive;

use crate::ChannelMask;

/**
* Configuration for the 'ezb_node'.
*
* We check for validity - if a configuration exists, it's also valid.
*/
pub struct Config {
    // Network
    pub primary_channels: ChannelMask,
    pub secondary_channels: ChannelMask,

    // Platform
    pub storage_partition_name: &'static str,   // e.g. "zb_storage" (but can be anything)

    // Node
    pub node: NodeType,

    // Endpoints
    pub endpoints: BTreeMap<u8, Endpoint>,
}

impl Config {
    //R
    /// Allows TOML->str code generation to check against invalid indices.
    #[cfg(feature = "toml")]
    #[cfg(false)]   // we do it already in generation; consider leaving this as a documentary method,
                    // but in such case it should actually check everything. Which is does not.
    fn invariant_check(&self) {
        self.endpoints.keys().for_each(|&id| {
            assert!(Endpoint::is_valid_id(id), "Invalid endpoint ID: {}", id);
        });
        assert!(!self.endpoints.is_empty(), "Missing '[endpoints.{{id}}]'")
    }
}

/**
* Node type and parameters.
*/
#[derive(Debug, Copy, Clone)]
pub enum NodeType {
    #[cfg(feature = "coordinator")]
    CoordinatorConfig {
        install_code_policy: bool,
        max_children: u8,
    },
    #[cfg(feature = "router")]
    RouterConfig {
        install_code_policy: bool,
        max_children: u8,
    },
    //#[cfg(feature = "end_device_UNTESTED")]
    // EndDeviceConfig {
    //     install_code_policy: bool,
    //     //ed_timeout: ezb_nwk_ed_timeout_e, // tbd. need work on that one: likely enum, similar to 'ezb_new_ed_timeout_e'
    //     //keep_alive: Duration,
    // }
}

#[derive(Debug, Copy, Clone)]
pub struct CommonFields {
    pub manufacturer_name: Option<&'static str>,
    pub model_identifier: Option<&'static str>,
}

/// A device type defines an endpoint's identity. "Who I am?" (What I can).
/// They imply a set of ZCL clusters. List here|1|.
///
/// |1|: https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32/introduction.html#home-automation-device-types
#[derive(Debug, Copy, Clone)]
pub enum DeviceType {
    #[cfg(feature = "dt_color_dimmable_light")]
    ColorDimmableLight,
    #[cfg(feature = "dt_color_dimmer_switch")]
    ColorDimmerSwitch,
    #[cfg(feature = "dt_ias_cie")]
    IasCie,
}

///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum ZclServerCluster {
    OnOff,            // implicit from device type (switch uses for discovery)
    LevelControl,     // implicit from device type (switch uses for discovery)
    IasZone,          // implicit from device type
    #[cfg(feature = "cl_power_config")]
    HA_PowerConfig,     // for collecting battery information
}

///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
#[cfg(false)]   // #later
pub enum ZclClientCluster {
    //OnOff,            // implicit from device type
    //LevelControl,     // implicit from device type
    //IasZone,          // implicit from device type
    #[cfg(feature = "cl_power_config_client")]
    HA_PowerConfig,     // for providing battery information
}

/// Certain endpoint's configuration (except its id, which is carried as a map key).
///
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub common_fields: CommonFields,
    pub device_type: DeviceType,
    pub additional_server_clusters: Vec<ZclServerCluster>,
        //#later pub match_remote_server_clusters: Option<Vec<ZclServerCluster>>,
}

// Though 'id' is not part of the 'Endpoint' struct, we can help validate them.
//
impl Endpoint {
    pub(crate) const VALID_RANGE: RangeInclusive<u8> = 1..=240;

    pub(crate) fn is_valid_id(id: u8) -> bool {
        Self::VALID_RANGE.contains(&id)
    }
}
