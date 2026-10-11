use esp_idf_svc::nvs::{
    EspDefaultNvsPartition,
    EspCustomNvsPartition,
};
use esp_idf_svc::sys::EspError;

/**
* Init flash filesystem for using with 'esp-zigbee-lib'.
*/
pub fn init_nvs(partition_name: Option<&'static str>) -> Result<(),EspError> {

    // "Take the default NVS partition, initializing it if full or if a new version is detected."
    // Should be like C 'nvs_flash_init()'.
    //
    let a = EspDefaultNvsPartition::take()?;

    // In 'esp-zigbee-sdk' 2.x, examples moved from a named partition to the default ("nvs").
    // Initiate the separate partition, if used.
    //
    let b_opt = match partition_name {
        Some(name) => {
            // Initialize a named partition; should be like C 'nvs_flash_init_partition(name)'.
            let b = EspCustomNvsPartition::take(name)?;
            Some(b)
        },
        _ => None
    };

    // It's important to keep the returned values away from being dropped. This implementation
    // does that by 'Box::leak'. If you want, you can return the values to the caller, instead.
    //
    let _keep = Box::new((a,b_opt));
    Box::leak(_keep);

    Ok(())
}
