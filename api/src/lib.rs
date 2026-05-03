#![no_std]
extern crate alloc;
pub(crate) use esp_zb_raw as raw; // protect elsewhere within 'api' from naming changes

mod config;
pub use config::*;

pub mod node;

mod signal;
pub use signal::*;

mod version;
pub use version::*;

mod error;
pub use error::Error;

pub mod endpoint;
//pub use endpoint::{
//    Endpoint,
//};

pub mod utils;

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
