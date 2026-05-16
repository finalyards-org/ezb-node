/*
*/
use esp_idf_svc::sys;

// These were _not_ available via 'esp_idf_svc::sys::'; don't know why. :o
//  <<
//    use esp_idf_svc::sys::{
//        nvs_flash_init,
//        nvs_flash_init_partition,
//        ESP_ERR_NVS_NO_FREE_PAGES,
//        ESP_ERR_NVS_NEW_VERSION_FOUND
//    };
//  <<
use ezb_node::sys::{
    nvs_flash_init,
    nvs_flash_init_partition,
    ESP_ERR_NVS_NO_FREE_PAGES,
    ESP_ERR_NVS_NEW_VERSION_FOUND
};

use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvsPartition};
use esp_idf_svc::sys::EspError;

/**
* Init flash filesystem for using with 'esp-zigbee-lib'.
*/
// Has two implementations: one for C API (1:1); another for 'esp-idf-svc'. Try both and decide. tbd.
pub fn init_nvs(partition_name: &'static str) -> Result<(),EspError> {

    #[cfg(true)]
    {
        let err = unsafe { nvs_flash_init() };
        check(err)?;

        let err = unsafe { nvs_flash_init_partition(partition_name.as_ptr()) };
        check(err)?;

        Ok(())
    }

    // Note:
    //      pin the return values to something, so that the partitions are guaranteed to stay good.
    #[cfg(false)]
    {
        // "Take the default NVS partition, initializing it if full or if a new version is detected."
        //
        // Hopefully close enough to the C: 'nvs_flash_init()'.
        //
        let _ = EspDefaultNvsPartition::take()?;

        // Initialize a named partition; should be like C 'nvs_flash_init_partition(name)'.
        let _ = EspNvsPartition::take(partition_name)?;
    }
}

fn check(err: i32) -> Result<(), EspError> {
    if err == 0 { Ok(()) } else { Err(EspError::from(err).unwrap()) }
}
