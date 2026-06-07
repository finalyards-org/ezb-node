use strum;

use ezb_node_raw::ezb_zcl_status_e;

// Design:
//  instead of using C-like enums (mapping to integers), doing this as separate structs allows for additional
//  data to be added, if we so choose (the '.info' field of the ZCL event, or whatever makes sense from the viewpoint
//  of the application).

/// @brief ZCL status codes.
///
/// Returned by ZCL operations to indicate success or various error conditions.
#[derive(Debug, Clone, strum::Display)]
#[repr(u8)]    // In C, carried by 'ezb_zcl_status_t' which is 'u8'; range is 0..=195; bindgen expands to 'u32'
pub enum ZclError {
    ///< ZCL Fail
    Fail,
    ///< Server is not authorized to upgrade the client
    NotAuthorized,
    ///< Malformed command
    MalformedCmd,
    ///< Unsupported cluster command
    UnsupCmd,
    ///< Invalid field
    InvalidField,
    ///< Unsupported attribute
    UnsupAttrib,
    ///< Invalid value
    InvalidValue,
    ///< Read only
    ReadOnly,
    ///< Insufficient space
    InsufficientSpace,
    ///< Not found
    NotFound,
    ///< Unreportable attribute
    UnreportableAttrib,
    ///< Invalid type
    InvalidType,
    ///< Supplied values are inconsistent
    Inconsistent,
    ///< The credentials presented by the device sending the command are not sufficient to perform this action.
    ActionDenied,
    ///< Timeout
    Timeout,
    ///< Abort
    Abort,
    ///< Invalid OTA upgrade image
    InvalidImage,
    ///< Server does not have data block available yet
    WaitForData,
    ///< No OTA upgrade image available for the client
    NoImageAvailable,
    ///< The client still requires more OTA upgrade image files to successfully upgrade.
    RequireMoreImage,
    ///< The command has been received and is being processed.
    NotificationPending,
    ///< Calibration error
    CalibrationError,
    ///< Cluster is not found on the target endpoint
    UnsupportedCluster,
    //
    _Other(u8)
}

use ezb_zcl_status_e::*;

impl ZclError {

    // Design note:
    //  One benefit of the taken design is that we don't need to be concerned of unknown values.
    //  Rust doesn't allow unknown values in 'ezb_zcl_status_e', and the compiler *will* check
    //  that our match is exhaustive.
    //
    pub(crate) fn new(v: ezb_zcl_status_e) -> Option<Self> {
        match v {
            EZB_ZCL_STATUS_SUCCESS => None,
              //
            EZB_ZCL_STATUS_FAIL => Some(ZclError::Fail),
            EZB_ZCL_STATUS_NOT_AUTHORIZED => Some(ZclError::NotAuthorized),
            EZB_ZCL_STATUS_MALFORMED_CMD => Some(ZclError::MalformedCmd),
            EZB_ZCL_STATUS_UNSUP_CMD => Some(ZclError::UnsupCmd),
            EZB_ZCL_STATUS_INVALID_FIELD => Some(ZclError::InvalidField),
            EZB_ZCL_STATUS_UNSUP_ATTRIB => Some(ZclError::UnsupAttrib),
            EZB_ZCL_STATUS_INVALID_VALUE => Some(ZclError::InvalidValue),
            EZB_ZCL_STATUS_READ_ONLY => Some(ZclError::ReadOnly),
            EZB_ZCL_STATUS_INSUFFICIENT_SPACE => Some(ZclError::InsufficientSpace),
            EZB_ZCL_STATUS_NOT_FOUND => Some(ZclError::NotFound),
            EZB_ZCL_STATUS_UNREPORTBLE_ATTRIB => Some(ZclError::UnreportableAttrib),
            EZB_ZCL_STATUS_INVALID_TYPE => Some(ZclError::InvalidType),
            EZB_ZCL_STATUS_INCONSISTENT => Some(ZclError::Inconsistent),
            EZB_ZCL_STATUS_ACTION_DENIED => Some(ZclError::ActionDenied),
            EZB_ZCL_STATUS_TIMEOUT => Some(ZclError::Timeout),
            EZB_ZCL_STATUS_ABORT => Some(ZclError::Abort),
            EZB_ZCL_STATUS_INVALID_IMAGE => Some(ZclError::InvalidImage),
            EZB_ZCL_STATUS_WAIT_FOR_DATA => Some(ZclError::WaitForData),
            EZB_ZCL_STATUS_NO_IMAGE_AVAILABLE => Some(ZclError::NoImageAvailable),
            EZB_ZCL_STATUS_REQUIRE_MORE_IMAGE => Some(ZclError::RequireMoreImage),
            EZB_ZCL_STATUS_NOTIFICATION_PENDING => Some(ZclError::NotificationPending),
            EZB_ZCL_STATUS_CALIBRATION_ERROR => Some(ZclError::CalibrationError),
            EZB_ZCL_STATUS_UNSUPPORTED_CLUSTER => Some(ZclError::UnsupportedCluster),
        }
    }

    // Reality is, the 'esp_zigbee_lib' message structures give us 'u8's, not enums.
    //
    pub(crate) fn parse(v: u8) -> Option<Option<Self>> {
        ezb_zcl_status_e::from_repr(v as u32)
            .map(Self::new)
    }
}
