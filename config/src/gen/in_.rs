#![cfg(feature = "gen")]

/**
* Structs that describe the expected TOML content.
*
* For structs where inner validation is needed (set of valid values etc.) we may provide parsing
* functions. This keeps all the errors on the build side of things.
*/
use serde::{
    de::Error as SerdeDeError,
    Deserialize,
    Deserializer,
};

use itertools::Itertools;

use std::{
    collections::BTreeMap,
    //convert::identity,
    ops::RangeInclusive,
    string::String,
    vec::Vec,
};

use super::error::ConfigError;

/**
* Presents 1:1 relation from the TOML to Struct, for reading things in.
*
* Value validation may be done using custom deserializers.
*/
#[derive(Deserialize, Debug)]
pub struct RootConfig {
    pub network: NetworkSection,
    pub node: NodeSection,
    pub endpoint: EndpointConfig,

    //r /// This is logically a part of 'NodeSection::Coordinator' but having it as a separate root-level
    //r /// entry allows more freedom in how the TOML is arranged. These entries can be at the end.
    //r ///
    //r #[serde(rename = "pre_configured_device", default)]
    //r pub pre_configured_devices: Vec<PreConfiguredDevice>,   // coordinator only
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

//r #[derive(Deserialize, Debug)]
//r pub struct PlatformSection {
//r     #[serde(default)]   // optional
//r     pub storage_partition_name: Option<String>,
//r }

#[derive(Deserialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NodeSection {
    // Note: 'allow_open_pairing' is '!install_code_policy'.
    //      The author wanted to rename it, to:
    //          a) make it more understandable for a larger audience
    //          b) make the boolean term work (having a "policy" true/false is weird)

    #[cfg(feature = "coordinator")]
    Coordinator{
        //r allow_open_pairing: bool,
        max_children: u8,

        // tbd. EXPLORE: does '[[node.pre_configured_device]]' reach us?
        #[serde(rename = "pre_configured_device", default)]
        pre_configured_devices: Vec<PreConfiguredDevice>,
    },
    #[cfg(feature = "router")]
    Router{
        /// Try joining the network with default install code.
        allow_open_pairing: bool,
        max_children: u8
    },
    #[cfg(false)]
    ///#[cfg(feature = "end_device_UNUSED")]
    #[allow(dead_code)]
    EndDevice{  // NOT TESTED
        /// Try joining the network with default install code.
        allow_open_pairing: bool,
        #[serde(with = "humantime_serde")]
        ed_timeout: Duration,
            // tbd. allow only certain durations in validation (which is not to be here..): "10s", ... "16384 minutes"; see 'ezb_new_ed_timeout_e'
        #[serde(with = "humantime_serde")]
        keep_alive: Duration,
    },
}

#[cfg(false)]  //R
impl NodeSection {
    #[cfg(feature = "coordinator")]
    pub(crate) fn is_coordinator(&self) -> bool { matches!(self, Self::Coordinator{..}) }
    #[cfg(feature = "router")]
    pub(crate) fn is_router(&self) -> bool { matches!(self, Self::Router{..}) }
}

#[cfg(feature = "coordinator")]
#[derive(Deserialize, Debug)]
pub struct PreConfiguredDevice {
    /// This is *only* a descriptive name; can be used for error messages, though. It is not needed
    /// by the underlying 'esp_zigbee_sdk', but might prove useful for the human managing a configuration.
    #[serde(rename = "log_name")]
    pub(crate) _name: String,

    #[serde(deserialize_with = "chunked_hex")]   // "aaaa ... dddd" (8 bytes)
    pub(crate) mac_address: [u8;8],
    #[serde(deserialize_with = "chunked_hex")]   // "aaaa bbbb ..." (18 bytes)
    pub(crate) install_code: [u8;18],
}

#[derive(Deserialize, Debug)]
pub struct EndpointConfig {
    #[serde(default)]
    defaults: EndpointDefaults,

    #[serde(flatten)]
    // Note: 'String' (not 'u8') as map key avoids collision with ".defaults".
    //  tbd. Try using 'u8'.
    instances: BTreeMap<String, EndpointInstance>,
}

impl EndpointConfig {
    // tbd. instead of '.parse', use 'serde(deserialize_with = ...' on 'instances'

    /// Convert 'String' to 'u8' and also validate the values are valid endpoint indices.
    ///
    #[deprecated]
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

    /// Clusters (client and/or server) that a binding candidate must provide, in order for the join
    /// to take place. FOR ROUTER AND END DEVICE ONLY.
    ///
    /// @note We need to sometimes let it exist also for a Coordinator, and just not be used (we'll
    ///     give an alert later, if it does exist). This is for cases where coordinator *and* some
    ///     other node type are enabled ("gen" allows this).
    ///
    //let it always exist: #[cfg(any(feature = "router" /*, feature = "end_device_NOT_YET"*/))]
    #[serde(default)]   // optional
    pub discover_server_clusters: Option<Vec<Cluster>>,
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

/// Deserialize "aabb ccdd ..." (four chars hex + one space; repeat) into byte arrays.
///
/// @note This is the syntax printed on Zigbee devices, for both MAC address and Install Code.
///
fn chunked_hex<'de, D, const N: usize>(deserializer: D) -> Result<[u8;N], D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;   // "aaaa bbbb ..."

   let bytes: Vec<u8> = s.chars()
       .chunk_by(|&c| c == ' ')    // leads to: (false, "aabb"), (true, " "), ...
       .into_iter()

       // key: the evaluated key for each chunk
       // chunk: the portion, where the 'key' stayed constant
       //
       .map(|(is_white_space, chunk)| {
           let chunk: String = chunk.collect();

           log::warn!("XXX iterating: {} '{:?}'", is_white_space, chunk);    // DEBUG

           if is_white_space {
               // Note: We could silently allow the extra spaces
               if chunk.len() > 1 {
                   return Err(format!("Expected one space between hex blocks (got {}): {}", chunk.len(), s));
               }
               Ok(vec!())   // nothing
           } else {
               if chunk.len() != 4 {
                   return Err(format!("Expected blocks of 4 hex chars, got: '{}': {}", &chunk, s));
               }

               let mut buf = [0u8; 2];

               for i in 0..=1 {
                   let sl = &chunk[i*2..=i*2+1];
                   buf[i]= u8::from_str_radix(&sl, 16).map_err(|_| {
                       format!("Expected hexadecimal, got: '{}'", &sl)
                   })?;
               }
               Ok(buf.to_vec())
           }
       })
       .flatten_ok()
       .collect::<Result<Vec<u8>, String>>()
       .map_err(D::Error::custom)?;

    if bytes.len() != N {
        return Err(D::Error::custom(format!(
            "Unexpected length ({} != {}): {}", bytes.len(), N, &s )));
    }

    let mut arr = [0u8; N];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

//---

/*** DISABLED; simpler tests, please!?

/// Helper for reading a string with repeating syntax (a MAC address or install code), into
/// byte slice.
///
// tbd. simplify; now only supporting DELIM=<space> and N=4
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

//impl Hexable for u8 {
//    const BYTES: usize = 1;
//    fn from_hex(s: &str) -> Result<Self, ParseIntError> {
//        Self::from_str_radix(s, 16)
//    }
//}

impl Hexable for u16 {
    const BYTES: usize = 2;
    fn from_hex(s: &str) -> Result<Self, ParseIntError> {
        Self::from_str_radix(s, 16)
    }
}

fn parse_mac_address(s: &str) -> Option<[u8; 8]> {
    // First try with "00:0D:6F:..." -> [u8; 8]
    //f11::<':', 8, u8>(s)
        //.or_else(|| {
            // ..if not, with Zigbee on-device print: "000D 6F..." -> [u16; 4]
            f11::<' ', 4, u16>(s).and_then(|u16_array| {
                let bytes_vec: Vec<u8> = u16_array
                    .iter()
                    .flat_map(|&val| val.to_be_bytes()) // e.g. 0x000D -> [0x00, 0x0D]
                    .collect();

                bytes_vec.try_into().ok()
            })
        //})
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
        //assert_eq!(parse_mac_address("12:34:56:78:90:AB:cd:EF"), Some(mac_sample));
        assert_eq!(parse_mac_address("1234 5678 90AB cdEF"), Some(mac_sample));

        // Fails on wrong length
        //assert_eq!(parse_mac_address("12:34:56:78:90:AB:cd"), None);
        assert_eq!(parse_mac_address("1234 5678 90AB"), None);

        // Fails on non-hex
        //assert_eq!(parse_mac_address("12:34:56:78:90:AB:cd:ZZ"), None);
        assert_eq!(parse_mac_address("1234 5678 90AB cdZZ"), None);

        // Fails on wrongly placed delimiter
        //assert_eq!(parse_mac_address("1234:5678:90:AB:cd:EF"), None);
        assert_eq!(parse_mac_address("12345 678 90AB cdEF"), None);

        // Fails on wrong delimiter
        assert_eq!(parse_mac_address("1234_5678 90AB cdEF"), None);
    }
}
***/
