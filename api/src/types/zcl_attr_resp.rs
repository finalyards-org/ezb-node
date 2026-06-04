
use ezb_node_raw::{
    RspVariableIter,
    RspVariableEntry,
};

use crate::{
    ZclAttr,
    ZclError,
    AttrId,
};

// ezb_zcl_write_attr_rsp_variable_t
#[derive(Debug, Clone)]
pub enum ZclAttrResp {
    Success(ZclAttr),
    Failure(AttrId, ZclError)
}

impl ZclAttrResp {
    /**
    * Parse a list of successful or failed variable responses.
    *
    * @return None if there is a problem (bad data, or internal problems logged); Some if conversion took place.
    */
    // note: Raw level has provided a unified iterator for us, but it has not touched the
    //      contents. It's up to us to turn the 'u8'/'u16's into more application friendly types.
    //
    //pub struct RspVariableEntry {
    //     pub attr_id: u16,
    //     pub status: u8,
    //     pub attr_type: u8,
    //     pub attr_value: *mut ::core::ffi::c_void
    // }
    pub(crate) fn parse_list(iter: RspVariableIter) -> Option<Vec<Self>> {

        iter.map(|RspVariableEntry{ attr_id, status, attr_type, attr_value }| {

            let st = ZclError::parse(status)?;

            match st {
                None => {
                    let attr = ZclAttr::parse2(entry)?;
                    Some(ZclAttrResp::Success(attr))
                }
                Some(err) => Some(ZclAttrResp::Failure(AttrId(entry.attr_id), err)),
            }
        })
            .flatten() // Siivoaa mahdolliset None-arvot pois, jos parse2 epäonnistuu
            .collect();
    }
}



#[cfg(false)]  //R; took the iteration approach
impl ZclAttrResp {
    //pub struct ezb_zcl_read_attr_rsp_variable_s {
    //     ///< Attribute ID that was read.
    //     pub attr_id: u16,
    //     ///< Status of the read operation. See @ref ezb_zcl_status_t.
    //     pub status: u8,
    //     ///< Data type of the attribute. See @ref ezb_zcl_attr_type_t. Only valid
    //     /// if status is SUCCESS.
    //     pub attr_type: u8,
    //     ///< Pointer to the attribute value buffer. Only valid if status is
    //     /// SUCCESS.
    //     pub attr_value: *mut ::core::ffi::c_void,
    //     ///< Pointer to the next variable in the response list, or NULL if last.
    //     pub next: *mut ezb_zcl_read_attr_rsp_variable_s,
    // }
    fn parse_next<T>(o: &T) -> Option<(ZclAttrResp,*const T)> {
        let T {
            attr_id,
            status,
            attr_type,
            attr_value,
            next
        } = *o;

        let entry = match ZclError::from(status) {
            None => {
                let attr = ZclAttr::parse2(attr_id, attr_type, attr_value)?;
                ZclAttrResp::Success(attr)
            },
            Some(err) => {
                // note: 'attr_type', 'attr_value' not to be used
                ZclAttrResp::Failure(AttrId(attr_id), err)
            }
        };
        Some((entry,next))
    }

    /**
    * Parse a list of responses on variable access (read/write/...).
    */
    pub(crate) fn parse_list<T>(raw: &T) -> Option<Vec<ZclAttrResp>> {
        let mut vec = Vec::new();

        let mut next: &T = raw;
        loop {
            let Some((entry,next)) = Self::parse_next(next) else {
                log::error!("Parsing attribute responses failed!");
                return None;
            };
            vec.push(entry);

            if next.is_null() { break; }
            todo!()
        }
        Some(vec)
    }
}
