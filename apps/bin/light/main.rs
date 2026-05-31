/*
* Based on:
*   - "Color dimmable light" example of 'esp-zigbee-sdk' v.2.0
*       -> https://github.com/espressif/esp-zigbee-sdk/tree/main/examples/home_automation_devices/color_dimmable_light
*/
#![feature(never_type)]
extern crate alloc;

use embassy_executor::Spawner;

use esp_idf_svc::{
    log::{init as log_init},
    sys::{link_patches},
    //hal,
};

use log::LevelFilter;

use ezb_node::{
    Config,
    Node,
};

use ezb_node_apps::{
    init_nvs,
    set_panic_hook,
    AppError,
};
use crate::light_coordinator::LightCoordinator;

//use hal::peripherals::Peripherals;

mod light_coordinator;
mod scheduler;

//use my_controller::LightController;

/**
* The entry point.
*
* The main thread runs the application. Part/most of it happens within the 'LightController'
* methods, which are called within the application (main) thread.
*/
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    log_init(LevelFilter::Debug);    // or '::init_from_env()' and 'RUST_LOG'

    const CONFIG: &Config = include!(concat!(env!("OUT_DIR"), "/light_conf.in"));

    // Initialize and provide error message
    //
    let _keep: (_,_);

    let lc: LightCoordinator = (|| {    // Rust note: scope the '?' by an anonymous closure
        _keep = init_nvs(CONFIG.storage_partition_name)?;
        let tmp = LightCoordinator::new(CONFIG)?;
            // The Zigbee task is now running (will be, at least..)

        Ok(tmp)
    })()
    .unwrap_or_else(|e: ezb_node::Error| {
        panic!("Initialization failed: {:?}", e);
    });

    lc.run() .await;
}
