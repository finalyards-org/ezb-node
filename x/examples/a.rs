#![no_std]
#![no_main]

extern crate alloc;

mod common;
use common::esp_log_init;   // ties esp-idf-sys logging to 'log'

#[unsafe(no_mangle)]
fn main() {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    esp_log_init();

    log::info!("Hello, world!");
}
