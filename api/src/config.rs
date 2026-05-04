/*
* Configuration of a node
*
* Intended to be fed via a TOML, but this is not absolutely necessary. However, the data contents here may be
* flatter, and have duplicates, i.e. the struct is meant to be generated, not necessarily hand coded.
*/
use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::node::ChannelMask;

pub struct Config {
    // Network
    pub primary_channel_mask: ChannelMask,
    pub secondary_channel_mask: ChannelMask,

    // NVRAM
    pub storage_partition_name: String,   // e.g. "zb_storage" (but can be anything)

    // Manufacturer info
    pub manufacturer_name: String,
    pub model_identifier: String,

    pub endpoints: BTreeMap<u8, EndpointConfig>,
}

impl Config {
    // Library should use this when getting one, before using any of it.
    //
    pub(crate) fn check(&self) {
        let valid_ids = 1..=240;

        // Endpoint id's within the valid range
        self.endpoints.keys().for_each(|id| {
            assert!(valid_ids.contains(&id), "invalid endpoint id: {}", id);
        });
    }
}

pub enum EndpointConfig {

    #[cfg(feature = "ep_color_dimmable_light")]
    ColorDimmableLightEPC,
}
