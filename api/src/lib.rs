#![no_std]
extern crate alloc;
pub(crate) use esp_zb_raw as raw;   // internal 'crate::raw'

mod config;
pub use config::*;

pub mod node;

mod app_signal;
pub use app_signal::*;

mod version;
pub use version::*;

mod error;
pub use error::Error;

pub mod utils;
pub use utils::IeeeAddr;

mod device_descriptor;
pub use device_descriptor::*;

// Prelude-like experience with 'use esp_zb::router::*'
//
pub mod router {
    pub mod prelude {
        pub use crate::{
            node::Router,
            node::Node,     // so trait methods show up
        };
    }
}
