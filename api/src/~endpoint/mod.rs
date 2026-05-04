/*
* Endpoint.
*
* A Zigbee endpoint has one or more Clusters; Basic Cluster being #0.
*/
pub mod profile;

//mod endpoint_config;
//pub use endpoint_config::*;

pub enum Endpoint {
    #[cfg(feature = "ep_color_dimmable_light")]
    ColorDimmableLight,
}
