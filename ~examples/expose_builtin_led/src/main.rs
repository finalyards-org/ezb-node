#[cfg(any(
  not(esp_idf_version_at_least_5_4_0),
  esp_idf_version_patch_at_most_5_5_3
))]
compile_error!("Meant for ESP_IDF 5.4 .. 5.5");

esp_idf_sys::esp_app_desc! {}

fn main() {
    // Call this function once. Otherwise, some patches to the runtime implemented by 'esp-idf-sys'
    // might not link properly.
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    //
    esp_idf_svc::sys::link_patches();

    // For logging options, see -> https://github.com/esp-rs/esp-idf-svc/blob/master/examples/logging.rs
    //
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::init_from_esp_idf();

    log::info!("Hello, world!");
}
