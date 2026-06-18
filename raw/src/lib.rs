#![no_std]
extern crate alloc;

#[path = "../bindings.rs"]
mod bindings;

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
    //esp_zb_nwk_device_type_t,
    esp_zigbee_start,
    esp_zigbee_zczr_config_s,
    //esp_zb_app_signal_t,
    ezb_app_signal_type_t,
    ezb_app_signal_get_params,
    esp_zigbee_launch_mainloop,
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
    ezb_extaddr_t,
    ezb_bdb_comm_status_e,
    ezb_aps_secur_enable_distributed_security,
    ezb_app_signal_add_handler,
};

// Node constants and methods (global in C API)
pub use bindings::{
    ezb_bdb_start_top_level_commissioning,
    ezb_bdb_comm_mode_t,
    ezb_bdb_comm_mode_e,
    ezb_bdb_is_factory_new,
    ezb_nwk_get_panid,
    ezb_nwk_get_current_channel,
    //ezb_factory_reset,
    esp_zigbee_device_config_t,
    ezb_nwk_get_extended_panid,
    ezb_extpanid_t,
    ezb_nwk_get_short_address,
    ezb_app_signal_t,
    ezb_nwk_device_type_t,
    esp_zigbee_config_t,
    ezb_bdb_set_primary_channel_set,
    ezb_bdb_set_secondary_channel_set,
    ezb_app_signal_get_type,
};

// Endpoints
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
    ezb_zha_color_dimmable_light_config_t,
    ezb_af_ep_desc_t,
    ezb_zha_create_color_dimmable_light,
    ezb_zcl_cluster_id_e,
    ezb_zcl_cluster_desc_t,
    ezb_af_endpoint_get_cluster_desc,
    ezb_zcl_basic_cluster_desc_add_attr,
    ezb_zcl_basic_server_attr_t,
    ezb_af_device_add_endpoint_desc,
    ezb_af_device_desc_register,
    ezb_zcl_core_action_handler_register,
    ezb_af_create_device_desc,
    ezb_af_device_desc_t,
    EZB_ZCL_CLUSTER_SERVER,
    EZB_ZCL_CLUSTER_CLIENT,
    //ezb_zcl_core_action_callback_id_t,    // DO NOT expose this (it's u32), use '..._e' instead
    ezb_zcl_core_action_callback_id_e,
    ezb_zcl_status_e,
    //ezb_zcl_status_t,
    ezb_zcl_attribute_s,
    ezb_zcl_cmd_hdr_t,
    ezb_zcl_read_attr_rsp_variable_t,
};

// ZCL Core; callback indications
//
pub use bindings::{
    //ezb_zcl_set_attr_value_message_t,
    esp_zigbee_lock_acquire,
    esp_zigbee_lock_release,
    ezb_zcl_set_attr_value_message_t,
    ezb_zcl_cmd_default_rsp_message_t,
    ezb_zcl_message_info_s,
    // As an exception, allow this interim struct to be passed to 'api' level.
    ezb_zcl_attribute_s__bindgen_ty_1,
    ezb_zcl_attr_type_e,
    ezb_zcl_cmd_hdr_s,
    ezb_address_s,
    ezb_addr_u,
    ezb_addr_mode_e,
    ezb_zcl_core_action_callback_id_t,
    ezb_zcl_write_attr_rsp_variable_s,
    ezb_zcl_cluster_id_t,
    ezb_zcl_cmd_read_attr_rsp_message_t,
    ezb_zcl_cmd_write_attr_rsp_message_t,
    RspVariableEntry,
    RspVariableIter,
    ezb_zcl_read_attr_rsp_variable_s,
    WriteRspVariableEntry,
    WriteRspVariableIter,
    ezb_zcl_attr_type_t,
    ezb_zha_color_dimmer_switch_config_t,
    ezb_zha_create_color_dimmer_switch,
    ezb_zdo_match_desc_req,
    ezb_zdo_match_desc_req_s,
    ezb_zdo_match_desc_req_callback_t,
};
