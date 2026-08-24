use core::fmt;

use ezb_node_raw::{
    ezb_eui64_s,
};

/**
* Used both for MAC addresses, and extended PAN IDs.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IeeeAddr(pub(crate) u64);

#[cfg(false)]   // disabled; dealing with 'u64' instead
impl From<[u8; 8]> for IeeeAddr {
    fn from(bytes: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(bytes))
    }
}

impl From<ezb_eui64_s> for IeeeAddr {
    fn from(v: ezb_eui64_s) -> Self {
        Self(v.into())
    }
}

impl Into<ezb_eui64_s> for IeeeAddr {
    fn into(self) -> ezb_eui64_s {
        self.0 .into()
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
