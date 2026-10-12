//?? #![feature(never_type)]
    // applies to 'node/mod.rs'; needs to be declared here.

mod config_views;
pub use config_views::*;
    // DeviceDescriptorView,
    // PlatformDeviceView,

mod node;
pub use node::*;

mod app_signal;
pub use app_signal::*;

mod error;
pub use error::*;

pub mod utils;
pub use utils::{
    IeeeAddr,
    ShortAddr,
};

mod types;
pub use types::*;

// Pass-through the config.
//
// Essential for the from-TOML-generated code.
pub use ezb_node_config as config;
use ezb_node_config::Config;    // just for own consumption, this way

mod zcl_event;
pub use zcl_event::*;
