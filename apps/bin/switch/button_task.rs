//use embassy_executor::task;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::Channel,
};
use embassy_time::{Duration, Timer};

use esp_idf_hal::gpio::{PinDriver, Pull, InputPin};

pub type ButtonChannel = Channel<CriticalSectionRawMutex, ButtonEvent, 4>; // type alias

pub enum ButtonEvent {
    Pressed,
    Depressed,
}

pub static BUTTON_CHANNEL: ButtonChannel = Channel::new();

/**
* Listens to the BOOT button, reflecting its state to 'ButtonChannel'
*/
pub async fn button_task(pin: impl InputPin) {
    let mut btn = PinDriver::input(pin).unwrap();
    btn.set_pull(Pull::Up).unwrap();    // BOOT is active low

    loop {
        loop {
            btn.wait_for_low().await.unwrap();
            Timer::after(Duration::from_millis(50)).await;  // debounce check
            if btn.is_low() { break; }
        }
        BUTTON_CHANNEL.send(ButtonEvent::Pressed).await;

        loop {
            btn.wait_for_high().await.unwrap();
            Timer::after(Duration::from_millis(50)).await; // release debounce check
            if btn.is_high() { break; }
        }
        BUTTON_CHANNEL.send(ButtonEvent::Depressed).await;
    }
}
