/*
* Based on:
*   - "Color dimmable light" example of 'esp-zigbee-sdk' v.2.0
*       -> https://github.com/espressif/esp-zigbee-sdk/tree/main/examples/home_automation_devices/color_dimmable_light
*/
use embassy_executor::Spawner;

use esp_idf_svc::{
    nvs::EspDefaultNvsPartition,
    sys::link_patches,
};

use ezb_node::Node;

use ezb_node_apps::{
    set_panic_hook,
};

mod light_coordinator;
use light_coordinator::LightCoordinator;

/**
* The entry point.
*
* The main thread runs the application. Part/most of it happens within the '[...]Coordinator'
* methods, which are called within the application (main) thread.
*/
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    // Use for tying log levels to ESP-IDF ('CONFIG_LOG_DEFAULT_LEVEL')
    esp_idf_svc::log::EspIdfLogger::initialize_default();

    // Initialize the flash.
    {
        let part = EspDefaultNvsPartition::take()
            .unwrap_or_else(|e| panic!("Initialization of flash failed: {:?}", e));

        core::mem::forget(part);
    }

    spawner.spawn( node_task() .unwrap_or_else(|e| panic!("spawn failed: {:?}", e)) );

    // Processing happens in the background.
    loop {
        embassy_time::Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn node_task() -> ! {
    let zb_node = LightCoordinator::new()
        .unwrap_or_else(|e| panic!("Failed to initialize the Zigbee node: {:?}", e));

    log::debug!("Starting zigbee node");

    zb_node.run_loop() .await;
}
