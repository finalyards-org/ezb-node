/*
* Gathering the bindgen-generated binding like this allows us to attach
* 'Default' (and/or other traits) to its types.
*/
#[allow(non_camel_case_types)]
#[allow(unused)]
    // Disable warnings of unused entries, and imports in 'tmp/bindings_0.rs'.

#[allow(unsafe_op_in_unsafe_fn)]
    // 'bindgen' (0.72.1) generates code that has 'unsafe fn' but not using 'unsafe within the body; this seems to be a problem.
    //  <<
    //      #[inline]
    //      pub unsafe fn as_slice(&self, len: usize) -> &[T] {
    //          // <-- no 'unsafe {' here
    //          ::core::slice::from_raw_parts(self.as_ptr(), len)
    //      }
    //  <<

// Silence:
//  <<
//warning: unnecessary transmute
//     --> raw/src/../tmp/bindings_0.rs:2006:48
//      |
// 2006 |             let router_capacity: u8 = unsafe { ::core::mem::transmute(router_capacity) };
//      |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//      |
// help: replace this with
//      |
// 2006 -             let router_capacity: u8 = unsafe { ::core::mem::transmute(router_capacity) };
// 2006 +             let router_capacity: u8 = unsafe { u8::from(router_capacity) };
//  <<
//
#[allow(unnecessary_transmutes)]
mod a {
    include!("tmp/bindings_0.rs");
}
pub use a::*;

use core::mem::MaybeUninit;

// NVS
//
pub mod sys {
    pub use super::{
        nvs_flash_init,
        nvs_flash_init_partition,
    };
}

impl ezb_extpanid_t {
    /**
    * Provide an empty struct, e.g. to be used as a buffer.
    */
    pub fn empty() -> Self {
        let un = MaybeUninit::zeroed();
        unsafe { un.assume_init() }
    }
}

/*** 1.x
const STORAGE_PARTITION_NAME: &str = "zb_storage";    // from 'partitions.csv'; tbd. maybe bring in from TOML

// typedef struct esp_zigbee_platform_config_s {
//     const char *storage_partition_name;     /*!< The name of the storage partition */
//     esp_zigbee_radio_config_t radio_config; /*!< The radio configuration */
// } esp_zigbee_platform_config_t;
//
//typedef struct esp_zigbee_radio_config_s {
//     esp_zigbee_radio_mode_t  radio_mode;            /*!< The radio mode */
//     union {
//         esp_zigbee_uart_config_t radio_uart_config; /*!< The uart configuration to RCP */
//     };
// } esp_zigbee_radio_config_t;
//
impl Default for esp_zigbee_platform_config_t {
    fn default() -> Self {
        esp_zigbee_platform_config_t {
            storage_partition_name: STORAGE_PARTITION_NAME.as_ptr(),
            radio_config: esp_zigbee_radio_config_t {
                radio_mode: esp_zigbee_radio_mode_t::ESP_ZIGBEE_RADIO_MODE_NATIVE,
                __bindgen_anon_1: unsafe { core::mem::zeroed() } // not used
            }
        }
    }
}
***/

//R
#[cfg(false)]
impl esp_zb_uart_config_t {
    fn nada() -> Self {
        // Not useful setting the random fields; for the tail two we don't even know how to.
        let un = MaybeUninit::zeroed();
        unsafe{ un.assume_init() }

        /***
        uart_config_t {
            baud_rate: 0,
            data_bits: uart_word_length_t::UART_DATA_8_BITS,
            parity: uart_parity_t::UART_PARITY_DISABLE,
            stop_bits: uart_stop_bits_t::UART_STOP_BITS_1,
            flow_ctrl: uart_hw_flowcontrol_t::UART_HW_FLOWCTRL_DISABLE,
            rx_flow_ctrl_thresh: 0,
            __bindgen_anon_1: {
                let un = MaybeUninit::zeroed();
                unsafe{ un.assume_init() }
            },
            flags: {
                let un = MaybeUninit::zeroed();
                unsafe{ un.assume_init() }
            }
            __bindgen_anon_1: uart_config_t__bindgen_ty_1 {
                source_clk: uart_sclk_t::UART_SCLK_DEFAULT
            },
            flags: uart_config_t__bindgen_ty_2 {
                _bitfield_align_1: [],
                _bitfield_1: [],
                __bindgen_padding_0: []
            }
        }***/
    }
}

//R
//  typedef struct {
//      esp_zb_host_connection_mode_t   host_connection_mode;   /*!< The host connection mode */
//      esp_zb_uart_config_t            host_uart_config;       /*!< The uart configuration to host */
//  } esp_zb_host_config_t;
//
#[cfg(false)]
impl Default for esp_zb_host_config_t {
    fn default() -> Self {
        esp_zb_host_config_t {
            host_connection_mode: esp_zb_host_connection_mode_t::ZB_HOST_CONNECTION_MODE_NONE,
            host_uart_config: esp_zb_uart_config_t::nada(),
        }
    }
}

//R
// /**
//  * @brief Structure of device descriptor on a endpoint
//  */
// typedef struct esp_zb_endpoint_config_s {
//     uint8_t    endpoint;                        /*!< Endpoint */
//     uint16_t   app_profile_id;                  /*!< Application profile identifier */
//     uint16_t   app_device_id;                   /*!< Application device identifier */
//     uint32_t   app_device_version: 4;           /*!< Application device version */
// } ESP_ZB_PACKED_STRUCT
// esp_zb_endpoint_config_t;
//
#[cfg(false)]
const O: esp_zb_endpoint_config_t = {
    let un = MaybeUninit::zeroed();
    unsafe { un.assume_init() }
};

#[cfg(false)]
impl Default for esp_zb_endpoint_config_t where Self: Clone {
    fn default() -> Self {
        O   // tbd. rename if works
    }
}

//#define ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG()
//    {
//         .basic_cfg =
//             {
//                 .zcl_version = ESP_ZB_ZCL_BASIC_ZCL_VERSION_DEFAULT_VALUE,
//                 .power_source = ESP_ZB_ZCL_BASIC_POWER_SOURCE_DEFAULT_VALUE,
//             },
//         .identify_cfg =
//             {
//                 .identify_time = ESP_ZB_ZCL_IDENTIFY_IDENTIFY_TIME_DEFAULT_VALUE,
//             },
//         .groups_cfg =
//             {
//                 .groups_name_support_id = ESP_ZB_ZCL_GROUPS_NAME_SUPPORT_DEFAULT_VALUE,
//             },
//         .scenes_cfg =
//             {
//                 .scenes_count = ESP_ZB_ZCL_SCENES_SCENE_COUNT_DEFAULT_VALUE,
//                 .current_scene = ESP_ZB_ZCL_SCENES_CURRENT_SCENE_DEFAULT_VALUE,
//                 .current_group = ESP_ZB_ZCL_SCENES_CURRENT_GROUP_DEFAULT_VALUE,
//                 .scene_valid = ESP_ZB_ZCL_SCENES_SCENE_VALID_DEFAULT_VALUE,
//                 .name_support = ESP_ZB_ZCL_SCENES_NAME_SUPPORT_DEFAULT_VALUE,
//             },
//         .on_off_cfg =
//             {
//                 .on_off = ESP_ZB_ZCL_ON_OFF_ON_OFF_DEFAULT_VALUE,
//             },
//         .level_cfg =
//             {
//                 .current_level = ESP_ZB_ZCL_LEVEL_CONTROL_CURRENT_LEVEL_DEFAULT_VALUE,
//             },
//         .color_cfg =
//             {
//                 .current_x = ESP_ZB_ZCL_COLOR_CONTROL_CURRENT_X_DEF_VALUE,
//                 .current_y = ESP_ZB_ZCL_COLOR_CONTROL_CURRENT_Y_DEF_VALUE,
//                 .color_mode = ESP_ZB_ZCL_COLOR_CONTROL_COLOR_MODE_DEFAULT_VALUE,
//                 .options = ESP_ZB_ZCL_COLOR_CONTROL_OPTIONS_DEFAULT_VALUE,
//                 .enhanced_color_mode = ESP_ZB_ZCL_COLOR_CONTROL_ENHANCED_COLOR_MODE_DEFAULT_VALUE,
//                 .color_capabilities = 0x0008,
//             },
//     }
//
#[cfg(feature = "ep_color_dimmable_light")]
#[cfg(false)]
impl Default for esp_zigbee_color_dimmable_light_cfg_t {
    fn default() -> Self {
        unsafe {
            ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG
        }
    }
}

/***R
impl ezb_nwk_network_status_t {
    // with 'strum'
    fn from_raw(v: u8) -> Option<Self> {
        Self::from_repr(v as u32)
    }

    // Note: '#[repr(u32)]' but signal carries only 'u8'.
    #[cfg(false)]   // without 'strum'
    fn from_raw(v: u8) -> Option<Self> {
        use ezb_nwk_network_status_t::*;

        match v {
            EZB_NWK_NETWORK_STATUS_LEGACY_NO_ROUTE_AVAILABLE |
            EZB_NWK_NETWORK_STATUS_LEGACY_LINK_FAILURE |
            EZB_NWK_NETWORK_STATUS_LINK_FAILURE |
            EZB_NWK_NETWORK_STATUS_LOW_BATTERY_LEVEL |
            EZB_NWK_NETWORK_STATUS_NO_ROUTING_CAPACITY |
            EZB_NWK_NETWORK_STATUS_NO_INDIRECT_CAPACITY |
            EZB_NWK_NETWORK_STATUS_INDIRECT_TRANSACTION_EXPIRY |
            EZB_NWK_NETWORK_STATUS_TARGET_DEVICE_UNAVAILABLE |
            EZB_NWK_NETWORK_STATUS_TARGET_ADDRESS_UNALLOCATED |
            EZB_NWK_NETWORK_STATUS_PARENT_LINK_FAILURE |
            EZB_NWK_NETWORK_STATUS_VALIDATE_ROUTE |
            EZB_NWK_NETWORK_STATUS_SOURCE_ROUTE_FAILURE |
            EZB_NWK_NETWORK_STATUS_MANY_TO_ONE_ROUTE_FAILURE |
            EZB_NWK_NETWORK_STATUS_ADDRESS_CONFLICT |
            EZB_NWK_NETWORK_STATUS_VERIFY_ADDRESS |
            EZB_NWK_NETWORK_STATUS_PAN_IDENTIFIER_UPDATE |
            EZB_NWK_NETWORK_STATUS_NETWORK_ADDRESS_UPDATE |
            EZB_NWK_NETWORK_STATUS_BAD_FRAME_COUNTER |
            EZB_NWK_NETWORK_STATUS_BAD_KEY_SEQUENCE_NUMBER |
            EZB_NWK_NETWORK_STATUS_UNKNOWN_COMMAND |
            EZB_NWK_NETWORK_STATUS_PANID_CONFLICT => {
                //Some( unsafe { core::mem::transmute::<u8, ezb_nwk_network_status_t>(v) } )
                Some(v as ezb_nwk_network_status_t)
            }
            _ => {
                None
            }
        }
    }
}
***/
