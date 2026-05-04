/*
* EndpointConfig
*
* Bridges the (TOML-originated) configuration to 'esp-zigbee-lib' C side structs (that can be used for endpoint registration).
*/
use crate::utils::PascalString;

// Zigbee note: Each endpoint carries "basic cluster" as their cluster #0.
//
pub struct EndpointConfig(EndpointType, BasicConfig);

pub struct BasicConfig {
    pub manufacturer_name: PascalString,
    pub model_identifier: PascalString,
}

pub enum EndpointType {
    #[cfg(feature = "ep_color_dimmable_light")]
    ColorDimmableLight(ColorDimmableLightConfig)
}

#[cfg(feature = "ep_color_dimmable_light")]
pub struct ColorDimmableLightConfig {}

#[cfg(feature = "ep_color_dimmable_light")]
impl Default for ColorDimmableLightConfig {
    fn default() -> Self { Self{} }
}

