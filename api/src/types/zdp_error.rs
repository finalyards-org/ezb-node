use strum;

use ezb_node_raw::ezb_zdp_status_e;

// Design: (see 'zcl_error.h')

/// ZDP status codes.
///
/// Returned by device binding attempt
#[derive(Debug, Clone, strum::Display)]
#[repr(u8)]    // In C, carried by 'ezb_zdp_status_t' which is 'u8'; range is 0..=143; bindgen expands to 'u32'
pub enum ZdpError {
    /// The supplied request type was invalid.
    InvalidRequestType,
    /// The requested device did not exist on a device following a child descriptor request to a parent.
    DeviceNotFound,
    /// The supplied endpoint was equal to 0x00 or between 0xf1 and 0xff.
    InvalidEp,
    /// The requested endpoint is not described by simple descriptor.
    NotActive,
    /// The requested optional feature is not supported on the target device.
    NotSupported,
    /// A timeout has occurred with the requested operation.
    Timeout,
    /// The end device bind request was unsuccessful due to a failure to match any suitable clusters.
    NoMatch,
    /// The unbind request was unsuccessful due to the coordinator or source device not having an entry
    /// in its binding table to unbind.
    NoEntry,
    /// A child descriptor was not available following a discovery request to a parent.
    NoDescriptor,
    /// The device does not have space to support the requested operation.
    InsufficientSpace,
    /// The device is not in the proper state to support the requested operation.
    NotPermitted,
    /// The device does not have table space to support the operation.
    TableFull,
    /// The permissions configuration table on the target indicates that the request is not authorized
    /// from this device.
    NotAuthorized,
    /// The device doesn't have binding table space to support the operation
    BindingTableFull,
    /// The index in the received command is out of bounds.
    InvalidIndex,
    /// The response was too large to fit in a single unfragmented message. (+ 2.1.0)
    FrameTooLarge,
    /// The requested Key Negotiation Method was not accepted. (+ 2.1.0)
    BadKnMethod,
    /// The request encountered a temporary failure but a retry at a later time should be attempted
    /// and may succeed. (+ 2.1.0)
    TemporaryFailure,

    _Other(u8)
}

impl ZdpError {

    // Design note:
    //  One benefit of the taken design is that we don't need to be concerned of unknown values.
    //  Rust doesn't allow unknown values in 'ezb_zdp_status_e', and the compiler *will* check
    //  that our match is exhaustive.
    //
    pub(crate) fn new(v: ezb_zdp_status_e) -> Option<Self> {
        #[allow(non_camel_case_types)]
        type e = ezb_zdp_status_e;  // trick to make sure 'match' treats them as enums

        match v {
            e::EZB_ZDP_STATUS_SUCCESS => None,
              //
            e::EZB_ZDP_STATUS_INV_REQUESTTYPE       => Some(Self::InvalidRequestType),
            e::EZB_ZDP_STATUS_DEVICE_NOT_FOUND      => Some(Self::DeviceNotFound),
            e::EZB_ZDP_STATUS_INVALID_EP            => Some(Self::InvalidEp),
            e::EZB_ZDP_STATUS_NOT_ACTIVE            => Some(Self::NotActive),
            e::EZB_ZDP_STATUS_NOT_SUPPORTED         => Some(Self::NotSupported),
            e::EZB_ZDP_STATUS_TIMEOUT               => Some(Self::Timeout),
            e::EZB_ZDP_STATUS_NO_MATCH              => Some(Self::NoMatch),
            e::EZB_ZDP_STATUS_NO_ENTRY              => Some(Self::NoEntry),
            e::EZB_ZDP_STATUS_NO_DESCRIPTOR         => Some(Self::NoDescriptor),
            e::EZB_ZDP_STATUS_INSUFFICIENT_SPACE    => Some(Self::InsufficientSpace),
            e::EZB_ZDP_STATUS_NOT_PERMITTED         => Some(Self::NotPermitted),
            e::EZB_ZDP_STATUS_TABLE_FULL            => Some(Self::TableFull),
            e::EZB_ZDP_STATUS_NOT_AUTHORIZED        => Some(Self::NotAuthorized),
            e::EZB_ZDP_STATUS_BINDING_TABLE_FULL    => Some(Self::BindingTableFull),
            e::EZB_ZDP_STATUS_INVALID_INDEX         => Some(Self::InvalidIndex),

            // Added in 2.1.0
            e::EZB_ZDP_STATUS_FRAME_TOO_LARGE       => Some(Self::FrameTooLarge),
            e::EZB_ZDP_STATUS_BAD_KN_METHOD         => Some(Self::BadKnMethod),
            e::EZB_ZDP_STATUS_TEMPORARY_FAILURE     => Some(Self::TemporaryFailure),

            _ => {
                log::warn!("Unknown ZDP error code: {} ({})", v, v as u32);
                Some(Self::_Other(v as u8))
            }
        }
    }

    // Reality is, the 'esp_zigbee_lib' message structures give us 'u8's, not enums.
    //
    pub(crate) fn parse(v: u8) -> Option<Option<Self>> {
        ezb_zdp_status_e::from_repr(v as u32)
            .map(Self::new)
    }
}
