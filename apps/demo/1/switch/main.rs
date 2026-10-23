/*
* Based on:
*   - "Color dimmer switch" example of 'esp-zigbee-sdk' v.2.0
*       -> https://github.com/espressif/esp-zigbee-sdk/tree/main/examples/home_automation_devices/color_dimmer_switch
*/
//r #![feature(never_type)]

mod switch_router;
use switch_router::LightSwitchRouter;

mod button_task;
use button_task::button_task;

use anyhow::*;

use embassy_executor::{
    Spawner,
};

use esp_idf_hal::{
    gpio::{PinDriver, Pull},
    peripherals::Peripherals,
};
use esp_idf_svc::{
    sys::{link_patches},
};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use ezb_node::{
    Config,
};

use ezb_node_apps::{
    init_nvs,
    set_panic_hook,
};
use crate::switch_router::SwitchRouter;

/**
* The entry point.
*
* The main thread runs the application. Part/most of it happens within the '[...]Router' methods,
* which are called within the application (main) thread.
*/
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    // Logging
    {
        // Steer by 'RUST_LOG' env.var. (info is default)
        esp_idf_svc::log::init_from_env();

        log::info!("TESTING 1");
        log::debug!("TESTING 2");
        log::trace!("TESTING 3");
    }

    let peripherals = Peripherals::take().unwrap();
    {
        // Set GPIO9 (BOOT btn) as input (pressed is Low)
        let btn_pin = PinDriver::input(peripherals.pins.gpio9, Pull::Up).unwrap();

        spawner.spawn( button_task(btn_pin).unwrap() );
    }

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
    let zb_node = SwitchRouter::new()
        .unwrap_or_else(|e| panic!("Failed to initialize the Zigbee node: {:?}", e));

    log::debug!("Starting zigbee node");

    zb_node.run_loop() .await;
}
