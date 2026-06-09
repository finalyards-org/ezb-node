// Many anonymous structs are essentially the same. This helps 'api' level deal with them, as one.
//
// The struct is essentially two-in-one. We do the split into two, and we take care of the 'next' pointer
// (implementing an Iterator for the 'api' level).
//
//pub struct ezb_zcl_read_attr_rsp_variable_s {
//     ///< Attribute ID that was read.
//     pub attr_id: u16,
//     ///< Status of the read operation. See @ref ezb_zcl_status_t.
//     pub status: u8,
//     ///< Data type of the attribute. See @ref ezb_zcl_attr_type_t. Only valid
//     /// if status is SUCCESS.
//     pub attr_type: u8,
//     ///< Pointer to the attribute value buffer. Only valid if status is
//     /// SUCCESS.
//     pub attr_value: *mut ::core::ffi::c_void,
//     ///< Pointer to the next variable in the response list, or NULL if last.
//     pub next: *mut ezb_zcl_read_attr_rsp_variable_s,
// }
//
#[derive(Debug)]
pub enum RspVariableEntry {
    Success{ attr_id: u16, attr_type: ezb_zcl_attr_type_t /*u8*/, attr_value: *const core::ffi::c_void }, // status: 0
    Failure{ attr_id: u16, status: u8 }
}

impl RspVariableEntry {
    fn new(attr_id: u16, status: u8, attr_type: u8, attr_value: *const core::ffi::c_void) -> Self {
        if status == ezb_zcl_status_e::EZB_ZCL_STATUS_SUCCESS as _ {
            Self::Success { attr_id, attr_type, attr_value }
        } else {
            Self::Failure { attr_id, status }
        }
    }
}

// T ∈ { ezb_zcl_read_attr_rsp_variable_s, ... }
pub struct RspVariableIter<'a, T> {
    current: *mut T,
    _marker: core::marker::PhantomData<&'a T>,
}

/**
* Help the iterator (below) to extract necessary fields from various raw types. These types have *same form* but are
* separate.
*/
// ezb_zcl_read_attr_rsp_variable_s
pub struct SomeCommonFields<T> {
    attr_id: u16,
    status: u8,
    attr_type: u8,
    attr_value: *mut ::core::ffi::c_void,
    next: *mut T
}

macro_rules! impl_common_fields_from {
    ($struct_name:ident) => {
        impl From<$struct_name> for SomeCommonFields<$struct_name> {
            fn from(x: $struct_name) -> Self {
                let $struct_name {
                    attr_id, status, attr_type, attr_value, next
                } = x;
                Self {
                    attr_id, status, attr_type, attr_value, next
                }
            }
        }
    }
}
impl_common_fields_from!(ezb_zcl_read_attr_rsp_variable_s);
//r impl_common_fields_from!(ezb_zcl_write_attr_rsp_variable_s);

impl<'a, T> Iterator for RspVariableIter<'a, T>
where
//T: Clone,                       // 'core::ptr::read_unaligned' often demands "at least bitwise copy/clone" (google.ai); raw structs are ':Copy, Clone'
    SomeCommonFields<T>: From<T>,   // 'T' has 'SomeCommonFields' that we can access
{
    type Item = RspVariableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {     // end of list
            return None;
        }

        // Read current struct
        //  - alignment safely (only IEEE addresses have 'packed' alignment in the C library (2.0.1))
        //  - ensuring we handle all their fields
        //
        let SomeCommonFields {
            attr_id,
            status,
            attr_type,
            attr_value,
            next
        } = unsafe { core::ptr::read_unaligned(self.current) }.into();  // magic!

        let entry = RspVariableEntry::new(attr_id, status, attr_type, attr_value);

        // move the iterator
        self.current = next;

        Some(entry)
    }
}

macro_rules! impl_common_fields_iter {
    ($struct_type:ty) => {
        impl $struct_type {
            // The iterator's lifespan is tied to ours, meaning the linked list remains available to it.
            //
            pub fn iter(&self) -> RspVariableIter<'_,Self> {
                RspVariableIter {
                    current: self as *const Self as *mut Self,
                    _marker: core::marker::PhantomData,
                }
            }
        }

        impl<'a> Into<RspVariableIter<'a, $struct_type>> for &'a $struct_type {
            fn into(self) -> RspVariableIter<'a, $struct_type> {
                self.iter()
            }
        }
    }
}
impl_common_fields_iter!(ezb_zcl_read_attr_rsp_variable_s);
//..more
