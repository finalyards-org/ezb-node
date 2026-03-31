#![no_std]
#![no_main]

extern crate alloc;

mod common;
use common::esp_log_init;
use esp_zb_raw::ESP_ZB_VER;
// ties esp-idf-sys logging to 'log'

#[unsafe(no_mangle)]
fn main() {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Note: If you end up using 'esp-idf-svc', also change to using its logging.
    //      -> github.com/finalyards/esp-idf-sample
    esp_log_init();

    let ver = ESP_ZB_VER;

    log::info!("Hello, world! esp_zigbee_sdk: {}\n", ver);
}
