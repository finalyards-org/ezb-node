use crate::node::ChannelMask;
use super::Config;

/**
* A view to a 'Config' struct that only allows access to config entries needed by 'Node'.
*/
pub(crate) struct NodeView<'a>(&'a Config);

impl<'a> NodeView<'a> {
    pub fn get_primary_channel_mask(&self) -> ChannelMask {
        self.0.primary_channel_mask
    }
    pub fn get_secondary_channel_mask(&self) -> ChannelMask {
        self.0.secondary_channel_mask
    }
}
