use embassy_executor::Spawner;
use esp_idf_hal::sys::EspError;
use esp_idf_svc::{
    nvs::EspDefaultNvsPartition,
    sys::link_patches,
};
//use log::LevelFilter;
use log::debug;

use ezb_node_apps::{
    set_panic_hook,
};

mod ias_coordinator;
use ias_coordinator::IasCoordinator;

/**
* The entry point.
*
* The main thread runs the application. Part/most of it happens within the '[...]Coordinator' methods,
* which are called within the application (main) thread.
*/
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    // Recommended logging, level steered by 'sdkconfig.defaults'. Guarantees C and Rust sides observe same logging.
    esp_idf_svc::log::EspIdfLogger::initialize_default();

    let () = init_nvs()
        .unwrap_or_else(|e| panic!("Initialization of flash failed: {:?}", e));

    let zb_node = IasCoordinator::new()
        .unwrap_or_else(|e| panic!("Initialization of Zigbee failed: {:?}", e));

    zb_node.run() .await;
}

/**
* Init flash filesystem for using with 'esp-zigbee-lib'.
*/
// tbd. once this works, consider integrating above
pub fn init_nvs() -> Result<(),EspError> {

    // "Take the default NVS partition, initializing it if full or if a new version is detected."
    // Should be like C 'nvs_flash_init()'.
    //
    let a = EspDefaultNvsPartition::take()?;

    //r?
    // In 'esp-zigbee-sdk' 2.x, examples moved from a named partition to the default ("nvs").
    // Initiate the separate partition, if used.
    //
    #[cfg(false)]
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
    let _keep = Box::new(a);
    Box::leak(_keep);

    Ok(())
}
