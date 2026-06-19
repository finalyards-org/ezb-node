use core::fmt;

use ezb_node_raw::{
    ezb_extaddr_t,
};

/**
* Wrapper for 64-bit addresses; C represents them as packed memory blocks.
*
* This is used both for MAC addresses, and extended PAN IDs.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IeeeAddr(u64);

//EZB_PACKED_BEGIN
// struct ezb_eui64_s {
//     union {
//         uint8_t u8[8];  /*!< Extended Address as byte array */
//         uint64_t u64;   /*!< Extended Address as 64-bit value */
//     } EZB_PACKED_FIELD;
// } EZB_PACKED_END;
impl IeeeAddr {
}

impl From<[u8; 8]> for IeeeAddr {
    fn from(bytes: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(bytes))
    }
}

impl From<ezb_extaddr_t> for IeeeAddr {
    fn from(v: ezb_extaddr_t) -> Self {
        Self(v.to_u64())
    }
}

impl fmt::Display for IeeeAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.0.to_le_bytes();

        // Display MSB first (like MAC addresses)
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
           bytes[7], bytes[6], bytes[5], bytes[4], bytes[3], bytes[2], bytes[1], bytes[0]
        )
    }
}

// Enables 'println!("Osoite: {:x}", addr);'; do we need it?
#[cfg(false)]
impl fmt::LowerHex for IeeeAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}
