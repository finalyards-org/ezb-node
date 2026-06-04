
use ezb_node_raw::{
    ezb_zcl_write_attr_rsp_variable_s,
    ezb_zcl_status_e,
};

use crate::{

};

/**
 * Represents the result of writing one attribute in a write attribute response.
 */
// Note: C struct only reports 'attr_id's for failed writes.
//
//typedef struct ezb_zcl_write_attr_rsp_variable_s {
//    ezb_zcl_status_t status;  /*!< Status of the write operation. See @ref ezb_zcl_status_t. */
//    uint16_t         attr_id; /*!< Attribute ID that was written. Only present if status is not SUCCESS. */
//    struct ezb_zcl_write_attr_rsp_variable_s *next; /*!< Pointer to the next variable in the response list, or NULL if last. */
//} ezb_zcl_write_attr_rsp_variable_t;
//
#[derive(Debug, Clone)]
pub enum ZclWriteAttrResp {
    Success,            // since we don't get to hear which attribute succeeded, this feels a bit squid
    Failure(ezb_zcl_status_e, u16)  // attr id are standardized; depend on the cluster
}

impl ZclWriteAttrResp {
    fn parse(v: &ezb_zcl_write_attr_rsp_variable_s) -> Option<Self> {

        let happy_r = if v.status == 0 {  // EZB_ZCL_STATUS_SUCCESS
            Self::Success
        } else {
            Self::Failure(
                ezb_zcl_status_e::parse(v.status)?,
                v.attr_id
            )
        };
        Some(happy_r)
    }
}
