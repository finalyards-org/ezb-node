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

    // In 'esp-zigbee-sdk' 2.x, examples moved from a named partition to the default ("nvs").
    // Initiate the separate partition, if used.
    //
    let ob = if partition_name != "nvs" {
        warn!("Using a dedicated partition for Zigbee persistence: {}", partition_name);

        // Initialize a named partition; should be like C 'nvs_flash_init_partition(name)'.
        let b = EspCustomNvsPartition::take(partition_name)?;
        Some(b)
    } else {
        None
    };

    Ok((a,ob))
}
