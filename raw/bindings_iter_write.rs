//
// Handle iteration of linked lists ('next' pointers) already here.
//
//pub struct ezb_zcl_write_attr_rsp_variable_s {
//     ///< Status of the write operation. See @ref ezb_zcl_status_t.
//     pub status: ezb_zcl_status_t,
//     ///< Attribute ID that was written. Only present if status is not SUCCESS.
//     pub attr_id: u16,
//     ///< Pointer to the next variable in the response list, or NULL if last.
//     pub next: *mut ezb_zcl_write_attr_rsp_variable_s,
// }
//
#[derive(Debug)]
pub enum WriteRspVariableEntry {
    Success,    // no other info if it's a success
    Failure{ status: u8, attr_id: u16 }
}

impl WriteRspVariableEntry {
    fn new(status: u8, attr_id: u16) -> Self {
        if status == ezb_zcl_status_e::EZB_ZCL_STATUS_SUCCESS as _ {
            Self::Success
        } else {
            Self::Failure { status, attr_id }
        }
    }
}

// T ∈ { ezb_zcl_write_attr_rsp_variable_s, ... }
pub struct WriteRspVariableIter<'a, T> {
    current: *mut T,
    _marker: core::marker::PhantomData<&'a T>,
}

/**
* Help the iterator (below) to extract necessary fields from various raw types. These types have *same form* but are
* separate.
*/
// ezb_zcl_write_attr_rsp_variable_s
pub struct WriteCommonFields<T> {
    status: u8,
    attr_id: u16,
    next: *mut T
}

macro_rules! impl_write_common_fields_from {
    ($struct_name:ident) => {
        impl From<$struct_name> for WriteCommonFields<$struct_name> {
            fn from(x: $struct_name) -> Self {
                let $struct_name {
                    attr_id, status, next
                } = x;
                Self {
                    attr_id, status, next
                }
            }
        }
    }
}
impl_write_common_fields_from!(ezb_zcl_write_attr_rsp_variable_s);
//..? more?

impl<'a, T> Iterator for WriteRspVariableIter<'a, T>
where
//T: Clone,                       // 'core::ptr::read_unaligned' often demands "at least bitwise copy/clone" (google.ai); raw structs are ':Copy, Clone'
    WriteCommonFields<T>: From<T>,   // 'T' has fields that we can access
{
    type Item = WriteRspVariableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {     // end of list
            return None;
        }

        // Read current struct
        //  - alignment safely (only IEEE addresses have 'packed' alignment in the C library (2.0.1))
        //  - ensuring we handle all their fields
        //
        let WriteCommonFields {
            status,
            attr_id,
            next
        } = unsafe { core::ptr::read_unaligned(self.current) }.into();  // magic!

        let entry = WriteRspVariableEntry::new(status, attr_id);

        // move the iterator
        self.current = next;

        Some(entry)
    }
}

macro_rules! impl_write_common_fields_iter {
    ($struct_type:ty) => {
        impl $struct_type {
            // The iterator's lifespan is tied to ours, meaning the linked list remains available to it.
            //
            pub fn iter(&self) -> WriteRspVariableIter<'_,Self> {
                WriteRspVariableIter {
                    current: self as *const Self as *mut Self,
                    _marker: core::marker::PhantomData,
                }
            }
        }

        impl<'a> Into<WriteRspVariableIter<'a, $struct_type>> for &'a $struct_type {
            fn into(self) -> WriteRspVariableIter<'a, $struct_type> {
                self.iter()
            }
        }
    }
}
impl_write_common_fields_iter!(ezb_zcl_write_attr_rsp_variable_s);
//..more?
