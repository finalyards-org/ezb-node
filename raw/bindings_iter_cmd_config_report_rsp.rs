//
// Iteration of a "Configure Reporting" response ('ezb_zcl_cmd_config_report_rsp_message_t').
//
// If all records were configured successfully, the list has just one entry, with SUCCESS status.
// If not, there's one entry per failed record.
//      @claude: Do you mean in such a case (>1 entries) only the failures are reported. We could
//              change the iterator to reflect this, iterating only the errors. #later
//
//pub struct ezb_zcl_config_report_rsp_variable_s {
//     // "@ref ezb_zcl_status_t"
//     pub status: u8,
//     /// "@ref EZB_ZCL_REPORTING_SEND or @ref EZB_ZCL_REPORTING_RECV.
//     /// If status is SUCCESS, this field is [...] 0xFF."
//     pub direction: u8,   // <-- this can be ignored; just informs whether it was server->client or other way
//
//     /// Attribute ID for the report. If status is SUCCESS, this field is [...] 0xFFFF.
//     pub attr_id: u16,
//     pub next: *mut ezb_zcl_config_report_rsp_variable_s,
// }
//

// tbd. If we change the iterator to only show errors (see comment above), could call this
//      'ConfigReportRspVariableFailure' (and ditch the 'Success' + it being an enum).
#[derive(Debug)]
pub enum ConfigReportRspVariableEntry {
    Success,
    Failure {
        /// Status of the configure report operation.
        status: u8,     // ezb_zcl_status_t (not zero)
        // tbd. the C header doesn't really say much: @claude what is the purpose of it?
        attr_id: u16,
    }
}

impl ConfigReportRspVariableEntry {
    fn new(status: u8, attr_id: u16) -> Self {
        if status == 0 {
            Self::Success
        } else {
            Self::Failure { status, attr_id }
        }
    }
}

pub struct ConfigReportRspVariableIter<'a> {
    current: *mut ezb_zcl_config_report_rsp_variable_s,
    _marker: core::marker::PhantomData<&'a ezb_zcl_config_report_rsp_variable_s>,
}

impl<'a> Iterator for ConfigReportRspVariableIter<'a> {
    type Item = ConfigReportRspVariableEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {     // end of list
            return None;
        }

        let ezb_zcl_config_report_rsp_variable_s {
            status, direction: _, attr_id, next
        } = unsafe { core::ptr::read_unaligned(self.current) };

        self.current = next;

        Some(ConfigReportRspVariableEntry::new(status, attr_id))
    }
}

impl ezb_zcl_config_report_rsp_variable_s {
    pub fn iter(&self) -> ConfigReportRspVariableIter<'_> {
        ConfigReportRspVariableIter {
            current: self as *const Self as *mut Self,
            _marker: core::marker::PhantomData,
        }
    }
}
