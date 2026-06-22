
use ezb_node_raw::{
    ezb_zcl_cmd_hdr_s,
};

use super::{
    AddrMode,
};

use crate::{
    ClusterId,
};

/**
 * @brief Structure containing information about a received ZCL command.
 *
 * Populated when a ZCL command is received and contains all addressing and frame control information from the
 * command header.
 */
//typedef struct ezb_zcl_cmd_hdr_s {
//     ezb_address_t src_addr;   /*!< Source address (short and IEEE address). */
//     ezb_address_t dst_addr;   /*!< Destination address of the command. */
//     uint8_t       src_ep;     /*!< Source endpoint ID. */
//     uint8_t       dst_ep;     /*!< Destination endpoint ID. */
//     uint16_t      cluster_id; /*!< Cluster ID for the command. */
//     uint16_t      profile_id; /*!< Application profile identifier (e.g., 0x0104 for Home Automation). */
//     uint8_t       fc;         /*!< Frame control byte (frame type, direction, etc.). */
//     uint16_t      manuf_code; /*!< Manufacturer code (0x0000 if not manufacturer-specific). */
//     uint8_t       tsn;        /*!< Transaction sequence number (0x00-0xFF). */
//     int8_t        rssi;       /*!< Received signal strength indicator in dBm. */
//     uint8_t       cmd_id;     /*!< Command identifier (0x00-0xFF). */
// } ezb_zcl_cmd_hdr_t;
//
// typedef struct ezb_address_s {
//     ezb_addr_mode_t addr_mode;  /*!< Address mode, refer to @ref ezb_addr_mode_e */
//     ezb_addr_t u;               /*!< The data of address, refer to @ref ezb_addr_u */
// } ezb_address_t;
//
//typedef union ezb_addr_u {
//     ezb_shortaddr_t short_addr;   /*!< 16-bit network short address */
//     ezb_grpaddr_t group_addr;     /*!< Group address information */
//     ezb_extaddr_t extended_addr;  /*!< 64-bit extended address */
// } ezb_addr_t;
#[derive(Debug, Clone)]
pub struct CommandHeader {
    src_addr: AddrMode,
    dst_addr: AddrMode,
    src_ep: u8,
    dst_ep: u8,
    cluster_id: ClusterId,
    #[allow(non_snake_case)]
    profile_id_X: u16,  // tbd. enum?
    #[allow(non_snake_case)]
    fc_X: u8,   // tbd. enum?
    /// Manufacturer code (0x0000 = not manufacturer-specific; presented as 'None').
    manuf_code: Option<u16>,
    /// Transaction sequence number (0x00..= 0xff).
    tsn: u8,
    /// Received signal strength indicator in dBm.
    rssi: i8,
    /// Command identifier (0x00-0xFF). Each cluster has their own interpretations.
    cmd_id: u8
}

impl CommandHeader {
    /**
    * Prepare a command header from C struct.
    */
    pub(crate) fn parse(v: *const ezb_zcl_cmd_hdr_s) -> Option<Self> {
        let v = unsafe { v.as_ref() }?;     // fail parsing if 'null'

        let ezb_zcl_cmd_hdr_s {
            src_addr, dst_addr,
            src_ep, dst_ep,
            cluster_id,
            profile_id,
            fc,
            manuf_code,
            tsn,
            rssi,
            cmd_id
        } = *v;
            // if new fields are added, won't compile!

        let o = Self {
            src_addr: AddrMode::parse(src_addr)?,
            dst_addr: AddrMode::parse(dst_addr)?,
            src_ep,
            dst_ep,
            cluster_id: ClusterId::parse(cluster_id)?,
            profile_id_X: profile_id,
            fc_X: fc,
            manuf_code: (manuf_code!=0).then(|| manuf_code),
            tsn,
            rssi,
            cmd_id
        };
        Some(o)
    }
}
