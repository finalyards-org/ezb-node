use embassy_executor::Spawner;

use esp_idf_svc::{
    log::EspLogger,
    sys::link_patches
};

use embassy_time::{Duration, Timer};

use esp_zb_examples::set_panic_hook;

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

    set_panic_hook();

    EspLogger::initialize_default();

    log::info!("Hello, world!\n");
    todo!();    // testing, what kind of panic message we get (restart-looping?; line number? message?)
                //  - with default (std) panic handler, proper line + message, but seems to go to "abort"
        // <<
        //  I (309) a: Hello, world!
        //
        //  PANIC: panicked at apps/bin/a.rs:35:5:
        //  not yet implemented
        // <<

    let mut tick = true;
    loop {
        log::info!("T{}ck...", if tick {"i"} else {"o"});
        tick = !tick;

        // Use embassy-time timer
        Timer::after(Duration::from_secs(1)).await;
    }
}
