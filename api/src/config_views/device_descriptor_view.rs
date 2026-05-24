
use std::{
    collections::{
        btree_map,
    }
};

use ezb_node_config::{
    BaseConfig,
    Config,
    EndpointConfig,
};

/**
* A view to 'Config' used in initializing the 'DeviceDescriptor' (i.e. endpoints).
*
* Covers TOML sections '[endpoint.defaults]' and '[endpoint.{id}]'.
*/
pub struct DeviceDescriptorView<'a>(&'a Config);

impl<'a> DeviceDescriptorView<'a> {

    pub(crate) fn expand(&self) -> ! /*(BaseConfig, btree_map::Iter<u8, EndpointConfig>)*/ {
        let a = &self.0.endpoint_defs;
        let b = self.0.endpoints.iter();

        // tbd. provide 'ezb_zigbee_lib' entities
        unimplemented!()
        //(a,b)
    }
}

// This allows an app to provide '&Config' where the needing party only takes a view.
impl<'a> From<&'a Config> for DeviceDescriptorView<'a> {
    fn from(c: &'a Config) -> Self {
        Self(c)
    }
}
