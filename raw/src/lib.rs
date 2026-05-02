#![no_std]
#![allow(non_snake_case)]
extern crate alloc;

#[path = "../bindings.rs"]
mod bindings;

#[path = "../stubs.rs"]
mod stubs;

pub use bindings::{
    esp_zigbee_get_version_string,
};

// We selectively choose the elements that make it to the API layer.
//
// Could also filter in the 'bindgen' stage, but this turns out to be convenient, in practice.
//
pub use bindings::{
    //R esp_zb_platform_config,
    esp_zigbee_platform_config_t,   // 1.x: esp_zb_platform_config_t,
        // {
        //    radio_config: {
        //      radio_mode:         ZB_RADIO_MODE_NATIVE | ZB_RADIO_MODE_UART_RCP,
        //      radio_uart_config: {
        //        port:  UART_NUM_0 ... UART_NUM_MAX (3)
        //        rx_pin:
        //        tx_pin:
        //        uart_config: { ... }
        //    }
        // }

    esp_zigbee_radio_mode_t,
    //R esp_zb_host_connection_mode_t,
    //r esp_zb_uart_config_t,
    //r esp_zb_host_config_t,
    //r esp_zb_radio_config_t,
    //r uart_port_t,
    //r gpio_num_t
};

// Router; main loop; signal hook
pub use bindings::{
    esp_zigbee_init,
    //esp_zb_cfg_t,
    //esp_zb_nwk_device_type_t,
    esp_zigbee_start,
    //esp_zb_cfg_s__bindgen_ty_1,
    esp_zigbee_zczr_config_s,

    //esp_zb_app_signal_t,
    ezb_app_signal_type_t,
    esp_err_t,
    //esp_zb_app_signal_get_params,
    //esp_zb_stack_main_loop_iteration,   // 2.0: "deprecated"; RATHER DISCONTINUED: expands to a no-op; Q: How to get involved in the main loop, in 2.0???
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
    esp_zb_get_extended_pan_id,
    esp_zb_factory_reset,
    esp_zb_set_primary_network_channel_set,
};

// Endpoints
pub use bindings::{
    esp_zb_zcl_basic_attr_t,
    esp_zb_ep_list_t,
    esp_zb_device_register,
    esp_zb_color_dimmable_light_cfg_t,
    //esp_zb_color_dimmable_switch_cfg_t,
    esp_zb_color_dimmable_light_ep_create,
    esp_zb_ep_list_create,
    esp_zb_ep_list_add_ep,
    esp_zb_cluster_list_t,
    esp_zb_endpoint_config_t,
    esp_zb_color_dimmable_light_clusters_create,
    esp_zb_af_profile_id_t::{
        self,
        ESP_ZB_AF_HA_PROFILE_ID
    },
    esp_zb_ha_standard_devices_t::{
        self,
        ESP_ZB_HA_COLOR_DIMMABLE_LIGHT_DEVICE_ID
    },
};
