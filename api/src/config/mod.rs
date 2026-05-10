/*
* Configuration of a Zigbee application.
*
* Intended to be fed via a TOML, but this is not absolutely necessary. However, the data contents here may be
* flatter, and have duplicates, i.e. the struct is meant to be generated, not necessarily hand coded.
*/
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use core::ops::RangeInclusive;

use crate::{
    node::ChannelMask,
    utils::PascalString,
};

mod platform_device_node_view;
pub(crate) use platform_device_node_view::PlatformDeviceNodeView;

mod device_descriptor_view;
pub(crate) use device_descriptor_view::DeviceDescriptorView;

use crate::Error::BadConfig;

const VALID_ENDPOINT_IDS: RangeInclusive<u8> = 1..=240;

// Note: Since this is mostly an internal construct (after we go TOML), we can do things like
//      convert strings already initially to '&'static PascalString'. We don't have to be "neat"
//      for the apps developer.
//
// Validity of the struct is checked right up front. If you have one, it's valid!
// A created config is a singleton, and static (lives "forever").
//
pub struct Config {
    // Network
    pub channel_masks: [ChannelMask; 2],

    // Platform
    pub storage_partition_name: String,   // e.g. "zb_storage" (but can be anything)

    // Node
    pub node: NodeType,
        // CoordinatorConfig | RouterConfig | EndDeviceConfig

    //r // Manufacturer info (defaults: we should allow endpoints to carry their own)
    //r pub manufacturer_name: &'static PascalString,
    //r pub model_identifier: &'static PascalString,

    pub endpoints: BTreeMap<u8, (EndpointConfig, BaseConfig)>,
}

impl Config {
    // Library should use this when getting one, before using any of it.
    //
    pub(crate) fn check(&self) -> Result<(),crate::Error> {

        self.endpoints.keys().try_for_each(|id| {
            if !VALID_ENDPOINT_IDS.contains(id) {
                return Err(BadConfig(format!("Invalid endpoint id: {}", id)));
            };
            Ok(())
        })?;

        Ok(())
    }
}

/*
* Node type, and what parameters for it.
*
* Note: while there's duplication in the fields, that's not that bad. If we'll read these
*       from a TOML, there's little value in trying to normalize their keys.
*/
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
    // tbd. consider, whether we wish to expose 'ezb_new_ed_timeout_e' in the API? (if enum, make our own!)
    #[cfg(false)]
    EndDeviceConfig {
        install_code_policy: bool,
        ed_timeout: ezb_nwk_ed_timeout_e,
        keep_alive: Duration,
    }
}

pub enum EndpointConfig {
    #[cfg(feature = "ep_color_dimmable_light")]
    ColorDimmableLightEPC,
}

/**
* Carries the base endpoint [tbd. is that the right term?] (0) configuration. This can be different for each endpoint
* though often the same values would be used.
*/
pub struct BaseConfig {
    pub manufacturer_name: &'static PascalString,
    pub model_identifier: &'static PascalString,
}
