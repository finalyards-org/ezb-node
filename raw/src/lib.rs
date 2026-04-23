#![no_std]
#![allow(non_snake_case)]

#[path = "../bindings.rs"]
mod bindings;

#[path = "../stubs.rs"]
mod stubs;

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

// Router; main loop; signal hook
pub use bindings::{
    esp_zb_init,
    esp_zb_cfg_t,
    esp_zb_nwk_device_type_t,
    esp_zb_start,
    esp_zb_cfg_s__bindgen_ty_1,
    esp_zb_zczr_cfg_t,

    esp_zb_app_signal_t,
    esp_zb_app_signal_type_t,
    esp_err_t,
    esp_zb_app_signal_get_params,
};

pub use bindings::{
    esp_zb_stack_main_loop_iteration
};

// signals
pub use bindings::{
    esp_zb_zdo_signal_device_annce_params_t,
    esp_zb_zdo_signal_leave_params_t,
    esp_zb_bdb_signal_touchlink_nwk_started_params_t,
    esp_zb_bdb_signal_touchlink_nwk_joined_router_t,
    esp_zb_nwk_signal_device_associated_params_t,
    esp_zb_zdo_signal_leave_indication_params_t,
    //esp_zb_zgp_signal_commissioning_params_t,
    esp_zb_zdo_signal_can_sleep_params_t,
    esp_zb_zdo_signal_device_authorized_params_t,
    esp_zb_zdo_signal_device_update_params_t,
    esp_zb_zdo_signal_nwk_status_indication_params_t,
    esp_zb_zdo_device_unavailable_params_t,
    //esp_zb_zgp_signal_approve_comm_params_t,
    esp_zb_nwk_leave_type_t,
    esp_zb_get_extended_pan_id,
};

// Node constants and methods (global in C API)
pub use bindings::{
    esp_zb_bdb_commissioning_mode_t,
    //
    esp_zb_bdb_start_top_level_commissioning,
    esp_zb_bdb_is_factory_new,
    esp_zb_get_pan_id,
    esp_zb_get_current_channel,
    esp_zb_get_short_address,

};

