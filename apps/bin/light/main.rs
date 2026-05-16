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

use esp_zb_apps::{
    init_nvs,
    set_panic_hook,
    AppError,
};

use log::LevelFilter;
use ezb_node::{
    node::ChannelMask,
    //router::prelude::*,
};

//use hal::peripherals::Peripherals;

mod my_controller;
mod scheduler;

use my_controller::LightController;

mod my_config_temp;
use my_config_temp::my_config;

/**
* The entry point.
*/
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    log_init(LevelFilter::Debug);    // or '::init_from_env()' and 'RUST_LOG'

    let _ = main2().await .map_err(async |e| {
        panic!("Fatal error: {:?}", e);
    });
}

/**
* An inner 'main()' that may fail its initialization.
*/
async fn main2() -> Result<!, AppError> {
    //#later let _ = Peripherals::take()?;

    let c = my_config();

    init_nvs(c.storage_partition_name.as_str())?;

    // Initialize Zigbee
    //
    let _ = LightController::new()?
        //R .core_action_handler_register()
        //
        .roll(false) .await;
}
