//
// Version of the 'esp_zigbee_sdk'
//
use alloc::string::String;

use crate::raw::{
    esp_zigbee_get_version_string
};

use core::ffi::CStr;

pub fn zigbee_get_version() -> String {
    let c_ptr = unsafe {
        esp_zigbee_get_version_string()
    };
    assert!(!c_ptr.is_null());  // 'esp_zigbee_lib' wouldn't be so..

    let c_str = unsafe {
        CStr::from_ptr(c_ptr)
    };
    c_str.into()
}
