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
#[allow(dead_code)]
/// Commentary code (not necessarily run), to define the invariants.
impl non_gen::InvariantCheck for Config {
    fn assert_invariants(&self) {
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

impl Config {
    /// Find the index of the local end point that implements a certain device type.
    ///
    // @note If we replace 'BTreeMap' with a const-friendly construct, also this can be done
    //      as 'const fn'.
    //
    pub fn my_ep_for(&self, dt: DeviceType) -> Option<u8> {

        self.endpoints.iter()
            .find(|(_,entry)| entry.device_type == dt)
            .map(|(&id, _)| id)
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

#[derive(Debug, Copy, Clone)]
#[cfg(feature = "coordinator")]
pub struct PreConfiguredDevice {
    /// IEEE (MAC) address as a number; "4030 59FF FE9E 7D3D" -> 0x4030_59FF_FE9E_7D3D
    ///
    /// Note: Kept as a 'u64' so that the (little endian) byte order of the C side is handled by the
    ///     'api' level.
    pub mac_address: u64,
    /// Install code + CRC, in the order printed on the device
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
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DeviceType {
    #[cfg(feature = "dt_color_dimmable_light")]
    ColorDimmableLight,
    #[cfg(feature = "dt_color_dimmer_switch")]
    ColorDimmerSwitch,
    #[cfg(feature = "dt_ias_cie")]
    IasCie,
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
    /// @note Intentionally 'Option<Vec<..>>': discovers end points that have _all_ the stated
    ///         clusters.
    ///
    #[cfg(not(feature = "coordinator"))]
    pub discover_server_clusters: Option<Vec<Cluster>>,
}
