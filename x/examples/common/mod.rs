
mod logging;
pub use logging::esp_log_init;

mod nvs;
pub use nvs::init_nvs;

// Panic
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // ROM 'print' is slightly safer to use than 'log::error!'; both should work.
    {
        log::error!("*** PANIC ***\n{}", info);
    }
    #[cfg(false)]
    {
        print("*** PANIC ***");
        print("{}", info);
    }

    // Do not reboot
    loop {
        unsafe { esp_idf_sys::vTaskDelay(1000) };
    }
}

#[cfg(false)]
fn print(s: &str) {
    unsafe {
        esp_idf_sys::esp_rom_printf("%s\0".as_ptr() as _, s.as_ptr());
    }
}
