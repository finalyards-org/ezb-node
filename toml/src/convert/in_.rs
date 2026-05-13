#![cfg(feature = "toml")]

use serde::Deserialize;
use std::collections::HashMap;

//use core::time::Duration;

/**
* Presents 1:1 relation from the TOML to Struct, for reading things in.
*
* @note Not the eventual 'Config' format, which resides in 'esp_zb::Config'.
*/
#[derive(Deserialize, Debug)]
pub struct RootConfig {
    pub network: NetworkSection,
    pub platform: PlatformSection,
    pub node: NodeSection,
    pub endpoint: EndpointConfig,
}

#[derive(Deserialize, Debug)]
pub struct NetworkSection {
    pub primary_channels: Vec<u8>,
    pub secondary_channels: SecondaryChannels,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SecondaryChannels {
    All,
    Preferred
}

#[derive(Deserialize, Debug)]
pub struct PlatformSection {
    pub storage_partition_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeSection {
    Coordinator{ install_code_policy: bool, max_children: u8 },
    Router{ install_code_policy: bool, max_children: u8 },
    #[cfg(false)]
    EndDevice{ install_code_policy: bool }, // tbd. there's more fields
}

#[derive(Deserialize, Debug)]
pub struct EndpointConfig {
    pub defaults: EndpointDefaults,

    #[serde(flatten)]
    pub instances: HashMap<u8, EndpointInstance>,
}

#[derive(Deserialize, Debug)]
pub struct EndpointDefaults {
    pub manufacturer_name: Option<String>,
    pub model_identifier: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "device_type", rename_all = "snake_case")]
pub enum EndpointInstance {
    ColorDimmableLight {
        //| #[serde(with = "humantime_serde")]
        //| transition_time: Option<Duration>,  // an example, for now
    }
}
