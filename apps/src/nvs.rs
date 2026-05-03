/*
*/
use esp_idf_svc::sys;

use sys::{
    nvs_flash_init,
    ESP_ERR_NVS_NO_FREE_PAGES,
    ESP_ERR_NVS_NEW_VERSION_FOUND
};

use crate::AppError;

pub fn init_nvs(partition_name: &'static str) -> Result<(),AppError> {
    unsafe {
        let err = nvs_flash_init();

        // Additional logic suggested by
        //
        if err == ESP_ERR_NVS_NO_FREE_PAGES || err == ESP_ERR_NVS_NEW_VERSION_FOUND {
            nvs_flash_erase();
            nvs_flash_init();
        }
    }

    unsafe {
        nvs_flash_init_partition(partition_name)?;
    }

    Ok(())
}
