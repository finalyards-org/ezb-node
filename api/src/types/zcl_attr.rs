
use std::{mem, slice};

use core::ffi::c_void;

use ezb_node_raw::{
    ezb_zcl_attribute_s,
    ezb_zcl_attribute_s__bindgen_ty_1,
    ezb_zcl_attr_type_e,
};

// ZCL attributes are defined by the protocol specification.
// 'esp_zigbee_lib' has ~58 of them, in 'zcl_type.h'.
//
// We need a Rust-y (higher level) way of dealing with them than what the C 'esp_zigbee_lib'
// has.
//
// We _don't_ need to support all attributes, just the ones actually needed (at any given time). 🔅🔆

// Sample on how C code provides an attribute (bindgen level):
//  <<
//      pub struct ezb_zcl_attribute_s {
//           ///< Attribute identifier.
//           pub id: u16,
//           ///< Attribute data and type information.
//           pub data: ezb_zcl_attribute_s__bindgen_ty_1,
//       }
//       #[repr(C)]
//       #[derive(Debug, Copy, Clone)]
//       pub struct ezb_zcl_attribute_s__bindgen_ty_1 {
//           ///< Attribute data type, see @ref ezb_zcl_attr_type_t.
//           pub type_: u8,
//           ///< Size of the attribute value in bytes.
//           pub size: u16,
//           ///< Pointer to the attribute value buffer. Must be valid for the specified
//           /// size.
//           pub value: *mut ::core::ffi::c_void,
//       }
//  <<

#[derive(Debug, Clone)]
pub struct ZclAttr {
    id: u16,    // standardized; dependent on the cluster
    data: ZclValue
}

impl ZclAttr {
    /**
    * Convert input data to a Rustified struct.
    *
    * @return None if there's a problem in the parsing (details are logged); Some when parsing was possible.
    */
    fn parse(v: &ezb_zcl_attribute_s) -> Option<Self> {
        let o= Self{
            id: v.id,
            data: ZclValue::parse(&v.data)?
        };
        Some(o)
    }
}

#[derive(Debug, Clone)]
pub enum ZclValue {
    // id: u16
    // data.type_: u8
    // data.size: u16
    // data.value: {buffer of 'size' bytes}; untyped

    NoData,
    Data8(u8),
    Data16(u16),
    //...
    String(String),
    //...
    //Eui64(IeeeAddr),
    //...

    ///< Invalid attribute type.
    Unk,
}

impl ZclValue {
    fn parse(v: &ezb_zcl_attribute_s__bindgen_ty_1) -> Option<Self> {
        let ezb_zcl_attribute_s__bindgen_ty_1{
            type_,
            size,
            value
        } = *v;

        // tbd. How are 'NoData' presented? Alternative is to return 'None'.
        if v.value.is_null() {
            if type_ != 0 {
                // One could think this to occur, e.g. on an empty string?
                log::warn!("Null pointer, but type not 'NO_DATA' (skipping): type: {}, size: {}", type_, size);
                return None;
            } else {
                log::debug!("Null pointer, no data (type: {}, size: {})", type_, size);
                return Some(Self::NoData);
            }
        }

        if let Some(tmp_e) = ezb_zcl_attr_type_e::from_repr(type_ as u32) {} else {
            log::error!("[data error] ZCL value type NOT RECOGNIZED by 'esp_zigbee_lib'!: {}", type_);
            return None;
        };

        let happy_cow = match tmp_e {
            ezb_zcl_attr_type_e::EZB_ZCL_ATTR_TYPE_NO_DATA => Self::NoData,
            ezb_zcl_attr_type_e::EZB_ZCL_ATTR_TYPE_DATA8 => {
                let v = read_ptr::<u8>(v.value, v.size)?;
                Self::Data8(v)
            },
            ezb_zcl_attr_type_e::EZB_ZCL_ATTR_TYPE_DATA16 => {
                let v = read_ptr::<u16>(v.value, v.size)?;
                Self::Data16(v)
            }
            ezb_zcl_attr_type_e::EZB_ZCL_ATTR_TYPE_STRING => {
                let p = v.value as *const u8;
                let byte_slice = unsafe { std::slice::from_raw_parts(p, v.size as usize) };
                let s = String::from_utf8_lossy(byte_slice).into_owned();
                Self::String(s)
            },

            //    EZB_ZCL_ATTR_TYPE_DATA8        = 0x08U, /*!< 8-bit data. */
            //     EZB_ZCL_ATTR_TYPE_DATA16       = 0x09U, /*!< 16-bit data. */
            //     EZB_ZCL_ATTR_TYPE_DATA24       = 0x0aU, /*!< 24-bit data. */
            //     EZB_ZCL_ATTR_TYPE_DATA32       = 0x0bU, /*!< 32-bit data. */
            //     EZB_ZCL_ATTR_TYPE_DATA40       = 0x0cU, /*!< 40-bit data. */
            //     EZB_ZCL_ATTR_TYPE_DATA48       = 0x0dU, /*!< 48-bit data. */
            //     EZB_ZCL_ATTR_TYPE_DATA56       = 0x0eU, /*!< 56-bit data. */
            //     EZB_ZCL_ATTR_TYPE_DATA64       = 0x0fU, /*!< 64-bit data. */
            //     EZB_ZCL_ATTR_TYPE_BOOL         = 0x10U, /*!< Boolean type. */
            //     EZB_ZCL_ATTR_TYPE_MAP8         = 0x18U, /*!< 8-bit bitmap. */
            //     EZB_ZCL_ATTR_TYPE_MAP16        = 0x19U, /*!< 16-bit bitmap. */
            //     EZB_ZCL_ATTR_TYPE_MAP24        = 0x1aU, /*!< 24-bit bitmap. */
            //     EZB_ZCL_ATTR_TYPE_MAP32        = 0x1bU, /*!< 32-bit bitmap. */
            //     EZB_ZCL_ATTR_TYPE_MAP40        = 0x1cU, /*!< 40-bit bitmap. */
            //     EZB_ZCL_ATTR_TYPE_MAP48        = 0x1dU, /*!< 48-bit bitmap. */
            //     EZB_ZCL_ATTR_TYPE_MAP56        = 0x1eU, /*!< 56-bit bitmap. */
            //     EZB_ZCL_ATTR_TYPE_MAP64        = 0x1fU, /*!< 64-bit bitmap. */
            //     EZB_ZCL_ATTR_TYPE_UINT8        = 0x20U, /*!< Unsigned 8-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_UINT16       = 0x21U, /*!< Unsigned 16-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_UINT24       = 0x22U, /*!< Unsigned 24-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_UINT32       = 0x23U, /*!< Unsigned 32-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_UINT40       = 0x24U, /*!< Unsigned 40-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_UINT48       = 0x25U, /*!< Unsigned 48-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_UINT56       = 0x26U, /*!< Unsigned 56-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_UINT64       = 0x27U, /*!< Unsigned 64-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_INT8         = 0x28U, /*!< Signed 8-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_INT16        = 0x29U, /*!< Signed 16-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_INT24        = 0x2aU, /*!< Signed 24-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_INT32        = 0x2bU, /*!< Signed 32-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_INT40        = 0x2cU, /*!< Signed 40-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_INT48        = 0x2dU, /*!< Signed 48-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_INT56        = 0x2eU, /*!< Signed 56-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_INT64        = 0x2fU, /*!< Signed 64-bit integer. */
            //     EZB_ZCL_ATTR_TYPE_ENUM8        = 0x30U, /*!< 8-bit enumeration. */
            //     EZB_ZCL_ATTR_TYPE_ENUM16       = 0x31U, /*!< 16-bit enumeration. */
            //     EZB_ZCL_ATTR_TYPE_SEMI         = 0x38U, /*!< Semi-precision floating point. */
            //     EZB_ZCL_ATTR_TYPE_SINGLE       = 0x39U, /*!< Single-precision floating point. */
            //     EZB_ZCL_ATTR_TYPE_DOUBLE       = 0x3aU, /*!< Double-precision floating point. */
            //     EZB_ZCL_ATTR_TYPE_OCTSTR       = 0x41U, /*!< Octet string. */
            //     EZB_ZCL_ATTR_TYPE_STRING       = 0x42U, /*!< Character string. */
            //     EZB_ZCL_ATTR_TYPE_OCTSTR16     = 0x43U, /*!< Long octet string. */
            //     EZB_ZCL_ATTR_TYPE_STRING16     = 0x44U, /*!< Long character string. */
            //     EZB_ZCL_ATTR_TYPE_ARRAY        = 0x48U, /*!< Array. */
            //     EZB_ZCL_ATTR_TYPE_ARRAY_DATA16 = 0x49U, /*!< 16-bit array. */
            //     EZB_ZCL_ATTR_TYPE_ARRAY_DATA32 = 0x4aU, /*!< 32-bit array. */
            //     EZB_ZCL_ATTR_TYPE_STRUCT       = 0x4cU, /*!< Structure. */
            //     EZB_ZCL_ATTR_TYPE_SET          = 0x50U, /*!< Set. */
            //     EZB_ZCL_ATTR_TYPE_BAG          = 0x51U, /*!< Bag. */
            //     EZB_ZCL_ATTR_TYPE_TOD          = 0xe0U, /*!< Time of day. */
            //     EZB_ZCL_ATTR_TYPE_DATE         = 0xe1U, /*!< Date. */
            //     EZB_ZCL_ATTR_TYPE_UTC          = 0xe2U, /*!< UTC time. */
            //     EZB_ZCL_ATTR_TYPE_CLUSTER_ID   = 0xe8U, /*!< Cluster ID. */
            //     EZB_ZCL_ATTR_TYPE_ATTRIBUTE_ID = 0xe9U, /*!< Attribute ID. */
            //     EZB_ZCL_ATTR_TYPE_BAC_OID      = 0xeaU, /*!< BACnet object identifier. */
            //     EZB_ZCL_ATTR_TYPE_EUI64        = 0xf0U, /*!< IEEE address. */
            //     EZB_ZCL_ATTR_TYPE_KEY128       = 0xf1U, /*!< 128-bit security key. */
            //     EZB_ZCL_ATTR_TYPE_INTERNAL     = 0xfeU, /*!< Internal attribute type */

            // note: not sure if this would ever occur in data. Likely it's there so we can internally use such.
            ezb_zcl_attr_type_e::EZB_ZCL_ATTR_TYPE_UNK => Self::Unk,

            x => {
                log::warn!("ZCL attribute type NOT YET IMPLEMENTED (skipped): {}", x);
                return None;
            }
        };
        Some(happy_cow)
    }
}

// Read the ZCL value.
//
// This clones the value; those are normally short and it's useful anyhow for passing the value further.
//
fn read_ptr<T>(p: *const core::ffi::c_void, size: u16) -> Option<T> {

    // Validate the 'size' field matches. Note: NEVER PANIC based on input fields.
    {
        const EXPECTED_SZ: usize = mem::size_of::<T>();
        if size as usize != EXPECTED_SZ {
            log::error!("ZCL data fault: size does not match expectation by the type: {} != {}", size, EXPECTED_SZ);
            return None;
        }
    }

    let p = p as *const T;
    // Use 'read_unaligned' to ensure no alignment problems
    let v = unsafe { std::ptr::read_unaligned(p) };
    Some(v)
}
