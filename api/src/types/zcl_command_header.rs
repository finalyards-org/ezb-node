
use ezb_node_raw::{
    ezb_zcl_cmd_hdr_t,
    ezb_zcl_cmd_hdr_s,
    ezb_zcl_read_attr_rsp_variable_t,
    ezb_address_s,
    ezb_addr_u,
    ezb_addr_mode_e,
};

use crate::{IeeeAddr, ClusterId};

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
    src_addr: Addr,
    dst_addr: Addr,
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
    // Note: The caller has '*const' but we don't wish to burden them with the syntax of conversion.
    //      Don't want to burden the actual 'parse_ref' either.
    //
    pub(crate) fn parse(p: *const ezb_zcl_cmd_hdr_s) -> Option<Self> {
        Self::parse_ref(unsafe { p.as_ref() }?)
    }

    fn parse_ref(v: &ezb_zcl_cmd_hdr_s) -> Option<Self> {
        let ezb_zcl_cmd_hdr_s{
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
            src_addr: Addr::parse(src_addr)?,
            dst_addr: Addr::parse(dst_addr)?,
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

/**
 * @brief Address mode definition.
 * @anchor ezb_addr_mode_e
*/
// enum ezb_addr_mode_e {
//     /**
//      * MAC: PAN ID and address fields are not present.
//      * NWK: Reserved.
//      * APS: DstAddress and DstEndpoint not present.
//      */
//     EZB_ADDR_MODE_NONE = 0,
//     /**
//      * MAC: Reserved.
//      * NWK: Reserved.
//      * APS: 16-bit group address for DstAddress; DstEndpoint not present.
//      */
//     EZB_ADDR_MODE_GROUP = 1,
//     /**
//      * MAC: Address field contains a short address (16 bit).
//      * NWK: 16-bit network address of a device or a 16-bit broadcast address.
//      * APS: 16-bit address for DstAddress and DstEndpoint present.
//      */
//     EZB_ADDR_MODE_SHORT = 2,
//     /**
//      * MAC: Address field contains an extended address (64 bit).
//      * NWK: Reserved.
//      * APS: 64-bit extended address for DstAddress and DstEndpoint present.
//      */
//     EZB_ADDR_MODE_EXT = 3,
// };
#[derive(Debug, Clone)]
#[allow(dead_code)] // ..until we use 'Group' et.al.
pub(crate) enum Addr {
    #[cfg(false)]
    /**
     * MAC: PAN ID and address fields are not present.
     * NWK: Reserved.
     * APS: DstAddress and DstEndpoint not present.
     */
    None,

    /**
     * MAC: Reserved.
     * NWK: Reserved.
     * APS: 16-bit group address for DstAddress; DstEndpoint not present.
     *
     * @note In Rust, we _only_ cater for receiving at the moment. The value is the
     *      actual group id ('.group_addr.group' in C); the broadcast address is dropped.
     *      This will need to change, if we ever need to support group _sending_ and want to
     *      use this struct for doing so!
     */
    Group(u16),

    /**
     * MAC: Address field contains a short address (16 bit).
     * NWK: 16-bit network address of a device or a 16-bit broadcast address.
     * APS: 16-bit address for DstAddress and DstEndpoint present.
     */
    Short(u16),

    /**
     * MAC: Address field contains an extended address (64 bit).
     * NWK: Reserved.
     * APS: 64-bit extended address for DstAddress and DstEndpoint present.
     */
    Extended(IeeeAddr),
}

impl Addr {

    /**
    * @return None if some of the fields seem invalid/corrupt; Some for a valid conversion.
    */
    fn parse(v: ezb_address_s) -> Option<Self> {
        let ezb_address_s {
            addr_mode,
            u
        } = v;

        match ezb_addr_mode_e::parse(addr_mode)? {
            ezb_addr_mode_e::EZB_ADDR_MODE_NONE => {
                log::warn!("[internal] Met address mode 'NONE'. Enable it in the code! (currently skipped)");
                return None;    // skips
                //Some( Self::None )
            },
            ezb_addr_mode_e::EZB_ADDR_MODE_GROUP => {
                let x = unsafe { u.group_addr };
                let _ = x.bcast;    // Note! We do not take the 'u.bcast' further. Coded for receiving only, for now!
                Some( Self::Group(x.group) )
            },
            ezb_addr_mode_e::EZB_ADDR_MODE_SHORT => {
                let x = unsafe { u.short_addr };
                Some( Self::Short(x) )
            },
            ezb_addr_mode_e::EZB_ADDR_MODE_EXT => {
                let x = unsafe { u.extended_addr };
                Some( Self::Extended(x.into()) )
            }
        }
    }
}
