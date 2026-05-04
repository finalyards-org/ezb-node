#![no_std]
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
    ezb_app_signal_get_params,
    //R ezb_stack_main_loop_iteration,   // 2.0: "deprecated"; RATHER DISCONTINUED: expands to a no-op; Q: How to get involved in the main loop, in 2.0???
};

// signals
pub use bindings::{
    ezb_zdo_signal_device_annce_params_t,
    ezb_zdo_signal_leave_params_t,
    //R ezb_nwk_signal_device_associated_params_t,
    ezb_zdo_signal_leave_indication_params_t,
    //R ezb_zdo_signal_can_sleep_params_t,
    ezb_zdo_signal_device_authorized_params_t,
    ezb_zdo_signal_device_update_params_t,
    //R ezb_zdo_signal_nwk_status_indication_params_t,
    //R ezb_nwk_leave_type_t,
    ezb_zdo_signal_device_unavailable_params_t,
    ezb_bdb_signal_simple_params_t,
    ezb_nwk_signal_network_status_params_t,
    ezb_nwk_network_status_t,
    ezb_nwk_signal_permit_join_status_params_t,
    ezb_app_signal_type_e,
};

// Node constants and methods (global in C API)
pub use bindings::{
    ezb_bdb_start_top_level_commissioning,
    ezb_bdb_comm_mode_t,
    ezb_bdb_is_factory_new,
    ezb_nwk_get_panid,
    ezb_get_current_channel,
    //ezb_factory_reset,
    //ezb_set_primary_network_channel_set,
    esp_zigbee_device_config_t,
    ezb_nwk_get_extended_panid,
    ezb_extpanid_t,
    ezb_nwk_get_short_address,
    ezb_nwk_get_current_channel,
    ezb_app_signal_t,
};

// Endpoints
#[cfg(false)]
pub use bindings::{
    //ezb_zcl_basic_attr_t,
    //ezb_ep_list_t,
    //ezb_device_register,
    //ezb_color_dimmable_light_cfg_t,
    //esp_zb_color_dimmable_switch_cfg_t,
    //ezb_color_dimmable_light_ep_create,
    //ezb_ep_list_create,
    //ezb_ep_list_add_ep,
    //ezb_cluster_list_t,
    //ezb_endpoint_config_t,
    //ezb_color_dimmable_light_clusters_create,
    //ezb_af_profile_id_t::{
    //    self,
    //    EZB_AF_HA_PROFILE_ID,
    //},
    //ezb_ha_standard_devices_t::{
    //    self,
    //    ESP_ZB_HA_COLOR_DIMMABLE_LIGHT_DEVICE_ID
    //},
};

// NVS access
pub use bindings::sys::*;
