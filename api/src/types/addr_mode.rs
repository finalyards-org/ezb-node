
use ezb_node_raw::{
    ezb_addr_mode_e,
    ezb_address_s,
    ezb_grpaddr_s,
    ezb_addr_u,
    //ezb_eui64_s,
};

use crate::{
    IeeeAddr
};

// There are three possibilities:
//
//  - 0xFFFC: Broadcast to all routers
//  - 0xFFFD: Broadcast to all non-sleepy devices (used in C 'color_dimmer_switch' example)
//  - 0xFFFF: Broadcast to all devices
//
// We currently fix this (for outgoing messages). Can be changed, if there is need for application level control.
// For _incoming_ messages, the field can be ignored. There does not seem to be a reason to trouble the application
// level with this.
//
// Note! If implementing, consider doing it as a "const generic" because there are only three options.
//
const BCAST_FIELD_OUTGOING: u16 = 0xfffd;

/**
 * Address mode, and also the address.
 *
 * Merges what C API 'ezb_addr_mode_e' and 'ezb_address_s' jointly do.
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
pub enum AddrMode {
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
     */
    Group(u16),

    /**
     * MAC: Address field contains a short address (16 bit).
     * NWK: 16-bit network address of a device or a 16-bit broadcast address.
     * APS: 16-bit address for DstAddress and DstEndpoint present.
     */
    Short(u16),     // this might be 'ezb_grpaddr_s'

    /**
     * MAC: Address field contains an extended address (64 bit).
     * NWK: Reserved.
     * APS: 64-bit extended address for DstAddress and DstEndpoint present.
     */
    Extended(IeeeAddr),
}

impl AddrMode {
    /**
    * @return None if some of the fields seem invalid/corrupt; Some for a valid conversion.
    */
    // Rust note: 'AsRef' so that the caller can use '&' - or not.
    pub(crate) fn parse<V: AsRef<ezb_address_s>>(v: V) -> Option<Self> {
        let ezb_address_s {
            addr_mode,
            u
        } = *v.as_ref();

        match ezb_addr_mode_e::parse(addr_mode)? {
            ezb_addr_mode_e::EZB_ADDR_MODE_NONE => {
                Some( Self::None )
            },
            ezb_addr_mode_e::EZB_ADDR_MODE_GROUP => {
                let ezb_grpaddr_s {
                    group,
                    bcast: _bcast   // incoming message; not passed on (expecting 0xFFFC|0xFFFD|0xFFFF)
                } = unsafe { u.group_addr };

                match _bcast {
                    0xFFFC|0xFFFD|0xFFFF => {}, // expected
                    x => {
                        log::warn!("Unexpected '.group_addr.bcast' (let through): 0x{:04x} not among 0xFFFC, 0xFFFD or 0xFFFF", x);
                    }
                }

                Some( Self::Group(group) )
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

impl Into<ezb_address_s> for AddrMode {
    fn into(self) -> ezb_address_s {
        type Out = ezb_address_s;

        match self {
            Self::None => Out {
                addr_mode: ezb_addr_mode_e::EZB_ADDR_MODE_NONE as u8,
                u: unsafe { core::mem::zeroed() }
            },
            Self::Short(v) => Out {
                addr_mode: ezb_addr_mode_e::EZB_ADDR_MODE_SHORT as u8,
                u: ezb_addr_u { short_addr: v }
            },
            Self::Group(v) => Out {
                addr_mode: ezb_addr_mode_e::EZB_ADDR_MODE_GROUP as u8,
                u: ezb_addr_u { group_addr: ezb_grpaddr_s {
                    group: v,
                    bcast: BCAST_FIELD_OUTGOING
                } }
            },
            Self::Extended(v) => Out {
                addr_mode: ezb_addr_mode_e::EZB_ADDR_MODE_EXT as u8,
                u: ezb_addr_u { extended_addr: v.into() }
            },
        }
    }
}
