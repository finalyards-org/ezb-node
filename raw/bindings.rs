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

pub const EZB_ZCL_CLUSTER_CLIENT: u8 = a::EZB_ZCL_CLUSTER_CLIENT as _;
pub const EZB_ZCL_CLUSTER_SERVER: u8 = a::EZB_ZCL_CLUSTER_SERVER as _;

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

/*
* Because 'esp_zigbee_device_config_t' has a union field, it's best we treat it here.
*/
//pub struct esp_zigbee_device_config_s {
//     ///< The nwk device type, @ref ezb_nwk_device_type_t
//     pub device_type: ezb_nwk_device_type_t,
//     ///< Allow install code security policy or not
//     pub install_code_policy: bool,
//     pub __bindgen_anon_1: esp_zigbee_device_config_s__bindgen_ty_1,
// }
//
//pub union esp_zigbee_device_config_s__bindgen_ty_1 {
//     ///< The Zigbee zc/zr device configuration
//     pub zczr_config: esp_zigbee_zczr_config_s,
//     ///< The Zigbee zed device configuration
//     pub zed_config: esp_zigbee_zed_config_s,
// }
//
//pub struct esp_zigbee_zczr_config_s {
//     ///< Max number of the children
//     pub max_children: u8,
// }
#[cfg(false)]   //R
impl esp_zigbee_device_config_t {

    /// Create a device configuration for a Coordinator.
    pub fn for_zc(install_code_policy: bool, max_children: u8) -> Self {
        Self::for_zczr(EZB_NWK_DEVICE_TYPE_COORDINATOR, install_code_policy, max_children)
    }

    /// Create a device configuration for a Router.
    pub fn for_zr(install_code_policy: bool, max_children: u8) -> Self {
        Self::for_zczr(EZB_NWK_DEVICE_TYPE_ROUTER, install_code_policy, max_children)
    }

    fn for_zczr(device_type: ezb_nwk_device_type_t, install_code_policy: bool, max_children: u8) -> Self {
        Self {
            device_type,
            install_code_policy,
            __bindgen_anon_1: esp_zigbee_device_config_s__bindgen_ty_1 {
                zczr_config: esp_zigbee_zczr_config_s {
                    max_children
                }
            }
        }
    }

    /// Create a device configuration for an End Device.
    #[deprecated(note="Not yet tested - please do report if it works! :)")]
    #[cfg(false)]
    //? #[allow(dead_code)]
    pub fn for_zed(install_code_policy: bool, ed_timeout: ezb_nwk_ed_timeout_e, keep_alive: Duration) -> Self {
        let device_type = ezb_nwk_device_type_t::EZB_NWK_DEVICE_TYPE_END_DEVICE;
        Self {
            device_type,
            install_code_policy,
            __bindgen_anon_1: esp_zigbee_device_config_s__bindgen_ty_1 {
                zed_config: esp_zigbee_zed_config_s {
                    ed_timeout: ed_timeout as u8,
                    keep_alive: keep_alive.as_millis() as u32,
                }
            }
        }
    }
}

/*
* Default for a device configuration.
*/
impl Default for ezb_zha_color_dimmable_light_config_t {
    fn default() -> Self {
        unsafe {
            EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG
        }
    }
}


/*
* Instead of exposing the union bindgen-generated names, it's better to provide generators for the
* variants.
*/
impl esp_zigbee_device_config_t {
    pub fn for_zczr(device_type: ezb_nwk_device_type_t, install_code_policy: bool, max_children: u8) -> Self {
        esp_zigbee_device_config_t {
            device_type,
            install_code_policy,
            __bindgen_anon_1: esp_zigbee_device_config_s__bindgen_ty_1 {
                zczr_config: esp_zigbee_zczr_config_s {
                    max_children
                }
            }
        }
    }

    #[deprecated(note="Not yet tested")]
    #[cfg(false)]
    pub fn for_zed(ed_timeout: ezb_nwk_ed_timeout_e, keep_alive: u32) -> Self {
        esp_zigbee_device_config_t {
            device_type: EZB_NWK_DEVICE_TYPE_END_DEVICE,
            install_code_policy,
            __bindgen_anon_1: esp_zigbee_device_config_s__bindgen_ty_1 {
                zed_config: esp_zigbee_zed_config_s {
                    ed_timeout: ed_timeout as u8,
                    keep_alive
                }
            }
        }
    }
}

impl esp_zigbee_platform_config_t {
    pub fn for_native_mode(storage_partition_name: &'static str) -> Self {
        esp_zigbee_platform_config_t {
            storage_partition_name: storage_partition_name.as_ptr(),
            radio_config: esp_zigbee_radio_config_t {
                radio_mode: esp_zigbee_radio_mode_t::ESP_ZIGBEE_RADIO_MODE_NATIVE,
                __bindgen_anon_1: unsafe { core::mem::zeroed() } // not needed
            }
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
