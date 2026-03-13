
// Cannot check ESP-IDF component setup in 'build.rs' (too early; the configs are not reflected
// in the 'CARGO_CFG_...' env.vars, either). But here, we can (and do!):

#[cfg(esp_idf_comp_esp_wifi_enabled)]
compile_error!("ESP_IDF WiFi component found. Check 'sdkconfig.defaults' gets applied!!");

#[cfg(esp_idf_comp_fatfs_enabled)]
compile_error!("ESP_IDF FatFS component found. Check 'sdkconfig.defaults' gets applied!!");

#[cfg(not(esp_idf_comp_riscv_enabled))]
compile_error!("ESP_IDF: well, we should have RISC V enabled!! Check everything!!");
