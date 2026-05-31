use std::vec::Vec;

use core::ffi::c_void;

/**
* Handling strings (e.g. Zigbee manufacturer info) where the initial byte provides the length.
*/
#[derive(Debug,Clone)]
pub struct PascalString(Vec<u8>);

impl PascalString {
    /**
    * Eats the struct and turns it into a leaked memory pointer.
    */
    pub fn into_leaked(self) -> *const c_void {
        let bs: Box<[u8]> = self.0.into_boxed_slice();
        let leaked: &'static mut [u8] = Box::leak(bs);

        leaked.as_ptr() as *const c_void
    }
}

impl From<&str> for PascalString {
    fn from(s: &str) -> Self {
        Self( encode_pascal(s) )
    }
}

fn encode_pascal(s: &str) -> Vec<u8> {
    assert!(s.len() <= 255);
    let len: u8 = s.len() as _;

    let mut v = Vec::with_capacity(len as usize + 1);
    v.push(len);  // length as first byte
    v.extend_from_slice(s.as_bytes());
    v
}
