
use alloc::{
    collections::{
        btree_map,
    }
};

use crate::config::Config;
use crate::{BaseConfig, EndpointConfig};

/**
* A view to 'Config' used in initializing the 'DeviceDescriptor' (i.e. endpoints).
*/
pub(crate) struct DeviceDescriptorView<'a>(&'a Config);

impl<'a> DeviceDescriptorView<'a> {

    pub(crate) fn expand(&self) -> btree_map::Iter<u8, (EndpointConfig, BaseConfig)> {
        self.0.endpoints.iter()
    }
}
