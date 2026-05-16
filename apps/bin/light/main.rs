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
};

use ezb_node_apps::{
    init_nvs,
    set_panic_hook,
    AppError,
};
use crate::light_controller::LightController;
//r use ezb_node::ChannelMask;

//use hal::peripherals::Peripherals;

mod light_controller;
mod scheduler;

//use my_controller::LightController;

/**
* The entry point.
*/
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    log_init(LevelFilter::Debug);    // or '::init_from_env()' and 'RUST_LOG'

    let config =
        include!("light_conf.in");

    main2(config).await .unwrap_or_else(|e| {
        panic!("Fatal error: {:?}", e);
    });
}

/**
* An inner 'main()' that may fail its initialization.
*/
async fn main2(c: Config) -> Result<!, AppError> {
    //#later let _ = Peripherals::take()?;

    init_nvs(c.storage_partition_name)?;

    // Initialize Zigbee
    //
    let node = LightController::new(&c);
        // tbd. view that automatically kicks in

    node.add_endpoints(&c);

    let _ = LightController::new()?
        //
        .roll(false) .await;
}
