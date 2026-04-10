//
// Tie 'esp-idf-sys' logging to the 'log' crate.
//
// Usage:
//  <<
//      use common::logging::init as logInit;
//      ...
//      log::info!("abc");
//  <<
//
extern crate alloc;

use esp_idf_svc::sys;

use sys::{
    esp_log_write,
    esp_log_level_t_ESP_LOG_INFO
};
use log::Level;
use alloc::format;

struct EspLogger;

static LOGGER: EspLogger = EspLogger;

pub fn esp_log_init() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info); // tbd. #later level from env
}

impl log::Log for EspLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let msg = format!("{} - {}\0", record.level(), record.args());
        unsafe {
            esp_log_write(
                esp_log_level_t_ESP_LOG_INFO,
                b"APP\0".as_ptr() as *const _,
                msg.as_ptr() as *const _,
            );
        }
    }

    fn flush(&self) {}
}
