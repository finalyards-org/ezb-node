
use alloc::{
    boxed::Box,
    string::String
};

//r use esp_idf_sys::error::EspError;

use crate::raw::{
    ezb_af_device_desc_t,
    ezb_af_create_device_desc,
    ezb_af_ep_desc_t,
    ezb_af_endpoint_get_cluster_desc,
    ezb_zcl_basic_cluster_desc_add_attr,
    EZB_ZCL_CLUSTER_SERVER,
    ezb_zcl_basic_server_attr_t::{
        EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID,
        EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID,
    },
    ezb_zcl_cluster_id_e::{
        EZB_ZCL_CLUSTER_ID_BASIC,
    }
};

use crate::utils::PascalString;

/**
* Gathers multiple Application Framework (AF) endpoints and maps the logical device properties to a physical 'Node'.
*
* It acts as a builder for the underlying C endpoint lists, ensuring that clusters and attributes are correctly
* associated before registration with the Zigbee stack.
*/
pub(crate) struct DeviceDescriptor{
    inner: ezb_af_device_desc_t,
    manufacturer_name: &'static PascalString,
    model_info: &'static PascalString,
}

impl DeviceDescriptor {

    pub(crate) fn new(manufacturer_name: String, model_info: String) -> Self {
        let inner = unsafe {
            ezb_af_create_device_desc()
        };
        Self{
            inner,
            manufacturer_name: Box::leak(Box::new(PascalString::from(manufacturer_name))),
            model_info: Box::leak(Box::new(PascalString::from(model_info)),
        }
    }

    /**
    * Add an Application Framework (AF) endpoint. These are endpoints 1..=240.
    */
    // NOTE: Currently this only works for "server"; make that a parameter, once needed. tbd.
    pub(crate) fn add_af_ep<F>(&self, ep_id: u8, gen_: F)
    where
        F: Fn(u8) -> ezb_af_ep_desc_t
    {
        // Create the EP, based on closure we got.
        let ep_desc: ezb_af_ep_desc_t = gen_(ep_id);

        // Modify the 'ep_desc' Basic cluster's contents
        unsafe {
            let h =
                ezb_af_endpoint_get_cluster_desc(ep_desc, EZB_ZCL_CLUSTER_ID_BASIC as u16, EZB_ZCL_CLUSTER_SERVER);
            let err= ezb_zcl_basic_cluster_desc_add_attr(h, EZB_ZCL_ATTR_BASIC_MANUFACTURER_NAME_ID as u16, self.manufacturer_name.as_ptr());
            assert!((err == 0), "Failed to set 'manufacturer name': {}", err);
            //EspError::from(err)?;

            let err = ezb_zcl_basic_cluster_desc_add_attr(h, EZB_ZCL_ATTR_BASIC_MODEL_IDENTIFIER_ID as u16, self.model_info.as_ptr());
            assert!((err == 0), "Failed to set 'model info': {}", err);
            //EspError::from(err)?;
        };
    }

    /**
    * When all end points are provided, close by blurping out the 'inner' handle - used for registration.
    */
    pub(crate) fn expand(self) -> ezb_af_device_desc_t {
        self.inner
    }
}
