#![no_std]
#![allow(non_snake_case)]

#[path = "../tmp/bindings.rs"]
mod bindings;

pub use bindings::{
    ESP_ZB_VER_MAJOR,
    ESP_ZB_VER_MINOR,
    ESP_ZB_VER_PATCH,
};

// Rust note: converting '[u8;_]' to a const string requires 'unsafe': we confirm the contents are valid UTF8.
pub const ESP_ZB_VER: &str = unsafe {
    core::str::from_utf8_unchecked(bindings::ESP_ZB_VER_STR)
}; // "1.6.8"

//#[cfg(feature="zcl_alarms")]
// todo
