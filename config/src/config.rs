extern crate alloc;

use alloc::collections::BTreeMap;
use core::ops::RangeInclusive;

use crate::ChannelMask;

pub(crate) const VALID_ENDPOINT_IDS: RangeInclusive<u8> = 1..=240;

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
    /// @note Always has one. If config didn't have this section, all its fields are 'None'.
    //r #[cfg(false)] //r
    //r pub endpoint_defs: BaseConfig,

    pub endpoints: BTreeMap<u8, EndpointConfig>,
}

impl Config {
    pub(crate) fn is_valid_endpoint(id: u8) -> bool {
        VALID_ENDPOINT_IDS.contains(&id)
    }

    // Commentary; TOML->str code generation also checks against invalid indices.
    #[allow(dead_code)]
    fn invariant_check(&self) {

        self.endpoints.keys().for_each(|&id| {
            assert!(Self::is_valid_endpoint(id), "Invalid endpoint ID: {}", id);
        });
        assert!(!self.endpoints.is_empty(), "Missing '[endpoints.{{id}}]'")
    }
}

/**
* Node type, and what parameters for it.
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

pub struct CommonFields {
    pub manufacturer_name: &'static str,
    pub model_identifier: &'static str,
}

pub enum Specific {
    #[cfg(feature = "ep_color_dimmable_light")]
    ColorDimmableLightEPC,
}

// Each config has both common fields, and their specific type (and possibly config).
pub struct EndpointConfig(pub CommonFields, pub Specific);
