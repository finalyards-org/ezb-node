/*
* Based on:
*   - "Color dimmer switch" example of 'esp-zigbee-sdk' v.2.0
*       -> https://github.com/espressif/esp-zigbee-sdk/tree/main/examples/home_automation_devices/color_dimmer_switch
*/
#![feature(never_type)]
extern crate alloc;

use anyhow::*;

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
};
use crate::router::LightSwitchRouter;

//use hal::peripherals::Peripherals;

mod router;

/**
* The entry point.
*
* The main thread runs the application. Part/most of it happens within the 'LightSwitchRouter' methods, which are
* called within the application (main) thread.
*/
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    log_init(LevelFilter::Debug);    // or '::init_from_env()' and 'RUST_LOG'

    static CFG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| {
        include!(concat!(env!("OUT_DIR"), "/switch_conf.in"))
    });

    let (_keep, lc) = (|| -> anyhow::Result<(_,LightSwitchRouter)> {    // Rust note: scope the '?' by an anonymous closure
        let nvs_res = init_nvs(CFG.storage_partition_name)
            .context("Failed to initialize NVS")?;

        let tmp = LightSwitchRouter::new(&CFG)
            .context("Failed to initialize the Zigbee node")?;

        Ok((nvs_res, tmp))
    })()
    .unwrap_or_else(|e| {
        panic!("Initialization failed: {:?}", e);
    });

    lc.run() .await;
}

