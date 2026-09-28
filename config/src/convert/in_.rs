#![cfg(feature = "toml")]

/**
* Structs that describe the expected TOML content.
*
* For structs where inner validation is needed (set of valid values, string syntax etc.) we provide
* parsing functions. This keeps all the errors on the build side of things.
*/
use serde::Deserialize;

use std::{
    collections::BTreeMap,
    string::String,
    vec::Vec,
};

use core::time::Duration;
use std::ops::RangeInclusive;
use crate::convert::error::ConfigError;

/**
* Presents 1:1 relation from the TOML to Struct, for reading things in.
*
* @note Not the eventual 'Config' format, which resides in 'ezb_node_config::Config'.
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
    #[serde(default)]   // optional
    pub storage_partition_name: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeSection {
    // Note: All variants are included. This allows us to provide better error messages (i.e. we
    //      can read in a TOML variant that is not supported by the 'feature' set of the build).

    // Note: 'allow_open_pairing' is '!install_code_policy'. The author wanted to rename it, to:
    //          a) make it more understandable for a larger audience
    //          b) make the boolean term work (having a "policy" true/false is weird)
    //
    Coordinator{
        allow_open_pairing: bool,
        max_children: u8,
        #[serde(default)]
        pre_configured_devices: Vec<PreConfiguredDevice>,
    },
    Router{
        allow_open_pairing: bool,
        max_children: u8
    },
    #[allow(dead_code)]
    EndDevice{  // NOT TESTED
        allow_open_pairing: bool,
        #[serde(with = "humantime_serde")]
        ed_timeout: Duration,
            // tbd. allow only certain durations in validation (which is not to be here..): "10s", ... "16384 minutes"; see 'ezb_new_ed_timeout_e'
        #[serde(with = "humantime_serde")]
        keep_alive: Duration,
    },
}

#[derive(Deserialize, Debug)]
pub struct PreConfiguredDevice {
    /// This is *only* a descriptive name; can be used for error messages, though. It is not needed
    /// by the underlying 'esp_zigbee_sdk', but might prove useful for the human managing a configuration.
    #[serde(rename = "name")]
    _name: String,

    ieee_address: String,   // "xx:yy:...:zz" | "xxyy ... __zz" (8 bytes)
    install_code: String,   // "aaaa bbbb ..." (18 bytes)
}

#[cfg(feature = "coordinator")]
impl PreConfiguredDevice {
    pub(crate) fn parse(self) -> Result<([u8;8], [u8;18]),ConfigError> {
        let Self { ieee_address, install_code, _name: _ } = self;

        let a = parse_mac_address(ieee_address.as_str()).ok_or_else(|| {
            format!("Not correct syntax for '.ieee_address': {}", ieee_address)
        })?;

        let b = parse_install_code(install_code.as_str()).ok_or_else(|| {
            format!("Not correct syntax for '.install_code': {}", install_code)
        })?;

        Ok((a, b))
    }
}

#[derive(Deserialize, Debug)]
pub struct EndpointConfig {
    #[serde(default)]
    defaults: EndpointDefaults,

    #[serde(flatten)]
    // Note: 'String' (not 'u8') as map key avoids collision with ".defaults".
    instances: BTreeMap<String, EndpointInstance>,
}

impl EndpointConfig {
    /// Convert 'String' to 'u8' and also validate the values are valid endpoint indices.
    ///
    pub(crate) fn parse(/*mut*/ self) -> Result<(EndpointDefaults, BTreeMap<u8,EndpointInstance>), ConfigError> {
        let a = self.defaults;

        // Remove '.defaults'
        //
        // NOTE: google.ai claims that Serde automatically removes an explicitly stated key (.defaults)
        //      from such a flattened map. It this is so, we can safely remove this.
        #[cfg(false)]
        let _ = self.instances.remove("defaults");

        let b = self.instances.into_iter()
            .map(|(key_str, v)| {
                let key_num: u8 = key_str
                    .parse::<u8>()
                    .map_err(|_| format!("Invalid endpoint ID (not 'u8'): {}", key_str))?;

                let range = &Self::VALID_INDICES;
                if !range.contains(&key_num) {
                    return Err(format!("Invalid endpoint ID (∉ '{range:?}'): {}", key_num));
                }

                Ok((key_num, v))
            })
            .collect ::<Result<_,_>>()?;

        Ok((a, b))
    }

    const VALID_INDICES: RangeInclusive<u8> = 1..=240;
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
    #[serde(default)]   // optional
    pub additional_client_clusters: Vec<Cluster>,
    #[serde(default)]   // optional
    #[cfg(false)]
    pub additional_server_clusters_NOT_YET: Vec<Cluster>, // enable if needed

    /// Look for a remote end point with said server-side clusters, bind to it.
    ///
    /// Note: An end point can only bind to one other endpoint (it's 1:1). The list defines the
    ///     clusters required from such an end point.
    ///
    #[serde(default)]
    #[allow(dead_code)] // tbd. finish it. Perhaps grouped as 'discover.remote_server_clusters' (could include also a field for client clusters, of the same candidate)
    #[cfg(false)]
    pub discover_remote_server_clusters_UNTESTED: Option<Vec<Cluster>>,
        // tbd. check in construction that if given, must contain at least one cluster.
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

    //r /// Dummy for tests. This allows us to build without enabling *any* of the 'dt_...' features.
    //r //#[cfg(test)]
    //r TestDummy,
}

// Note: Supported clusters are not feature-flagged. Would be too much hassle to selectively include
//      them all.
#[derive(Deserialize, Debug)]
#[allow(non_camel_case_types)]
pub enum Cluster {
    #[serde(rename = "on_off", alias = "HA::on_off")]
    HA_OnOff,
    #[serde(rename = "level_control", alias = "HA::level_control")]
    HA_LevelControl,
    #[serde(rename = "power_config", alias = "HA::power_config")]
    HA_PowerConfig,
    #[serde(rename = "ias_zone", alias = "HA::ias_zone")]
    HA_IasZone,
}

//---

/// Helper for reading a string with repeating syntax (an IEEE address or install code), into
/// byte slice.
///
/// For inputs with '{XX}{delim}', use 'T = u8'.
/// For inputs with '{XXYY}{delim}', use 'T = u16'
///
fn f11<const DELIM: char, const N: usize, T: Hexable>(s: &str) -> Option<[T;N]> {

    let v: Vec<T> = s.split(DELIM)
        .map(|piece| {
            if piece.len() == T::BYTES*2 {  // 2 chars per byte
                T::from_hex(piece).ok()
            } else {
                None
            }
        })
        .collect ::<Option<_>>()?;

    v.try_into().ok()
}


//---
// Disclaimer!
//
// The below tools became more elaborate than thought.
//  - see if there are easier ways
//  - for now, "good enough; safe enough"!!!
//
use std::num::ParseIntError;

pub trait Hexable: Sized {
    const BYTES: usize;

    fn from_hex(s: &str) -> Result<Self, ParseIntError>;
}

impl Hexable for u8 {
    const BYTES: usize = 1;
    fn from_hex(s: &str) -> Result<Self, ParseIntError> {
        Self::from_str_radix(s, 16)
    }
}

impl Hexable for u16 {
    const BYTES: usize = 2;
    fn from_hex(s: &str) -> Result<Self, ParseIntError> {
        Self::from_str_radix(s, 16)
    }
}

fn parse_mac_address(s: &str) -> Option<[u8; 8]> {
    // First try with "00:0D:6F:..." -> [u8; 8]
    f11::<':', 8, u8>(s)
        .or_else(|| {
            // ..if not, with Zigbee on-device print: "000D 6F..." -> [u16; 4]
            f11::<' ', 4, u16>(s).and_then(|u16_array| {
                let bytes_vec: Vec<u8> = u16_array
                    .iter()
                    .flat_map(|&val| val.to_be_bytes()) // e.g. 0x000D -> [0x00, 0x0D]
                    .collect();

                bytes_vec.try_into().ok()
            })
        })
}

fn parse_install_code(s: &str) -> Option<[u8;18]> {
    f11::<' ', 9, u16>(s).and_then(|u16_array| {
        let bytes_vec: Vec<u8> = u16_array
            .iter()
            .flat_map(|&val| val.to_be_bytes()) // e.g. 0x000D -> [0x00, 0x0D]
            .collect();

        bytes_vec.try_into().ok()
    })
}

#[cfg(test)]
mod tests {
    use super::{f11, parse_mac_address};

    #[test]
    fn test_f11() {
        let mac_sample = [0x12, 0x34, 0x56, 0x78, 0x90, 0xAB, 0xCD, 0xEF];

        // Can parse hex (ignoring case)
        assert_eq!(parse_mac_address("12:34:56:78:90:AB:cd:EF"), Some(mac_sample));

        // ..also as written on devices
        assert_eq!(parse_mac_address("1234 5678 90AB cdEF"), Some(mac_sample));

        // Fails on wrong length
        assert_eq!(parse_mac_address("12:34:56:78:90:AB:cd"), None);
        assert_eq!(parse_mac_address("1234 5678 90AB"), None);

        // Fails on non-hex
        assert_eq!(parse_mac_address("12:34:56:78:90:AB:cd:ZZ"), None);

        // Fails on wrongly placed delimiter
        assert_eq!(parse_mac_address("1234:5678:90:AB:cd:EF"), None);

        // Fails on wrong delimiter
        assert_eq!(parse_mac_address("12 34 56 78 90 AB cd EF"), None);
    }
}
