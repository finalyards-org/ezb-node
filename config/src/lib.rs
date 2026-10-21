// If targeting embedded, add ability to use 'alloc'
#![cfg_attr(not(feature = "gen"), no_std)]

// IDE: don't suggest 'assert_eq!'
#![allow(clippy::assert_eq_with_expected_predicate)]

extern crate alloc;

#[cfg(feature = "gen")]
mod r#gen;
#[cfg(feature = "gen")]
pub use r#gen::*;

use log as _;   // don't warn even if no logging is used

#[cfg(not(feature = "gen"))]
mod non_gen;

// 'gen' gets two things from keeping the structs it generates in the build:
//  - validity checking, parsing etc. functions (we DID those, now moved to 'gen' Serde deserializers)
//  - code completion within the 'quote!' macros

use alloc::{
    collections::BTreeMap,
    vec::Vec,
};

mod channel_mask;
pub use channel_mask::ChannelMask;

// Note: No hesitation at the struct stage, only data (all fields 'pub'). We _can_ help the 'toml'
//      feature with some parsing or validation, but it needs to be in the form of plain functions.

/**
* Configuration for the 'ezb_node'.
*
* We check for validity - if a configuration exists, it's also valid.
*/
pub struct Config {
    // Network
    pub primary_channels: ChannelMask,
    pub secondary_channels: ChannelMask,

    // Node
    pub node: NodeType,

    // Endpoints
    pub endpoints: BTreeMap<u8, Endpoint>,
}

#[cfg(not(feature = "gen"))]
impl non_gen::InvariantCheck for Config {
    fn invariant_check(&self) {
        self.endpoints.keys().for_each(|id| {
            assert!( (1..=240).contains(id), "Invalid endpoint ID: {}", id);
        });

        // 'self.endpoints' may be empty only for a router (a purely network-extending one).
        // Coordinators and end devices must provide at least one end point.
        //
        #[cfg(not(feature = "router"))]
        assert!( !self.endpoints.is_empty(), "Role {} needs at least one endpoint", Self::ROLE);
    }
}

/**
* Node type and parameters.
*/
#[derive(Debug, Clone)]
pub enum NodeType {
    #[cfg(feature = "coordinator")]
    CoordinatorConfig {
        max_children: u8,

        /// MAC address, install code combinations that the coordinator will let join the network.
        /// Implies "install code policy" (!"allow open pairing"):
        ///     - if any are provided, publicly known install code ("Zigbee09Alliance") will not work
        ///     - if none are provided, open pairing (with known install codes) is allowed
        ///
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

// 'esp_zigbee_lib' 2.0.4 has the 'ezb_extaddr_t' "packed" (aligned to 1). This might, however,
// change. Our alignment to 8 is mere robustness. Allows a pointer to the static config to be used,
// all the way to C level.
//
#[derive(Debug, Copy, Clone)]
#[cfg(feature = "coordinator")]
#[repr(C, align(8))]
pub struct PreConfiguredDevice {
    /// Address bytes, parsed from the input string
    pub mac_address: [u8;8],
    /// Install code + CRC
    pub install_code: [u8;18],
}

#[derive(Debug, Copy, Clone)]
pub struct CommonFields {
    pub manufacturer_name: Option<&'static str>,
    pub model_identifier: Option<&'static str>,
}

/// A device type defines an endpoint's identity. "Who am I?" (What can I do?).
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

    // It's possible to build a Router node with no end points (no device types). In such cases,
    // this takes care of not creating an "empty enum" that complicates API layer coding.
    //r: still considering the approach. _Dummy,
}

#[derive(Debug, Copy, Clone)]
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
    pub additional_client_clusters: Vec<Cluster>,
    //pub additional_server_clusters: Vec<Cluster>,

    /// Clusters the endpoint wants to see, in order to bind. [router and end device only]
    ///
    /// @note Coordinator does not need active cluster discovery; it is sufficient for other nodes
    ///         to discover and bind to its endpoints.
    ///
    #[cfg(not(feature = "coordinator"))]
    pub discover_server_clusters: Option<Vec<Cluster>>,
}
