#![no_std]
#![allow(non_snake_case)]

pub(crate) use esp_zb_raw as raw;

// Next (current) module being worked upon; rename and take next.
mod _wip;
pub use _wip::*;

mod platform;
pub use platform::*;

mod version;
pub use version::*;

pub async fn process() {
    unsafe {
        raw::esp_zb_stack_main_loop_iteration();
    }
};
