/*
* Endpoint.
*
* A Zigbee endpoint has one or more Clusters; Basic Cluster being #0.
*/
#[cfg(feature = "ep_color_dimmable_light")]
mod color_dimmable_light;
#[cfg(feature = "ep_color_dimmable_light")]
pub use color_dimmable_light::*;

mod manufacturer_info;
//pub use manufacturer_info::ManufacturerInfo;

pub trait Endpoint {

}
