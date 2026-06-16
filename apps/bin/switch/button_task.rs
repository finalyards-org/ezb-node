use embassy_executor::task;
use embassy_time::{Duration, Timer};
use esp_idf_hal::gpio::{PinDriver, Pull, InputPin};

#[task]
async fn button_task(
    pin: impl InputPin,
    task_channel: &'static embassy_sync::channel::Channel<
        embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex,
        light::scheduler::ScheduledTask,
        4
    >
) {
    // Alustetaan asynkroninen input-ajuri
    let mut button = PinDriver::input(pin).unwrap();
    button.set_pull(Pull::Up).unwrap(); // BOOT-nappi vetää yleensä maihin

    loop {
        // 1. Odotetaan asynkronisesti, että linja laskee alas (painallus alkaa)
        // Tämä EI blokkaa CPU:ta, vaan antaa muiden taskien ajaa vapaasti
        button.wait_for_low().await.unwrap();

        // 2. Debounce: Odotetaan 50ms kontaktihäiriöiden ylipääsemiseksi
        Timer::after(Duration::from_millis(50)).await;

        // Varmistetaan, että nappi on edelleen pohjassa, eikä kyseessä ollut kohina
        if button.is_low() {
            log::info!("BOOT-nappia painettu (asynkronisesti)!");

            // 3. Lähetetään komento eteenpäin siihen aiemmin korjaamaamme Embassy-kanavaan
            task_channel.send(light::scheduler::ScheduledTask::ToggleLight).await;
        }

        // 4. Odotetaan, että nappi vapautetaan ennen seuraavaa kierrosta
        button.wait_for_high().await.unwrap();
        Timer::after(Duration::from_millis(50)).await; // Vapautuksen debounce
    }
}
