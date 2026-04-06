use esp_zb_examples::esp_log_init;

use esp_idf_svc as _;
use esp_idf_svc::sys as esp_idf_sys;

fn main() {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    // Note: If you end up using 'esp-idf-svc', also change to using its logging.
    //      -> github.com/finalyards/esp-idf-sample
    esp_log_init();

    log::info!("Hello, world!\n");
}
