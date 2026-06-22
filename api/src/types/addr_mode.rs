
use ezb_node_raw::{
    ezb_addr_mode_e,
    ezb_address_s,
    ezb_grpaddr_s,
};

use crate::{
    IeeeAddr
};

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
                    bcast: _bcast
                } = unsafe { u.group_addr };

                log::warn!("Converting group address with '.bcast' {} (.bcast omitted)", _bcast);  // tbd.
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
