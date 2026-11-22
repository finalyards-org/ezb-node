// 'NodeApi' methods that are IAS CIE device type specific.
#![cfg(feature = "dt_ias_cie")]

use ezb_node_raw::ezb_nwk_get_extended_address;

use crate::{
    error::EzbError,
    CommandHeader,
};

use super::{
    Node,
    ZigbeeGuard,
};

/// Provide applications' event handlers access to (parts of) the C level API.
///
impl<T: Node> NodeApiIasCie for T {}

/// Access to methods of the 'esp_zigbee_lib' from a context (application RTOS thread) that
/// requires locking.
///
/// @design
///     We could provide a '.guarded(|x| { x.aaa(); x.bbb() } )' interface that has the benefit
///     of allowing multiple calls per one lock. Currently, we don't. Everything is guarded,
///     but the application developer _does not need_ to know about it.
///
pub trait NodeApiIasCie {
    /**
    * Write our own IEEE address to a zone device's IAS Zone 'IAS_CIE_Address' attribute (0x0010).
    *
    * Many zone devices (e.g. door/window sensors) only send a "Zone Enroll Request" once a CIE has written its
    * address there.
    *
    * @param dst_addr   Short (network) address of the zone device; e.g. from 'AppSignal::ZdoSignalDeviceAnnce'.
    * @param dst_ep     Endpoint of the zone device that hosts the IAS Zone (server) cluster.
    * @param src_ep     Our endpoint (the IAS CIE one).
    *
    * @note The result is reported by the remote device as a 'ZclEvent::WriteAttrResp'.
    */
    fn write_ias_cie_address(&self, dst_addr: u16, dst_ep: u8, src_ep: u8) -> Result<(),EzbError> {
        use ezb_node_raw::{
            ezb_zcl_attribute_s,
            ezb_zcl_attribute_s__bindgen_ty_1,
            ezb_zcl_attr_type_e::EZB_ZCL_ATTR_TYPE_EUI64,
            ezb_zcl_cluster_id_e::EZB_ZCL_CLUSTER_ID_IAS_ZONE,
            ezb_zcl_cmd_ctrl_t,
            ezb_zcl_ias_zone_server_attr_t::EZB_ZCL_ATTR_IAS_ZONE_IAS_CIE_ADDRESS_ID,
            ezb_zcl_write_attr_cmd_req,
            ezb_zcl_write_attr_cmd_t,
        };
        use crate::AddrMode;

        let _guard = ZigbeeGuard::acquire();

        // Our address as a native 'u64' - which is the little endian byte order also used in the ZCL frame.
        let mut value: u64 = unsafe { ezb_nwk_get_extended_address() };

        let mut attr = ezb_zcl_attribute_s {
            id: EZB_ZCL_ATTR_IAS_ZONE_IAS_CIE_ADDRESS_ID as u16,
            data: ezb_zcl_attribute_s__bindgen_ty_1 {
                type_: EZB_ZCL_ATTR_TYPE_EUI64 as u8,
                size: size_of::<u64>() as u16,
                value: (&raw mut value).cast(),
            },
        };

        let mut cmd: ezb_zcl_write_attr_cmd_t = crate::utils::c_zeroed();
        cmd.cmd_ctrl = ezb_zcl_cmd_ctrl_t {
            dst_addr: AddrMode::Short(dst_addr).into(),
            dst_ep,
            src_ep,
            cluster_id: EZB_ZCL_CLUSTER_ID_IAS_ZONE as u16,
            ..crate::utils::c_zeroed()  // standard command, direction "to server", default response enabled
        };
        cmd.payload.attr_number = 1;
        cmd.payload.attr_field = &raw mut attr;

        // STRONG ASSUMPTION: the command is serialized during the call (i.e. 'attr', 'value' need not outlive it).
        // Not documented, though. #feedback
        EzbError::wrap(unsafe {
            ezb_zcl_write_attr_cmd_req(&cmd)
        })
    }

    /**
    * Reply to an IAS Zone "Zone Enroll Request" (see 'ZclEvent::IasZoneEnrollReq').
    *
    * The response is sent to the device (and endpoint) that sent the request, from the endpoint that received it.
    *
    * @param req_header The '.header' of the received 'ZclEvent::IasZoneEnrollReq'.
    * @param zone_id    The zone ID we (the CIE) give to the device; unique per enrolled zone (0x00..=0xFE).
    * @param code       Whether the enrollment is accepted.
    */
    fn ias_zone_enroll_response(&self, req_header: &CommandHeader, zone_id: u8, code: IasZoneEnrollResponse) -> Result<(), EzbError> {
        use ezb_node_raw::{
            ezb_zcl_ias_zone_enroll_cmd_resp,
            ezb_zcl_ias_zone_enroll_rsp_cmd_t,
            ezb_zcl_ias_zone_enroll_rsp_payload_t,
        };

        let cmd = ezb_zcl_ias_zone_enroll_rsp_cmd_t {
            cmd_ctrl: req_header.reply_ctrl(),
            payload: ezb_zcl_ias_zone_enroll_rsp_payload_t {
                enroll_rsp_code: code as u8,
                zone_id,
            },
        };

        let _guard = ZigbeeGuard::acquire();
        EzbError::wrap(unsafe {
            // Note: Also here, can we trust the C side does not need to access the pointer after the call? #espressif
            ezb_zcl_ias_zone_enroll_cmd_resp(&cmd)
        })
    }
}

/// Response code for an IAS Zone enrollment (see 'NodeApiIasCie::ias_zone_enroll_response').
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IasZoneEnrollResponse {
    Success = ezb_node_raw::ezb_zcl_ias_zone_enroll_rsp_code_t::EZB_ZCL_IAS_ZONE_ENROLL_RESPONSE_CODE_SUCCESS as u8,
    NotSupported = ezb_node_raw::ezb_zcl_ias_zone_enroll_rsp_code_t::EZB_ZCL_IAS_ZONE_ENROLL_RESPONSE_CODE_NOT_SUPPORTED as u8,
    NotPermitted = ezb_node_raw::ezb_zcl_ias_zone_enroll_rsp_code_t::EZB_ZCL_IAS_ZONE_ENROLL_RESPONSE_CODE_NOT_PERMITTED as u8,
    TooManyZones = ezb_node_raw::ezb_zcl_ias_zone_enroll_rsp_code_t::EZB_ZCL_IAS_ZONE_ENROLL_RESPONSE_CODE_TOO_MANY_ZONES as u8,
}
