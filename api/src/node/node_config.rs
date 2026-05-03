/*
* Configuration covering a node.
*/
use alloc::{
    collections::BTreeMap,
};
use super::ChannelMask;
use crate::endpoint::EndpointConfig;

/**
* Configuration structure defining Node parameters, limits etc.
*
* @note This is mostly for internal (behind-the-curtains) use for the application. It reads a TOML
*       file (in 'build.rs') and creates suitable code using this API. This means we can have a flatter
*       layout (e.g. no "[network]" section though the TOML might have one.
*/
pub struct NodeConfig {
    pub channel_mask: Option<ChannelMask>,
    pub max_children: Option<u8>,

    pub endpoints: BTreeMap<u8, EndpointConfig>,
}
