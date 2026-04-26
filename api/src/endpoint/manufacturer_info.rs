/*
*
*/
use core::fmt;

// typedef struct zcl_basic_manufacturer_info_s {
//     char *manufacturer_name;
//     char *model_identifier;
// } zcl_basic_manufacturer_info_t;
//
struct ManufacturerInfo {
    manfacturer_name: &str,
    model_identifier: &str,
}

impl Default for BasicManufacturerInfo {
    fn default() -> Self {}
}


fn encode_pascal(s: &str) -> Vec<u8> {
    let mut b = Vec::new();
    b.push(s.len() as u8); // Lasketaan pituus automaattisesti
    b.extend_from_slice(s.as_bytes());
    b
}



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



let info: zcl_basic_manufacturer_info_t = Info... {
.manufacturer_name = ESP_MANUFACTURER_NAME,
.model_identifier = ESP_MODEL_IDENTIFIER,
};
