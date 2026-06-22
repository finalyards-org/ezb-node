use strum;

use ezb_node_raw::{ezb_zcl_status_e, ezb_zdp_status_e};

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

impl ZclError {

    // Design note:
    //  One benefit of the taken design is that we don't need to be concerned of unknown values.
    //  Rust doesn't allow unknown values in 'ezb_zcl_status_e', and the compiler *will* check
    //  that our match is exhaustive.
    //
    pub(crate) fn new(v: ezb_zcl_status_e) -> Option<Self> {
        #[allow(non_camel_case_types)]
        type e = ezb_zcl_status_e;  // trick to make sure 'match' treats them as enums

        match v {
            e::EZB_ZCL_STATUS_SUCCESS => None,
              //
            e::EZB_ZCL_STATUS_FAIL                  => Some(Self::Fail),
            e::EZB_ZCL_STATUS_NOT_AUTHORIZED        => Some(Self::NotAuthorized),
            e::EZB_ZCL_STATUS_MALFORMED_CMD         => Some(Self::MalformedCmd),
            e::EZB_ZCL_STATUS_UNSUP_CMD             => Some(Self::UnsupCmd),
            e::EZB_ZCL_STATUS_INVALID_FIELD         => Some(Self::InvalidField),
            e::EZB_ZCL_STATUS_UNSUP_ATTRIB          => Some(Self::UnsupAttrib),
            e::EZB_ZCL_STATUS_INVALID_VALUE         => Some(Self::InvalidValue),
            e::EZB_ZCL_STATUS_READ_ONLY             => Some(Self::ReadOnly),
            e::EZB_ZCL_STATUS_INSUFFICIENT_SPACE    => Some(Self::InsufficientSpace),
            e::EZB_ZCL_STATUS_NOT_FOUND             => Some(Self::NotFound),
            e::EZB_ZCL_STATUS_UNREPORTBLE_ATTRIB    => Some(Self::UnreportableAttrib),
            e::EZB_ZCL_STATUS_INVALID_TYPE          => Some(Self::InvalidType),
            e::EZB_ZCL_STATUS_INCONSISTENT          => Some(Self::Inconsistent),
            e::EZB_ZCL_STATUS_ACTION_DENIED         => Some(Self::ActionDenied),
            e::EZB_ZCL_STATUS_TIMEOUT               => Some(Self::Timeout),
            e::EZB_ZCL_STATUS_ABORT                 => Some(Self::Abort),
            e::EZB_ZCL_STATUS_INVALID_IMAGE         => Some(Self::InvalidImage),
            e::EZB_ZCL_STATUS_WAIT_FOR_DATA         => Some(Self::WaitForData),
            e::EZB_ZCL_STATUS_NO_IMAGE_AVAILABLE    => Some(Self::NoImageAvailable),
            e::EZB_ZCL_STATUS_REQUIRE_MORE_IMAGE    => Some(Self::RequireMoreImage),
            e::EZB_ZCL_STATUS_NOTIFICATION_PENDING  => Some(Self::NotificationPending),
            e::EZB_ZCL_STATUS_CALIBRATION_ERROR     => Some(Self::CalibrationError),
            e::EZB_ZCL_STATUS_UNSUPPORTED_CLUSTER   => Some(Self::UnsupportedCluster),
        }
    }

    // Reality is, the 'esp_zigbee_lib' message structures give us 'u8's, not enums.
    //
    pub(crate) fn parse(v: u8) -> Option<Option<Self>> {
        ezb_zcl_status_e::from_repr(v as u32)
            .map(Self::new)
    }
}
