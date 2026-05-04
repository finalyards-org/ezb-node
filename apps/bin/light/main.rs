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
use esp_zb::{
    node::ChannelMask,
    router::prelude::*,
};

//use hal::peripherals::Peripherals;

mod my_controller;
mod scheduler;

use my_controller::LightController;

mod config;
use config::my_config;

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

    /*** next
    esp_zb_color_dimmable_light_cfg_t light_cfg = ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG();
    esp_zb_ep_list_t *esp_zb_color_dimmable_light_ep = esp_zb_color_dimmable_light_ep_create(HA_COLOR_DIMMABLE_LIGHT_ENDPOINT, &light_cfg);
    zcl_basic_manufacturer_info_t info = {
        .manufacturer_name = ESP_MANUFACTURER_NAME,
        .model_identifier = ESP_MODEL_IDENTIFIER,
    };

    esp_zcl_utility_add_ep_basic_manufacturer_info(esp_zb_color_dimmable_light_ep, HA_COLOR_DIMMABLE_LIGHT_ENDPOINT, &info);
    esp_zb_device_register(esp_zb_color_dimmable_light_ep);
    esp_zb_core_action_handler_register(zb_action_handler);
    esp_zb_set_primary_network_channel_set(ESP_ZB_PRIMARY_CHANNEL_MASK);
    ***/
}
