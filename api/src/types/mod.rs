/*
* Types that are visible to the application.
*
* Rules:
*   - We don't want to expose any of the 'raw' APIs to the applications. This means some
*     wrapping/renaming.
*/
//r use bitflags::bitflags;
// tbd. This may be eligible for breaking into sub-modules, one per each type?

use ezb_node_raw::{ezb_bdb_comm_mode_e, ezb_zcl_cluster_id_e};

mod addr_mode;
pub use addr_mode::AddrMode;

mod zcl_error;
pub use zcl_error::ZclError;

mod zcl_attr;
pub use zcl_attr::{ZclAttr, ZclValue};

mod zcl_attr_id;
pub use zcl_attr_id::AttrId;

mod zcl_command_header;
pub use zcl_command_header::CommandHeader;

mod zcl_cluster_id;
pub use zcl_cluster_id::ClusterId;

mod zcl_attr_read_resp;
pub use zcl_attr_read_resp::ZclAttrReadResp;

mod zcl_attr_write_resp;
pub use zcl_attr_write_resp::ZclAttrWriteResp;

mod zdp_error;
pub use zdp_error::ZdpError;

/**
* @brief Base Device Behavior (BDB) operation mode.
*/
// note:
//      The entity is a bit mask, but only one value seems to be used in reality. Thus, exposing this as a normal
//      'enum' seems okay.
//      Plus for enum:
//          - 'use BdbMode::...' will work (does not for 'struct' + inner consts)
//      Plus for 'struct':
//          - can carry the inner value ('ezb_bdb_comm_mode_e') directly, instead of the exposed C 'uint'.
//
#[derive(strum::FromRepr, Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]    // as in 'ezb_bdb_comm_capability_e' (though actual value range is less)
pub enum BdbMode {
    Initialization = ezb_bdb_comm_mode_e::EZB_BDB_MODE_INITIALIZATION.0, // 1
    #[cfg(false)]   // feature = "touchlink"
    TouchlinkInitiator = ezb_bdb_comm_mode_e::EZB_BDB_MODE_TOUCHLINK_INITIATOR.0,   // 2
    NetworkSteering   = ezb_bdb_comm_mode_e::EZB_BDB_MODE_NETWORK_STEERING.0,       // 4
    NetworkFormation  = ezb_bdb_comm_mode_e::EZB_BDB_MODE_NETWORK_FORMATION.0,      // 8
    FindingAndBinding = ezb_bdb_comm_mode_e::EZB_BDB_MODE_FINDING_N_BINDING.0,      // 16
    #[cfg(false)]   // feature = "touchlink"
    TouchlinkTarget   = ezb_bdb_comm_mode_e::EZB_BDB_MODE_TOUCHLINK_TARGET.0,       // 32
}

impl Into<ezb_bdb_comm_mode_e> for BdbMode {
    fn into(self) -> ezb_bdb_comm_mode_e {
        ezb_bdb_comm_mode_e(self as ::core::ffi::c_uint)
    }
}

#[cfg(false)]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CommissioningModesMask: u8 {
        const INITIALIZATION = ezb_bdb_comm_mode_t::EZB_BDB_MODE_INITIALIZATION.0 as u8; // 1
        #[cfg(false)]   // feature = "touchlink"
        const TOUCHLINK_INITIATOR = ezb_bdb_comm_mode_t::EZB_BDB_MODE_TOUCHLINK_INITIATOR.0 as u8; // 2
        const NETWORK_STEERING = ezb_bdb_comm_mode_t::EZB_BDB_MODE_NETWORK_STEERING.0 as u8; // 4
        #[cfg(feature = "coordinator")]
        const NETWORK_FORMATION = ezb_bdb_comm_mode_t::EZB_BDB_MODE_NETWORK_FORMATION.0 as u8; // 8
        const FINDING_N_BINDING = ezb_bdb_comm_mode_t::EZB_BDB_MODE_FINDING_N_BINDING.0 as u8; // 16
        #[cfg(false)]   // feature = "touchlink"
        const TOUCHLINK_TARGET = ezb_bdb_comm_mode_t::EZB_BDB_MODE_TOUCHLINK_TARGET.0 as u8; // 32

        // Declare all bits as "known". Recommended for 'bitflags', when working with C library APIs.
        const _ = !0;
    }
}

// it really is a bitmask, but we don't likely need it as such
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ClusterRole {
    Server = ezb_node_raw::EZB_ZCL_CLUSTER_SERVER as u8, // 1
    Client = ezb_node_raw::EZB_ZCL_CLUSTER_CLIENT as u8, // 2
}

impl ClusterRole {
    pub(crate) fn parse(v: u8) -> Option<Self> {
        match v as _ {
            ezb_node_raw::EZB_ZCL_CLUSTER_SERVER => Some(ClusterRole::Server),
            ezb_node_raw::EZB_ZCL_CLUSTER_CLIENT => Some(ClusterRole::Client),
            _ => None
        }
    }
}

/*** keep elsewhere! ;)
/**
* Helper for the types: allows input either as a const pointer, or a reference.
*/
pub(crate) trait AsPtrOrRef<'a, T> {
    fn as_ref(self) -> Option<&'a T>;
}

// Lifetime is the scope where the returned reference is placed.
impl<'a, T> AsPtrOrRef<'a, T> for *const T {
    fn as_ref(self) -> Option<&'a T> {
        if self.is_null() {
            None
        } else {
            unsafe { self.as_ref() }
        }
    }
}

impl<'a,T> AsPtrOrRef<'a,T> for &'a T {
    fn as_ref(self) -> Option<&'a T> {
        Some(self)
    }
}
***/

