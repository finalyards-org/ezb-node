//R use esp_zb_examples::esp_log_init;

use embassy_executor::Spawner;

use esp_idf_svc::{
    log::EspLogger,
    sys::link_patches
};

use embassy_time::{Duration, Timer};

// Note: Could use background tasks
//  <<
//    #[embassy_executor::task]
//    async fn blinky_task() {
//        loop {
//            println!("Blink!");
//            Timer::after(Duration::from_millis(500)).await;
//        }
//    }
//    ...
//    spawner.spawn(blinky_task()).unwrap();
//  <<

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // 'esp-idf-sys' needs it. See https://github.com/esp-rs/esp-idf-template/issues/71
    link_patches();

    EspLogger::initialize_default();

    //? embassy_time_driver_init();

    // Note: If you end up using 'esp-idf-svc', also change to using its logging.
    //      -> github.com/finalyards/esp-idf-sample
    #[cfg(false)]
    esp_log_init();

    log::info!("Hello, world!\n");
    todo!();

    let mut tick = true;
    loop {
        log::info!("T{}ck...", if tick {"i"} else {"o"});
        tick = !tick;

        // Use embassy-time timer
        Timer::after(Duration::from_secs(1)).await;
    }
}
