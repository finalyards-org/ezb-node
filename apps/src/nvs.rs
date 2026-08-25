use esp_idf_svc::nvs::{
    EspDefaultNvsPartition,
    EspCustomNvsPartition,
};
use esp_idf_svc::sys::EspError;

use log::warn;

/**
* Init flash filesystem for using with 'esp-zigbee-lib'.
*
* @note The caller is expected to keep the return value alive.
*/
pub fn init_nvs(partition_name: &'static str) -> Result<(EspDefaultNvsPartition,Option<EspCustomNvsPartition>),EspError> {

    // "Take the default NVS partition, initializing it if full or if a new version is detected."
    // Should be like C 'nvs_flash_init()'.
    //
    let a = EspDefaultNvsPartition::take()?;

    // In 2.x, 'esp-zigbee-sdk' examples moved from a named partition to the default ("nvs").
    // This means the partition would not be needed to be initialized, any more.
    //
    let b = if partition_name == "nvs" {
        warn!("Still using a separate partition name: {}", partition_name);

        // Initialize a named partition; should be like C 'nvs_flash_init_partition(name)'.
        let b = EspCustomNvsPartition::take(partition_name)?;

        Some(b)
    } else {
        None
    };

    Ok((a,b))
}
