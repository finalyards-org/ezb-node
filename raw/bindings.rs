/*
* Gathering the bindgen-generated binding like this allows us to attach
* 'Default' (and/or other traits) to its types.
*/
use alloc::vec;
use alloc::vec::Vec;
use core::mem::MaybeUninit;

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
#[allow(non_camel_case_types)]
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
mod a {
    include!("tmp/bindings_0.rs");
}

pub use a::*;
    //
    // Note: This only exposes the C bindings to 'lib.rs', not dependent crates.
    //      Some values, e.g. 'EZB_ZCL_CLUSTER_{SERVER|CLIENT}' are not going to be further exposed.

// #hack: allow use of 'OnceLock<esp_zigbee_config_t>' within the 'api'.
//
unsafe impl Send for esp_zigbee_config_s {}
unsafe impl Sync for esp_zigbee_config_s {}

// Larger constructs are grouped together, but they are still within this module.
include!("bindings_iter_read.rs");
include!("bindings_iter_write.rs");

/*
* Default for a device configuration.
*/
impl Default for ezb_zha_color_dimmable_light_config_t {
    fn default() -> Self {
        // See 'wrap.h' for details.
        //
        // ATTEMPT 1: "undefined reference to `EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG'" in linkage (i.e. either
        //      the 'const' does not generate any C object, or 'bindgen' is wrongly configured to take one along).
        #[cfg(false)]
        unsafe {
            EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG
        }

        // ATTEMPT 2: Did not generate output in 'tmp/bindings_0.rs'

        // ATTEMPT 3: "undefined reference to `wrap_EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG'"
        //#[cfg(false)]
        unsafe {
            wrap_EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG()
        }

        // ATTEMPT 4:
        #[cfg(false)]
        include!("bindings_default.rs");
    }
}

impl Default for ezb_zha_color_dimmer_switch_config_t {
    fn default() -> Self {
        unsafe {
            wrap_EZB_ZHA_COLOR_DIMMER_SWITCH_CONFIG()
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

//pub struct ezb_zdp_match_desc_req_field_s {
//     ///< Network address of the device to query
//     pub nwk_addr_of_interest: ezb_shortaddr_t,
//     ///< Profile ID to match at the destination, see @ref ezb_af_profile_id_t
//     pub profile_id: u16,
//     ///< Number of input clusters (server clusters) in the cluster_list
//     pub num_in_clusters: u8,
//     ///< Number of output clusters (client clusters) in the cluster_list
//     pub num_out_clusters: u8,
//     ///< Pointer to array of cluster IDs. First num_in_clusters entries are
//     /// input clusters,   followed by num_out_clusters entries for output
//     /// clusters. Total size must be   (num_in_clusters + num_out_clusters)
//     /// * sizeof(uint16_t)
//     pub cluster_list: *mut u16,
// }
//
impl ezb_zdp_match_desc_req_field_s {
    pub fn new(
        addr_of_interest: ezb_shortaddr_t,  // 0xFFFD
        profile_id: ezb_af_profile_id_e,    // ...HA...
        in_clusters: &[ezb_zcl_cluster_id_e],
        out_clusters: &[ezb_zcl_cluster_id_e]
    ) -> (Self, Vec<u16>) {
        let in_n = in_clusters.len();
        let out_n = out_clusters.len();

        let mut all_clusters: Vec<u16> = Vec::with_capacity(in_n + out_n);

        all_clusters.extend_from_slice(in_clusters.map(|x| x as u16));
        all_clusters.extend_from_slice(out_clusters.map(|x| x as u16));

        let me = Self {
            nwk_addr_of_interest: addr_of_interest,
            profile_id: profile_id as u16,
            num_in_clusters: in_n as u8,
            num_out_clusters: out_n as u8,
            cluster_list: all_clusters.as_ptr()
        };
        (me, all_clusters)
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

// Cover the raw function, 'ezb_zcl_cluster_id_e' for the enum.
/// @brief Get a cluster descriptor from an endpoint descriptor.
///
/// @param ep_desc    Pointer to endpoint descriptor.
/// @param cluster_id Cluster identifier.
/// @param role       Cluster role (server/client).
/// @return Pointer to the cluster descriptor, or NULL if not found.
#[cfg(false)]   //r: handle in 'api' level
pub unsafe fn ezb_af_endpoint_get_cluster_desc(
    ep_desc: ezb_af_ep_desc_t,
    cluster_id: ezb_zcl_cluster_id_e, // u16
    role: ClusterRole, // u8
) -> ezb_zcl_cluster_desc_t {
    unsafe {
        a::ezb_af_endpoint_get_cluster_desc(
            ep_desc,
            cluster_id as u16,
            role as u8
            // tbd. the role could be an enum, but we cannot mask out 'a::EZB_ZCL_CLUSTER_{SERVER|CLIENT}' easily, can we?
        )
    }
}

// Cover the raw function, 'ezb_zcl_basic_server_attr_t' for the enum.
/// @brief Add an attribute to a basic cluster descriptor.
///
/// @param cluster_desc Pointer to the basic cluster descriptor, see ezb_zcl_cluster_desc_t.
/// @param attr_id      Attribute identifier.
/// @param value        Pointer to the attribute value.
/// @return Error code.
pub unsafe fn ezb_zcl_basic_cluster_desc_add_attr(
    cluster_desc: ezb_zcl_cluster_desc_t,
    attr_id: ezb_zcl_basic_server_attr_t,   // u16
    value: *const ::core::ffi::c_void,
) -> ezb_err_t {
    unsafe {
        a::ezb_zcl_basic_cluster_desc_add_attr(
            cluster_desc,
            attr_id as u16,
            value,
        )
    }
}

// Allow use of '_e', not u8
/// @brief  Start top level commissioning procedure with specified mode
/// mask.
///
/// @param[in] mode_mask commissioning modes
///
/// @return - EZB_ERR_NONE on success
pub unsafe fn ezb_bdb_start_top_level_commissioning(mode_mask: ezb_bdb_comm_mode_e) -> ezb_err_t {
    unsafe { a::ezb_bdb_start_top_level_commissioning(mode_mask.0 as u8) }
}


//---
// Provide '::parse()' instead of strum's '::from_repr' to the API level; more consistent.
//
impl ezb_addr_mode_e {
    pub fn parse(v: ezb_addr_mode_t) -> Option<Self> {
        Self::from_repr(v as u32)
    }
}
impl ezb_nwk_network_status_t { // is an enum, in 'bindings_0.rs'
    pub fn parse(v: u8) -> Option<Self> {
        Self::from_repr(v as u32)
    }
}
impl ezb_app_signal_type_e {
    pub fn parse(v: ezb_app_signal_type_t) -> Option<Self> {
        Self::from_repr(v as u32)
    }
}
impl ezb_zcl_attr_type_e {
    pub fn parse(v: ezb_zcl_attr_type_t) -> Option<Self> {
        Self::from_repr(v as u32)
    }
}
impl ezb_zcl_core_action_callback_id_e {
    pub fn parse(v: ezb_zcl_core_action_callback_id_t) -> Option<Self> {
        Self::from_repr(v as u32)
    }
}
impl ezb_zcl_status_e {
    pub fn parse(v: ezb_zcl_status_t) -> Option<Self> {
        Self::from_repr(v as u32)
    }
}
impl ezb_zcl_cluster_id_e {
    pub fn parse(v: ezb_zcl_cluster_id_t) -> Option<Self> {
        Self::from_repr(v as u32)
    }
}

// Likely, we don't need any other profiles than 'HA'
/*** #keep for now
/**
 * @brief Zigbee application profile ID
 *    -
 */
// C code carries these in 'ezb_af_profile_id_t' (u16); bindgen repr is u32
#[repr(u16)]
enum ezb_af_profile_id_e {
    #[cfg(false)]   // not needed by applications
    EZB_AF_ZDP_PROFILE_ID = a::ezb_af_profile_id_e::EZB_AF_ZDP_PROFILE_ID.0, /*!< Zigbee Device Profile (ZDP) ID. Used by Zigbee internal network management. */
    // Covers all that's Zigbee 3.0
    //  - defines the standard clusters
    EZB_AF_HA_PROFILE_ID  = a::ezb_af_profile_id_e::EZB_AF_HA_PROFILE_ID.0, /*!< Home Automation (HA) profile ID */

    #[cfg(false)]   // SmartEnergy; not in focus
    EZB_AF_SE_PROFILE_ID  = 0x0109U, /*!< SE profile ID */
    #[cfg(feature = "touchlink")]
    EZB_AF_TL_PROFILE_ID  = a::ezb_af_profile_id_e::EZB_AF_TL_PROFILE_ID.0, /*!< Touchlink profile ID */
    #[cfg(false)]
    EZB_AF_GP_PROFILE_ID  = 0xA1E0U, /*!< GreenPower profile ID */
};
***/
