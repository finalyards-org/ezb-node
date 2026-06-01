/*
* Types that are visible to the application.
*
* Rules:
*   - We don't want to expose any of the 'raw' APIs to the applications. This means some
*     wrapping/renaming.
*/
//r use bitflags::bitflags;

use ezb_node_raw::ezb_bdb_comm_mode_e;

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
    #[cfg(feature = "touchlink")]
    TouchlinkInitiator = ezb_bdb_comm_mode_e::EZB_BDB_MODE_TOUCHLINK_INITIATOR.0,   // 2
    NetworkSteering   = ezb_bdb_comm_mode_e::EZB_BDB_MODE_NETWORK_STEERING.0,       // 4
    NetworkFormation  = ezb_bdb_comm_mode_e::EZB_BDB_MODE_NETWORK_FORMATION.0,      // 8
    FindingAndBinding = ezb_bdb_comm_mode_e::EZB_BDB_MODE_FINDING_N_BINDING.0,      // 16
    #[cfg(feature = "touchlink")]
    TouchlinkTarget   = ezb_bdb_comm_mode_e::EZB_BDB_MODE_TOUCHLINK_TARGET.0,       // 32
}

impl BdbMode {
    /***
    pub fn to_raw(self) -> ezb_node_raw::ezb_bdb_comm_mode_e {
        ezb_node_raw::ezb_bdb_comm_mode_e(self as ::core::ffi::c_uint)
    }***/
}

/***
pub struct BdbMode(ezb_bdb_comm_mode_e);

impl BdbMode {
    pub const INITIALIZATION: Self = Self(ezb_bdb_comm_mode_e::EZB_BDB_MODE_INITIALIZATION); // 1
    #[cfg(feature = "touchlink")]
    pub const TOUCHLINK_INITIATOR: Self = Self(ezb_bdb_comm_mode_e::EZB_BDB_MODE_TOUCHLINK_INITIATOR); // 2
    pub const NETWORK_STEERING: Self = Self(ezb_bdb_comm_mode_e::EZB_BDB_MODE_NETWORK_STEERING); // 4
    #[cfg(feature = "coordinator")]
    pub const NETWORK_FORMATION: Self = Self(ezb_bdb_comm_mode_e::EZB_BDB_MODE_NETWORK_FORMATION); // 8
    pub const FINDING_N_BINDING: Self = Self(ezb_bdb_comm_mode_e::EZB_BDB_MODE_FINDING_N_BINDING); // 16
    #[cfg(feature = "touchlink")]
    pub const TOUCHLINK_TARGET: Self = Self(ezb_bdb_comm_mode_e::EZB_BDB_MODE_TOUCHLINK_TARGET); // 32
}***/

#[cfg(false)]
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CommissioningModesMask: u8 {
        const INITIALIZATION = ezb_bdb_comm_mode_t::EZB_BDB_MODE_INITIALIZATION.0 as u8; // 1
        #[cfg(feature = "touchlink")]
        const TOUCHLINK_INITIATOR = ezb_bdb_comm_mode_t::EZB_BDB_MODE_TOUCHLINK_INITIATOR.0 as u8; // 2
        const NETWORK_STEERING = ezb_bdb_comm_mode_t::EZB_BDB_MODE_NETWORK_STEERING.0 as u8; // 4
        #[cfg(feature = "coordinator")]
        const NETWORK_FORMATION = ezb_bdb_comm_mode_t::EZB_BDB_MODE_NETWORK_FORMATION.0 as u8; // 8
        const FINDING_N_BINDING = ezb_bdb_comm_mode_t::EZB_BDB_MODE_FINDING_N_BINDING.0 as u8; // 16
        #[cfg(feature = "touchlink")]
        const TOUCHLINK_TARGET = ezb_bdb_comm_mode_t::EZB_BDB_MODE_TOUCHLINK_TARGET.0 as u8; // 32

        // Declare all bits as "known". Recommended for 'bitflags', when working with C library APIs.
        const _ = !0;
    }
}
