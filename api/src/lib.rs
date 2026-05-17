#![no_std]
#![feature(never_type)]
    // applies to 'node/mod.rs'; needs to be declared here.
extern crate alloc;

mod config_views;
pub use config_views::*;
    // DeviceDescriptorView,
    // PlatformDeviceView,

pub mod node;
pub use node::Node;

mod app_signal;
pub use app_signal::*;

mod error;
pub use error::Error;

pub mod utils;
pub use utils::IeeeAddr;

//mod device_descriptor;
//pub use device_descriptor::*;

// Pass-through the config.
//
// This is essential for the from-TOML-generated snippet digestion, but also useful for the apps in general,
// them not needing to depend on the config crate, directly.
pub use ezb_node_config as config;
pub use ezb_node_config::Config;
