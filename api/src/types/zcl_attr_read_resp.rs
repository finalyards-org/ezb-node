
use ezb_node_raw::{
    RspVariableIter,
    RspVariableEntry,
    ezb_zcl_read_attr_rsp_variable_s,
};

use crate::{
    ZclAttr,
    ZclError,
    AttrId,
};

#[derive(Debug, Clone)]
pub enum ZclAttrReadResp {
    Success(ZclAttr),
    Failure(AttrId, ZclError)
}

impl ZclAttrReadResp {
    /**
    * Parse a list of successful or failed variable (read) responses.
    *
    * @return None if there is a problem (bad data, or internal problems logged); Some if conversion took place.
    */
    // note: Raw level has provided a unified iterator for us, but it has not touched the contents.
    //      It's up to us to turn the 'u8'/'u16's into more application friendly types.
    //
    //pub enum RspVariableEntry {
    //     Success{ attr_id: u16, attr_type: ezb_zcl_attr_type_t /*u8*/, attr_value: *const core::ffi::c_void },
    //     Failure{ attr_id: u16, status: u8 }
    // }
    pub(crate) fn parse_list<'a,I>(vars: I) -> Option<Vec<Self>>
    where I: Into<RspVariableIter<'a, ezb_zcl_read_attr_rsp_variable_s>>
    {
        let iter = vars.into();

        iter //r ).into_iter() // Rust note: iterate by references
            .map(|x| Self::parse_one(&x))
            .collect()  // Any entry being 'None' (parsing failed) fails the whole list.
    }

    /**
    * Parse a single success/failure entry, from its 'raw' representation (plain numbers, may carry bad data).
    *
    * @note In reality, the fields are always as a linked list (raw), i.e. multiple fields.
    */
    fn parse_one(entry: &RspVariableEntry) -> Option<Self> {

        let o = match *entry {
            RspVariableEntry::Success{ attr_id, attr_type, attr_value } => {
                let attr = ZclAttr::parse2(attr_id, attr_type, attr_value)?;
                Self::Success(attr)
            },
            RspVariableEntry::Failure{ attr_id, status } => {
                let err = ZclError::parse(status)?.unwrap();     // st==0 would be a 'Success'
                Self::Failure(AttrId(attr_id), err)
            }
        };
        Some(o)
    }
}
