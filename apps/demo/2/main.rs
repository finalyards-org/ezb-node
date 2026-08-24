//? #![feature(never_type)]
extern crate alloc;

use anyhow::*;

use embassy_executor::Spawner;

use esp_idf_svc::{
    sys::{link_patches},
};

//use log::LevelFilter;

use ezb_node::{
    Config,
    Node,
};

use ezb_node_apps::{
    init_nvs,
    set_panic_hook,
};

mod ias_coordinator;
use ias_coordinator::IasCoordinator;

/**
* The entry point.
*
* The main thread runs the application. Part/most of it happens within the 'MyController' methods,
* which are called within the application (main) thread.
*/
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    // Recommended logging, level steered by 'sdkconfig.defaults'. Guarantees C and Rust sides observe same logging.
    esp_idf_svc::log::EspIdfLogger::initialize_default();

    static CFG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| {
        include!(concat!(env!("OUT_DIR"), "/2-door_conf.in"))
    });

    let (_keep, lc) = (|| -> anyhow::Result<(_,IasCoordinator)> {    // scope the '?' by an anonymous closure
        let nvs_res = init_nvs(CFG.storage_partition_name)
            .context("Failed to initialize NVS")?;

        let tmp = IasCoordinator::new(&CFG)
            .context("Failed to initialize the Zigbee node")?;

        Ok((nvs_res, tmp))
    })()
    .unwrap_or_else(|e| {
        panic!("Initialization failed: {:?}", e);
    });

    lc.run() .await;
}
