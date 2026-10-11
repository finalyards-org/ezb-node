use embassy_executor::Spawner;

use esp_idf_svc::{
    sys::{link_patches},
};

//use log::LevelFilter;
use log::debug;

use ezb_node::{
    Config,
};

use ezb_node_apps::{
    init_nvs,
    set_panic_hook,
};

mod ias_coordinator;
use ias_coordinator::IasCoordinator;

#[path = "./tmp/2-config.rs"]
mod cfg;
use cfg::Config;

/**
* The entry point.
*
* The main thread runs the application. Part/most of it happens within the '[...]Coordinator' methods,
* which are called within the application (main) thread.
*/
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    set_panic_hook();

    // Recommended logging, level steered by 'sdkconfig.defaults'. Guarantees C and Rust sides observe same logging.
    esp_idf_svc::log::EspIdfLogger::initialize_default();

    //r static CFG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| {
    //r     include!(concat!(env!("OUT_DIR"), "/2-door_conf.in"))
    //r });

    let zb_node = {
        let () = init_nvs(&Config.storage_partition_name)
            .unwrap_or_else(|e| panic!("Initialization of flash failed: {:?}", e));

        IasCoordinator::new(&Config)
            .unwrap_or_else(|e| panic!("Initialization of Zigbee failed: {:?}", e))
    };

    zb_node.run() .await;
}
