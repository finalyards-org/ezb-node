#![no_std]
//#![allow(non_snake_case)]

pub(crate) use esp_zb_raw as raw;

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

// We'll expose these _for now_ but the idea is 'raw' should not (need to) be visible in the API.
pub use raw::{
    esp_zb_bdb_commissioning_mode_t as bdb_commissioning_mode,
};

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
