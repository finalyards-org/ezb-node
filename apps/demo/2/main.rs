use embassy_executor::Spawner;
use esp_idf_svc::{
    nvs::EspDefaultNvsPartition,
    sys::link_patches,
};
//use log::LevelFilter;

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

    esp_idf_svc::log::EspIdfLogger::initialize_default();

    // Separate Rust logging level from C (steered in 'sdkconfig.defaults').
    {
        let lev = match env!("RUST_LOG_LEVEL") {
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info"  => log::LevelFilter::Info,
            "warn"  => log::LevelFilter::Warn,
            "error"|_ => log::LevelFilter::Error,
        };
        log::set_max_level(lev);
    }

    // Initialize the flash. Here we use default "nvs" partition for Zigbee.
    {
        let partition = EspDefaultNvsPartition::take()
            .unwrap_or_else(|e| panic!("Initialization of flash failed: {:?}", e));

        core::mem::forget(partition);
    }

    let zb_node = IasCoordinator::new()
        .unwrap_or_else(|e| panic!("Initialization of Zigbee failed: {:?}", e));

    zb_node.run() .await;
}
