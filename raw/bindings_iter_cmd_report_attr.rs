//
// Iteration of the linked list in a ZCL "Report Attributes" message ('ezb_zcl_cmd_report_attr_message_t').
//
// Unlike read responses, a report has no per-attribute status: every entry carries a value.
//
//pub struct ezb_zcl_report_attr_variable_s {
//     pub attr_id: u16,
//     pub attr_type: u8,
//     pub attr_value: *mut ::core::ffi::c_void,
//     pub next: *mut ezb_zcl_report_attr_variable_s,
// }
//
#[derive(Debug)]
pub struct ReportVariableEntry {
    pub attr_id: u16,
    pub attr_type: ezb_zcl_attr_type_t, /*u8*/
    pub attr_value: *const core::ffi::c_void,
}

pub struct ReportVariableIter<'a> {
    current: *mut ezb_zcl_report_attr_variable_s,
    _marker: core::marker::PhantomData<&'a ezb_zcl_report_attr_variable_s>,
}

impl<'a> Iterator for ReportVariableIter<'a> {
    type Item = ReportVariableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {     // end of list
            return None;
        }

        let ezb_zcl_report_attr_variable_s {
            attr_id, attr_type, attr_value, next
        } = unsafe { core::ptr::read_unaligned(self.current) };

        self.current = next;
        Some(ReportVariableEntry { attr_id, attr_type, attr_value })
    }
}

impl ezb_zcl_report_attr_variable_s {
    // The iterator's lifespan is tied to ours, meaning the linked list remains available to it.
    //
    pub fn iter(&self) -> ReportVariableIter<'_> {
        ReportVariableIter {
            current: self as *const Self as *mut Self,
            _marker: core::marker::PhantomData,
        }
    }
}
