use ezb_node_config::{
    NodeType::*,
};

use ezb_node_raw::{ezb_af_device_desc_t, ezb_af_create_device_desc, ezb_af_device_desc_register, ezb_aps_secur_enable_distributed_security};

use crate::{
    EzbError,
    EzbEspError,
};
use crate::config_views::EndpointCreator;

/// Wrapper on the Zigbee device.
///
/// @note Used as a clarifying abstraction, while the Endpoint configuration does the grunt of
///     initialization.
pub(crate) struct ZigbeeDev(ezb_af_device_desc_t);

impl ZigbeeDev {
    /// Creates the device with configured endpoints.
    /// Registers the device for C library.
    /// Initialize the endpoints.
    ///
    pub(crate) fn new(ep_cfg: EndpointCreator) -> Result<Self, EzbEspError> {
        // ezb_af_device_desc_t dev_desc  = ezb_af_create_device_desc();
        // ESP_ERROR_CHECK(ezb_af_device_desc_register(dev_desc));

        let desc: ezb_af_device_desc_t = unsafe { ezb_af_create_device_desc() };

        // Add the endpoints to the stated device descriptor.
        ep_cfg.create_all(desc)?;

        // Register the device with Zigbee
        EzbError::wrap( unsafe {
            ezb_af_device_desc_register(desc)
        })?;

        // In C examples, this is set to 'false'.
        //
        // We only support centralized trust, i.e. the coordinator (not a router) decides, which devices are let to enter.
        //
        // NOTE!:
        //      - Unknown whether the default is 'false'. Setting because the C examples do.
        //      - tbd. IS THIS IN THE RIGHT PLACE, with regard to device registration etc.?
        //
        let () = unsafe { ezb_aps_secur_enable_distributed_security(false) };

        // Only now, initialize the endpoints.
        ep_cfg.init_all();

        Ok(Self(desc))
    }
}
