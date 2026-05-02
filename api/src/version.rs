//
// Version of the 'esp_zigbee_sdk'
//
use crate::raw::{
    esp_zigbee_get_version_string
};

use core::ffi::CStr;
use once_cell::sync::OnceCell;

static ZB_VERSION: OnceCell<&'static str> = OnceCell::new();

pub fn zigbee_get_version() -> &'static str {
    // Note: This assumes the C pointer provided by 'esp_zigbee_get_version_string()' remains valid, and the same,
    //      throughout program execution. (If not, we can make 'ZB_VERSION' a 'String', or a number..)
    //
    ZB_VERSION.get_or_init(|| {
        unsafe {
            let c_ptr = esp_zigbee_get_version_string();
            assert!(!c_ptr.is_null());

            let c_str = CStr::from_ptr(c_ptr);
            c_str.to_str().unwrap() // version string would be valid UTF-8/ASCII
        }
    })
}
