#[cfg(not(esp_idf_version_at_least_5_5_3))]
compile_error!("Meant to use ESP_IDF 5.5");

esp_idf_svc::sys::esp_app_desc! {}

fn main() {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // For logging options, see -> https://github.com/esp-rs/esp-idf-svc/blob/master/examples/logging.rs
    //
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::init_from_esp_idf();

    log::info!("Hello, world!");
}
