extern crate alloc;

use alloc::collections::BTreeMap;
use core::ops::RangeInclusive;

pub(crate) const VALID_ENDPOINT_IDS: RangeInclusive<u8> = 1..=240;

use crate::ChannelMask;

/**
* Configuration for the 'ezb_node'.
*
* We check for validity - if a configuration has been created, it's also valid.
*/
pub struct Config {
    // Network
    pub channel_masks: [ChannelMask; 2],

    // Platform
    pub storage_partition_name: &'static str,   // e.g. "zb_storage" (but can be anything)

    // Node
    pub node: NodeType,

    // Endpoints
    pub endpoint_defs: BaseConfig,
    pub endpoints: BTreeMap<u8, EndpointConfig>,
}

impl Config {
    // Commentary; TOML->str code generation also checks against invalid indices.
    #[allow(dead_code)]
    fn invariant_check(&self) {

        self.endpoints.keys().for_each(|id| {
            assert!(VALID_ENDPOINT_IDS.contains(id), "Invalid endpoint ID: {}", id);
        });
        assert!(!self.endpoints.is_empty(), "Missing '[endpoints.{id}]'")
    }
}

/**
* Node type, and what parameters for it.
*/
pub enum NodeType {
    //#[cfg(feature = "coordinator")]
    CoordinatorConfig {
        install_code_policy: bool,
        max_children: u8,
    },
    //#[cfg(feature = "router")]
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

/**
* Carries the basic cluster information, often shared with all endpoints.
*/
pub struct BaseConfig {
    pub manufacturer_name: &'static str,
    pub model_identifier: &'static str,
}

pub enum EndpointConfig {
    #[cfg(feature = "ep_color_dimmable_light")]
    ColorDimmableLightEPC,
}
