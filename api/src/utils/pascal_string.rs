use std::vec::Vec;

use core::ffi::c_void;

// tbd. Consider moving 'PascalString' to 'config' cradle. All Zigbee strings are this way, it's not an 'esp_zigbee_lib'
//      specific implementation detail.  (renders the '.into_leaked' method unnecessary)
/**
* Handling strings (e.g. Zigbee manufacturer info) where the initial byte provides the length.
*/
#[derive(Debug,Clone)]
pub struct PascalString(Vec<u8>);

impl PascalString {
    /**
    * Eats the struct and turns it into a leaked memory pointer.
    */
    pub fn into_leaked(self) -> &'static [u8] {
        let bs: Box<[u8]> = self.0.into_boxed_slice();
        Box::leak(bs)
    }
}

impl From<&str> for PascalString {
    fn from(s: &str) -> Self {
        Self( encode_pascal(s) )
    }
}

fn encode_pascal(s: &str) -> Vec<u8> {
    assert!(s.len() <= 255, "Zigbee Pascal string max length is 255 bytes");
    let len: u8 = s.len() as _;

    let mut v = Vec::with_capacity(len as usize + 1);
    v.push(len);  // length as first byte
    v.extend_from_slice(s.as_bytes());
    v
}
