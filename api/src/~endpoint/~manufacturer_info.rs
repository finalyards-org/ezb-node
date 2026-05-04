/*
*
*/
extern crate alloc;

use alloc::vec::Vec;

/**
* Basic manufacturer information.
*/
// tbd. describe when and where this is exposed over Zigbee comms.
//
// typedef struct zcl_basic_manufacturer_info_s {
//     char *manufacturer_name;
//     char *model_identifier;
// } zcl_basic_manufacturer_info_t;
//
pub struct ManufacturerInfo {
    /// name of e.g. your company
    pub manufacturer_name: &'static str,
    /// name of your particular product
    pub model_identifier: &'static str,
}

impl ManufacturerInfo {
    // tbd. ensure the lifetime of the strings, and/or write directly to C buffers.

    pub(crate) fn as_zcl_strings(&self) -> (Vec<u8>, Vec<u8>) {
        (encode_pascal(self.manufacturer_name), encode_pascal(self.model_identifier))
    }
}

// #later; place the restriction is a suitable place
//let len: usize = s.len().min(32); // Zigbee normally limits it to 32 chars (source: google.ai); tbd. check

fn encode_pascal(s: &str) -> Vec<u8> {
    assert!(s.len() <= 255);
    let len: u8 = s.len() as _;

    let mut v = Vec::with_capacity(len as usize + 1);
        v.push(len);  // length as first byte
        v.extend_from_slice(s.as_bytes());
    v
}
