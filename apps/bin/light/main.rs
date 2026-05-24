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
use crate::light_controller::LightController;

//use hal::peripherals::Peripherals;

mod light_controller;
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

    // Initialize and provide error message
    //
    let _keep: (_,_);
    let init_res: Result<LightController,ezb_node::Error> = async {
        _keep = init_nvs(crate::CONFIG.storage_partition_name)?;

        let lc = LightController::new(&CONFIG)?;
        Ok(lc)
    };

    let lc = init_res.unwrap_or_else(|e| {
        panic!("Initialization failed: {:?}", e);
    });

    lc.spawn();

    // 'zb_task' listens to the radio; will feed 'LightController' methods events.
    // Listen to them.
    loop {
        lc.tick() .await
    }
}
