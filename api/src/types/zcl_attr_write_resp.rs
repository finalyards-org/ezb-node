
use ezb_node_raw::{
    WriteRspVariableIter,
    WriteRspVariableEntry,
    ezb_zcl_write_attr_rsp_variable_s,
};

use crate::{
    ZclAttr,
    ZclError,
    AttrId,
};

#[derive(Debug, Clone)]
pub enum ZclAttrWriteResp {
    Success,            // since we don't get to hear which attribute succeeded, this feels a bit squid
    Failure(ZclError, AttrId)
}

impl ZclAttrWriteResp {
    /**
    * Parse a list of successful or failed variable (write) responses.
    *
    * @return None if there is a problem (bad data, or internal problems logged); Some if conversion took place.
    */
    // note: Raw level has provided a unified iterator for us, but it has not touched the contents.
    //      It's up to us to turn the 'u8'/'u16's into more application friendly types.
    //
    //pub enum WriteRspVariableEntry {
    //     Success,    // no other info if it's a success
    //     Failure{ status: u8, attr_id: u16 }
    // }
    pub(crate) fn parse_list<'a,I>(vars: I) -> Option<Vec<Self>>
    where I: Into<WriteRspVariableIter<'a, ezb_zcl_write_attr_rsp_variable_s>>
    {
        let iter = vars.into();

        iter //r ).into_iter() // Rust note: iterate by references
            .map(|x| Self::parse_one(&x))
            .collect()  // Any entry being 'None' (parsing failed) fails the whole list.
    }

    /**
    * Parse a single success/failure entry, from its 'raw' representation (plain numbers, may carry bad data).
    */
    fn parse_one(entry: &WriteRspVariableEntry) -> Option<Self> {

        let o = match *entry {
            WriteRspVariableEntry::Success => {
                Self::Success
            },
            WriteRspVariableEntry::Failure{ attr_id, status } => {
                let err = ZclError::parse(status)?.unwrap();     // st==0 would be a 'Success'
                Self::Failure(err, AttrId(attr_id))
            }
        };
        Some(o)
    }
}
