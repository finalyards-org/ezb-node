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
    primary_channel_mask: ChannelMask,
    secondary_channel_mask: ChannelMask,

    // NVRAM
    storage_partition_name: String,   // e.g. "zb_storage" (but can be anything)

    // Manufacturer info
    manufacturer_name: String,
    model_identifier: String,

    endpoints: BTreeMap<u8, EndpointConfig>,
}

impl Config {
    // Library should use this when getting one, before using any of it.
    //
    pub(crate) fn assert_invariants(&self) {
        let valid_ids = 1..=240;

        // Endpoint id's within the valid range
        self.endpoints.keys().for_each(|id| {
            assert!(valid_ids.contains(&id), "invalid endpoint id: {}", id);
        });
    }
}

pub struct EndpointConfig {

}
