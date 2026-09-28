//
// Defines the structs that the generated code fills in.
//
// The 'api' depends on us, directly. This means an application only needs to pull in 'ezb-node-config'
// once, on the 'build.rs' side, and the second use (within the application) comes without mentioning.
//
// Note: It's good to keep this module in also for the TOML -> code generation (feature 'toml'):
//      the IDE might allow code completion etc. even within 'quote!' macros!!!
//
// Note: No hesitation at the struct stage, only data (all fields 'pub'). We _can_ help the 'toml'
//      feature with some parsing or validation, but it needs to be in the form of plain functions.
//
extern crate alloc;

use alloc::{
    collections::BTreeMap,
    vec::Vec,
};

mod channel_mask;
pub use channel_mask::ChannelMask;

#[cfg(feature = "coordinator")]
const ROLE: &'static str = stringify!(coordinator);
//not needed for 'router'
// #[cfg(feature = "router")]
// const ROLE: &'static str = stringify!(router);
//#[cfg(feature = "end_device_UNTESTED")]
//const ROLE: &'static str = stringify!(end_device);

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
    /// Partition name (needs to have 'SubType: nvs'). By default, use the common NVS partition
    /// ("nvs") with a namespace.
    pub storage_partition_name: &'static str,

    // Node
    pub node: NodeType,

    // Endpoints
    pub endpoints: BTreeMap<u8, Endpoint>,
}

#[allow(dead_code)]
trait InvariantCheck {
    /// A documentary (not necessarily enforced) way to declare invariants.
    /// The 'convert' part will provide output that should _always_ pass such checks.
    ///
    /// Note: If an invariant is declared for a struct, it must *cover all* potentially invalid
    ///     conditions.
    fn invariant_check(&self);
}

impl InvariantCheck for Config {
    fn invariant_check(&self) {
        self.endpoints.keys().for_each(|id| {
            assert!( (1..=240).contains(id), "Invalid endpoint ID: {}", id);
        });

        // 'self.endpoints' may be empty only for a router (a purely network-extending one).
        // Coordinators and end devices must provide at least one end point.
        //
        #[cfg(not(feature = "router"))]
        assert!( !self.endpoints.is_empty(), "Role {} needs at least one endpoint", ROLE);
    }
}

/**
* Node type and parameters.
*/
#[derive(Debug, Clone)]
pub enum NodeType {
    #[cfg(feature = "coordinator")]
    CoordinatorConfig {
        install_code_policy: bool,
        max_children: u8,
        pre_configured_devices: Vec<PreConfiguredDevice>,
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
#[cfg(feature = "coordinator")]
pub struct PreConfiguredDevice {
    /// Address bytes, parsed from the input string
    pub ieee_address: [u8;8],
    /// Install code + CRC
    pub install_code: [u8;18],
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
///
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
pub enum Cluster {
    OnOff,            // implicit from device type (switch uses for discovery)
    LevelControl,     // implicit from device type (switch uses for discovery)
    IasZone,          // implicit from device type
    PowerConfig,     // for collecting battery information
}

/// Certain endpoint's configuration (except its id, which is carried as a map key).
///
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub common_fields: CommonFields,
    pub device_type: DeviceType,
    pub additional_server_clusters: Vec<Cluster>,
    //#later pub discover_remote_server_clusters: Option<Vec<ZclServerCluster>>,
}
