
use esp_idf_svc::sys;

use sys::{
    nvs_flash_init,
    nvs_flash_erase,
    ESP_ERR_NVS_NO_FREE_PAGES,
    ESP_ERR_NVS_NEW_VERSION_FOUND
};

// The logic was suggested by Copilot, to replace C 'nvs_flash_init()'.
//
pub fn init_nvs() -> Result<(),crate::Error> {
    unsafe {
        let err = nvs_flash_init();
        if err == ESP_ERR_NVS_NO_FREE_PAGES || err == ESP_ERR_NVS_NEW_VERSION_FOUND {
            nvs_flash_erase();
            nvs_flash_init();
        }
    }
    Ok(())
}
