#![no_std]
#![allow(non_snake_case)]

#[path = "../bindings_tail.rs"]
mod bindings;

pub use bindings::{
    ESP_ZB_VER_MAJOR,
    ESP_ZB_VER_MINOR,
    ESP_ZB_VER_PATCH,
};

// Rust note: converting '[u8;_]' to a const string requires 'unsafe': we confirm the contents are valid UTF8.
pub const ESP_ZB_VER: &str = unsafe {
    core::str::from_utf8_unchecked(bindings::ESP_ZB_VER_STR)
}; // "1.6.8"

// We selectively choose the elements that make it to the API layer.
//
// Could also filter in the 'bindgen' stage, but this turns out to be convenient, in practice.
//
pub use bindings::{
    esp_zb_platform_config,
    esp_zb_platform_config_t,
        // {
        //    radio_config: {
        //      radio_mode:         ZB_RADIO_MODE_NATIVE | ZB_RADIO_MODE_UART_RCP,
        //      radio_uart_config: {
        //        port:  UART_NUM_0 ... UART_NUM_MAX (3)
        //        rx_pin:
        //        tx_pin:
        //        uart_config: { ... }
        //    },
        //    host_config: {
        //      host_connection_mode: ZB_HOST_CONNECTION_MODE_NONE | ... _MODE_CLI_UART | ... _MODE_RCP_UART
        //      host_uart_config: { port, rx_pin, tx_pin, uart_config }
        //    }
        // }

    esp_zb_radio_mode_t,
    esp_zb_host_connection_mode_t,
    esp_zb_uart_config_t,
    esp_zb_host_config_t,
    esp_zb_radio_config_t,
    uart_port_t,
    gpio_num_t
};

// Router; main loop
pub use bindings::{
    esp_zb_init,
    esp_zb_cfg_t,
    esp_zb_nwk_device_type_t,
    esp_zb_start,
    esp_zb_cfg_s__bindgen_ty_1,
    esp_zb_zczr_cfg_t
};

pub use bindings::{
    esp_zb_stack_main_loop_iteration
};

