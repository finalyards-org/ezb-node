
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    channel::Channel,
};
use embassy_sync::channel::DynamicReceiver;
use embassy_time::{Duration, Timer};

use esp_idf_hal::gpio::{PinDriver, Input};

pub enum ButtonEvent {
    Pressed,
    Depressed,
}

static BUTTON_CHANNEL: Channel<NoopRawMutex, ButtonEvent, 4> = Channel::new();

pub fn button_rx() -> DynamicReceiver<ButtonEvent> {
    BUTTON_CHANNEL.dyn_receiver()
}

/**
* Listens to the BOOT button, reflecting its state to 'BUTTON_CHANNEL'
*/
pub async fn button_task(mut btn: PinDriver<Input>) {

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
