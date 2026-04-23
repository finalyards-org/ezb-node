#![no_std]
#![allow(non_snake_case)]

pub(crate) use esp_zb_raw as raw;

pub mod node;

// Next (current) module being worked upon; rename and take next.
//mod _wip;
//pub use _wip::*;

mod signal;
pub use signal::*;

mod version;
pub use version::*;

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
