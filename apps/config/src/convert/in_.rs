#![cfg(feature = "toml")]

use serde::Deserialize;

use std::{
    collections::BTreeMap,
    string::String,
    vec::Vec,
};

//use core::time::Duration; //  use humantime instead, if needed

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
    pub secondary_channels: SecondaryChannels,  // want to keep it compulsory
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
    // Note: on input, we wish to allow all values, and err at the 'Config' construction.
    //      This allows us to be in charge of the error messages.
    Coordinator{ install_code_policy: bool, max_children: u8 },
    Router{ install_code_policy: bool, max_children: u8 },
    #[cfg(false)]
    EndDevice{
        install_code_policy: bool,
        ed_timeout: Duration, // tbd. use humantime (or perhaps a custom enum?), but allow only certain
            // durations: "10s", ... "16384 minutes"; see 'ezb_new_ed_timeout_e'
        #[serde(with = "humantime_serde")]
        keep_alive: Duration,
    },
}

#[derive(Deserialize, Debug)]
pub struct EndpointConfig {
    #[serde(default)]
    pub defaults: EndpointDefaults,

    #[serde(flatten)]
    // Note: 'String' (not 'u8') as map key avoids collision with ".defaults".
    pub instances: BTreeMap<String, EndpointInstance>,
}

#[derive(Deserialize, Debug, Default)]
pub struct EndpointDefaults {
    pub manufacturer_name: Option<String>,
    pub model_identifier: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub struct EndpointInstance {
    pub device_type: DeviceType,
    /// In addition to the clusters brought in by the '.device_type', add some more. Optional.
    #[serde(default)]   // optional field
    pub additional_server_clusters: Vec<ServerCluster>,
    /// Look for a remote end point with said server-side clusters, bind to it.
    ///
    /// Note: An end point can only bind to one other endpoint (it's 1:1). The list defines the
    ///     clusters required from such an end point.
    ///
    #[serde(default)]
    #[allow(dead_code)] // tbd. finish it. Perhaps grouped as 'discover.remote_server_clusters' (could include also a field for client clusters, of the same candidate)
    pub discover_remote_server_clusters: Option<Vec<ServerCluster>>,
        // tbd. check in construction that if given, must contain at least one cluster
}

#[derive(Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum DeviceType {
    #[cfg(feature = "dt_color_dimmable_light")]
    #[serde(rename = "color_dimmable_light", alias = "HA::color_dimmable_light")]
    HA_ColorDimmableLight,

    #[cfg(feature = "dt_color_dimmer_switch")]
    #[serde(rename = "color_dimmer_switch", alias = "HA::color_dimmer_switch")]
    HA_ColorDimmerSwitch,

    #[cfg(feature = "dt_ias_cie")]
    #[serde(rename = "ias_cie", alias = "HA::ias_cie")]
    HA_IasCie,
}

// Note: Though device types are feature-driven, the supported clusters are always-on. Would be
//      too much hassle to selectively include them all?
#[derive(Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum ServerCluster {
    #[serde(rename = "on_off", alias = "HA::on_off")]
    HA_OnOff,
    #[serde(rename = "level_control", alias = "HA::level_control")]
    HA_LevelControl,
    #[serde(rename = "power_config", alias = "HA::power_config")]
    HA_PowerConfig,
    #[serde(rename = "ias_zone", alias = "HA::ias_zone")]
    HA_IasZone,
}
