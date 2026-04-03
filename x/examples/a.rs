#![no_std]
#![no_main]

mod common;
use common::esp_log_init;

#[unsafe(no_mangle)]
fn main() {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Note: If you end up using 'esp-idf-svc', also change to using its logging.
    //      -> github.com/finalyards/esp-idf-sample
    esp_log_init();

    log::info!("Hello, world!\n");
}
