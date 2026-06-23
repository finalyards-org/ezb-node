/*
* Based on:
*   - "Color dimmer switch" example of 'esp-zigbee-sdk' v.2.0
*       -> https://github.com/espressif/esp-zigbee-sdk/tree/main/examples/home_automation_devices/color_dimmer_switch
*/
#![feature(never_type)]
extern crate alloc;

use anyhow::*;

use embassy_executor::{
    Spawner,
};

use esp_idf_svc::{
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

use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};

mod switch_router;
use switch_router::LightSwitchRouter;

mod button_task;
use button_task::button_task;

/**
* The entry point.
*
* The main thread runs the application. Part/most of it happens within the 'LightSwitchRouter' methods, which are
* called within the application (main) thread.
*/
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    // Recommended logging, level steered by 'sdkconfig.defaults'. Guarantees C and Rust sides observe same logging.
    esp_idf_svc::log::EspIdfLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    {
        // Set GPIO9 (BOOT btn) as input (pressed is Low)
        let btn = PinDriver::input(peripherals.pins.gpio9, Pull::Up).unwrap();

        spawner.spawn(button_task(btn));
    }

    // --- Zigbee ---
    //
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

