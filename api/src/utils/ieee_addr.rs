use core::fmt;

/**
* Wrapper for 64-bit addresses; C represents them as memory blocks.
*/
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IeeeAddr(pub u64);

impl IeeeAddr {
    /// Create from 'esp-zigbee-lib' (C side) [u8; 8] array.
    fn from_raw(bytes: [u8; 8]) -> Self {
        Self(u64::from_le_bytes(bytes))
    }

    /// Convert to what 'esp-zigbee-lib' uses.
    fn to_raw(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }
}

impl From<[u8; 8]> for IeeeAddr {
    fn from(bytes: [u8; 8]) -> Self {
        Self::from_raw(bytes)
    }
}

impl fmt::Display for IeeeAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.to_raw();

        // Display MSB first (like MAC addresses)
        write!(f,
               "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
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
