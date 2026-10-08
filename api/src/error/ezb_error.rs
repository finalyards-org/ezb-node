/*
* Covering 'esp_zigbee_lib' errors.
*
* Background:
*   The raw level prepares 'ezb_err_e' for us, which is a Strum-enabled enum carrying all known
*   values.
*/
use core::fmt::{
    Display,
    Formatter,
};
use log::warn;
use ezb_node_raw::{
    ezb_err_e,
    ezb_err_t,  // 'ffi::c_int'
};

/// C side errors
///
/// The purpose is to:
///     - take in a numeric error code from the C layer ('ezb_err_t')
///     - turn it into a meaningful, Rust-friendly representation
///     - provide a human-readable error message
///
/// We can:
///     - improve the naming (based on doc-comments in the C source)
///
/// The purpose is _not_ to:
///     - be able to programmatically differentiate between the codes, i.e. we don't expand them
///       into Rust enums
///
#[derive(Debug, strum::Display)]
pub enum EzbError {
    //  #define EZB_ERR_NO_MEM        EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x01) /*!< No sufficient memory */
    //  #define EZB_ERR_INV_ARG       EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x02) /*!< Invalid argument */
    //  #define EZB_ERR_INV_STATE     EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x03) /*!< Invalid state */
    //  #define EZB_ERR_INV_SIZE      EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x04) /*!< Invalid size */
    //  #define EZB_ERR_NOT_FOUND     EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x05) /*!< Requested resource not found */
    //  #define EZB_ERR_NOT_SUPPORTED EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x06) /*!< Operation is not supported */
    //  #define EZB_ERR_TIMEOUT       EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x07) /*!< Operation timed out */
    //  #define EZB_ERR_ABORT         EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x08) /*!< Operation is aborted */
    //  #define EZB_ERR_BUSY          EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x09) /*!< Busy state */
    //  #define EZB_ERR_NOT_FINISHED  EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x0C) /*!< Operation is not fully finished */
    //  #define EZB_ERR_NOT_ALLOWED   EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x0D) /*!< Operation is not allowed */
    //
    //  #define EZB_ERR_PARSE         EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x10) /*!< Fail to parse frame/packet */
    //  #define EZB_ERR_EMPTY_DATA    EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x11) /*!< Received Empty data */
    //  #define EZB_ERR_DROP          EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x12) /*!< Frame is dropped */
    //  #define EZB_ERR_SECURITY      EZB_MAKE_ERROR(EZB_ERR_CATEGORY_GENERIC, 0x13) /*!< Fail to process security */
    //
    NoSufficientMemory,
    InvalidArgument,
    InvalidState,
    InvalidSize,
    NotFound,
    NotSupported,
    TimedOut,
    Aborted,
    Busy,
    NotFinished,
    NotAllowed,
    FailToParse,
    EmptyData,
    FrameIsDropped,
    FailToProcessSecurity,

    // MAC:
    //  <<
    //    /** Unable to transmit due to channel being busy */
    //    #define EZB_ERR_MAC_CHANNEL_ACCESS_FAILURE EZB_MAKE_ERROR(EZB_ERR_CATEGORY_MAC, 0xe1U)
    //    /** ACK not received after tx with ack_req flag set */
    //    #define EZB_ERR_MAC_NO_ACK EZB_MAKE_ERROR(EZB_ERR_CATEGORY_MAC, 0xe9U)
    //  <<
    MacChannelAccessFailure,
    MacNoAck,

    _Other(ezb_err_t)   // "catch all" for possible other values
}

impl EzbError {
    /// @return None if 'ezb_err_t' is 0
    ///     Otherwise, a Rust-side enum that matches the C-side error code.
    ///
    fn parse(v: ezb_err_t) -> Option<Self> {
        if v == 0 {
            None    // no error
        } else {
            let x = if let Some(e) = ezb_err_e::parse(v) {     // recognized value
                use ezb_err_e::*;
                match e {
                    NO_MEM => Self::NoSufficientMemory,
                    INV_ARG => Self::InvalidArgument,
                    INV_STATE => Self::InvalidState,
                    INV_SIZE => Self::InvalidSize,
                    NOT_FOUND => Self::NotFound,
                    NOT_SUPPORTED => Self::NotSupported,
                    TIMEOUT => Self::TimedOut,
                    ABORT => Self::Aborted,
                    BUSY => Self::Busy,
                    NOT_FINISHED => Self::NotFinished,
                    NOT_ALLOWED => Self::NotAllowed,
                    PARSE => Self::FailToParse,
                    EMPTY_DATA => Self::EmptyData,
                    DROP => Self::FrameIsDropped,
                    SECURITY => Self::FailToProcessSecurity,

                    MAC_CHANNEL_ACCESS_FAILURE => Self::MacChannelAccessFailure,
                    MAC_NO_ACK => Self::MacNoAck,

                    // exhaustive match
                }
            } else {
                // UNRECOGNIZED value
                warn!("Unexpected 'ezb_err_t' value: {}", v);
                Self::_Other(v)
            };
            Some(x)
        }
    }

    pub(crate) fn wrap(v: ezb_err_t) -> Result<(), Self> {
        match Self::parse(v) {
            None => Ok(()),
            Some(err) => Err(err),
        }
    }
}

impl core::error::Error for EzbError {}

