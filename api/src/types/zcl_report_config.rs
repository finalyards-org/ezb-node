// tbd. Is this something that should be (could be?) in the 'raw' level?
use ezb_node_raw::{
    ezb_zcl_attr_type_e,
    ezb_zcl_attr_variable_t,
};

/**
* One attribute's reporting configuration, sent to a remote device with 'NodeApi::configure_reporting'.
*
* The remote device (cluster server) then reports the attribute:
*   - at least every 'max_secs' (even if it hasn't changed), and
*   - when it has changed by at least 'change' - but not more often than every 'min_secs'.
*/
#[derive(Debug, Clone)]
pub struct ReportConfig {
    pub attr_id: u16,   // 'AttrId(u16)' if stays in "api"
    pub min_secs: u16,
    /// 0xFFFF disables periodic reporting (only changes are reported).
    pub max_secs: u16,
    /// Minimum change that triggers a report. The variant also gives the attribute's type (which must match the
    /// attribute's definition in the cluster).
    pub change: ReportableChange,
}

#[derive(Debug, Clone, Copy)]
pub enum ReportableChange {
    U8(u8),
    U16(u16),
}

impl ReportableChange {
    pub(crate) fn attr_type(&self) -> u8 {
        (match self {
            Self::U8(_) => ezb_zcl_attr_type_e::EZB_ZCL_ATTR_TYPE_UINT8,
            Self::U16(_) => ezb_zcl_attr_type_e::EZB_ZCL_ATTR_TYPE_UINT16,
        }) as u8
    }

    pub(crate) fn to_c(&self) -> ezb_zcl_attr_variable_t {
        let mut v: ezb_zcl_attr_variable_t = crate::utils::c_zeroed();
        match *self {
            Self::U8(x) => v.u8_ = x,
            Self::U16(x) => v.u16_ = x,
        }
        v
    }
}
