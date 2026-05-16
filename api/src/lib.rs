#![no_std]
#![feature(never_type)]
    // applies to 'node/mod.rs'; needs to be declared here.
extern crate alloc;

mod config_views;

pub mod node;

mod app_signal;
pub use app_signal::*;

mod error;
pub use error::Error;

pub mod utils;
pub use utils::IeeeAddr;

mod device_descriptor;
pub use device_descriptor::*;

// Apps might like getting this.
pub use ezb_node_config::Config;
