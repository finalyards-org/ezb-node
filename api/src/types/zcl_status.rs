use ezb_node_raw::ezb_zcl_status_e;

/// @brief ZCL status codes.
///
/// Returned by ZCL operations to indicate success or various error conditions.
#[derive(Debug, Clone)]
#[repr(u32)]    // like in bindgen, though actual value range is 0..195
enum ZclStatus {
    ///< ZCL Success
    Success = ezb_zcl_status_e::EZB_ZCL_STATUS_SUCCESS as _, //0; must be zero (others indicate an error)
    ///< ZCL Fail
    Fail = ezb_zcl_status_e::EZB_ZCL_STATUS_FAIL as _,  //1
    ///< Server is not authorized to upgrade the client
    NotAuthorized = ezb_zcl_status_e::EZB_ZCL_STATUS_NOT_AUTHORIZED as _, //126
    ///< Malformed command
    MalformedCmd = ezb_zcl_status_e::EZB_ZCL_STATUS_MALFORMED_CMD as _, //128
    ///< Unsupported cluster command
    UnsupCmd = ezb_zcl_status_e::EZB_ZCL_STATUS_UNSUP_CMD as _, //129
    ///< Invalid field
    InvalidField = ezb_zcl_status_e::EZB_ZCL_STATUS_INVALID_FIELD as _, //133
    ///< Unsupported attribute
    UnsupAttrib = ezb_zcl_status_e::EZB_ZCL_STATUS_UNSUP_ATTRIB as _, //134
    ///< Invalid value
    InvalidValue = ezb_zcl_status_e::EZB_ZCL_STATUS_INVALID_VALUE as _, //135
    ///< Read only
    ReadOnly = ezb_zcl_status_e::EZB_ZCL_STATUS_READ_ONLY as _, //136
    ///< Insufficient space
    InsufficientSpace = ezb_zcl_status_e::EZB_ZCL_STATUS_INSUFFICIENT_SPACE as _, //137
    ///< Not found
    NotFound = ezb_zcl_status_e::EZB_ZCL_STATUS_NOT_FOUND as _, //139
    ///< Unreportable attribute
    UnreportableAttrib = ezb_zcl_status_e::EZB_ZCL_STATUS_UNREPORTBLE_ATTRIB as _, //140
    ///< Invalid type
    InvalidType = ezb_zcl_status_e::EZB_ZCL_STATUS_INVALID_TYPE as _, //141
    ///< Supplied values are inconsistent
    Inconsistent = ezb_zcl_status_e::EZB_ZCL_STATUS_INCONSISTENT as _, //146
    ///< The credentials presented by the device sending the command are not
    ///sufficient to perform this action.
    ActionDenied = ezb_zcl_status_e::EZB_ZCL_STATUS_ACTION_DENIED as _, //147
    ///< Timeout
    Timeout = ezb_zcl_status_e::EZB_ZCL_STATUS_TIMEOUT as _, //148
    ///< Abort
    Abort = ezb_zcl_status_e::EZB_ZCL_STATUS_ABORT as _, //149
    ///< Invalid OTA upgrade image
    InvalidImage = ezb_zcl_status_e::EZB_ZCL_STATUS_INVALID_IMAGE as _, //150
    ///< Server does not have data block available yet
    WaitForData = ezb_zcl_status_e::EZB_ZCL_STATUS_WAIT_FOR_DATA as _, //151
    ///< No OTA upgrade image available for the client
    NoImageAvailable = ezb_zcl_status_e::EZB_ZCL_STATUS_NO_IMAGE_AVAILABLE as _, //152
    ///< The client still requires more OTA upgrade image files to
    ///successfully upgrade.
    RequireMoreImage = ezb_zcl_status_e::EZB_ZCL_STATUS_REQUIRE_MORE_IMAGE as _, //153
    ///< The command has been received and is being processed.
    NotificationPending = ezb_zcl_status_e::EZB_ZCL_STATUS_NOTIFICATION_PENDING as _, //154
    ///< Calibration error
    CalibrationError = ezb_zcl_status_e::EZB_ZCL_STATUS_CALIBRATION_ERROR as _, //194
    ///< Cluster is not found on the target endpoint
    UnsupportedCluster = ezb_zcl_status_e::EZB_ZCL_STATUS_UNSUPPORTED_CLUSTER as _, //195
}
