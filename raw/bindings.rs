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
#[cfg(false)]   //R (if 'esp-idf-svc' works)
pub mod sys {
    pub use super::{
        nvs_flash_init,
        nvs_flash_init_partition,
    };
}

#[cfg(false)]
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

// About the 'esp_zigbee_lib' (2.0) API design:
//  - for 'ezb_eui64_s' (used e.g. for IEEE addresses), could it not be passed by-value, also in the C API?
//  - the use of 'union' is low friction within C, but burdensome for Rust.
//
// For these reasons, we overwrite certain C side functions:
//

/// @brief Get the IEEE (extended) address of the device.
/// @anchor ezb_nwk_get_extended_panid
///
/// @return Slice of 64-bit IEEE address, little-endian.
// note: Returning the slice, and not 'u64', because: - it makes the little-endianess more explicit,
//      - C API has the union "packed", meaning it can be 1-byte aligned. Though we provide the buffer
//        here, it still carries that packed attribute with it. So ... slice is likely having less surprises!
//
pub unsafe fn ezb_nwk_get_extended_panid() -> [u8;8] {
    let mut buf: ezb_extpanid_t = empty();
    unsafe {
        a::ezb_nwk_get_extended_panid(&mut buf)
    };

    unsafe { buf.__bindgen_anon_1.u8_ }
}

/// @brief Obtains the type of the application signal
///
// Wrapped so that API layer gets a Rust enum, straight up (without it needing to use 'strum').
pub unsafe fn ezb_app_signal_get_type(signal: *const ezb_app_signal_t) -> ezb_app_signal_type_e {
    let v = unsafe { a::ezb_app_signal_get_type(signal) };
    ezb_app_signal_type_e::from_repr(v as u32).unwrap_or_else(|| {
        panic!("No such 'ezb_app_signal_type_e': {v}")
    })
}

// 'ezb_eui64_s' is a rather unremarkable union, in the C API. It does, however, cause headache
// in Rust conversion. Here are helper methods!
//
// The issues are twofold: that it's a union, and that it's "packed". Packed means that it would be unsafe
// (as in, potentially invoking Undefined Behaviour) to simply read the '._u64' field of the union. We counteract
// this here, in the 'raw' layer, to avoid such concerns in the API.
//
impl ezb_eui64_s {
    pub fn to_le_bytes(self) -> [u8; 8] {
        unsafe {
            self.__bindgen_anon_1.u8_
        }
    }
    pub fn to_u64(self) -> u64 {
        // NOTE: NOT ENCOURAGED. 'google.ai' says it's UB, even when the compiler knows the struct is "packed".
        //|unsafe { self.__bindgen_anon_1.u64_ }

        // Safe way to read a potentially unaligned 'u64'
        unsafe {
            let ptr = core::ptr::addr_of!(self.__bindgen_anon_1.u64_);
            ptr.read_unaligned()
        }
    }
}

fn empty<T>() -> T {
    let un = MaybeUninit::zeroed();
    unsafe { un.assume_init() }
}
