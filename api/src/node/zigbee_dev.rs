//
// Task: Zigbee task
//
use ezb_node_config::{
    NodeType::*,
};

use ezb_node_raw::{
    ezb_af_device_desc_t,
    ezb_af_create_device_desc,
    ezb_af_device_desc_register,
    ezb_aps_secur_enable_distributed_security
};

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
    /// Creates the device descriptor (C library term) and initializes its endpoints.
    ///
    /// @note In the C examples, this corresponds with 'esp_zigbee_create_zha_{...}_device()'.
    ///
    pub(crate) fn new(ep_cfg: EndpointCreator) -> Result<Self, EzbEspError> {
        // ezb_af_device_desc_t dev_desc  = ezb_af_create_device_desc();
        // ...
        //      bunch of endpoint creation, see within 'EndpointCreator::create_all'
        // ...
        // ESP_ERROR_CHECK(ezb_af_device_desc_register(dev_desc));

        let dev_desc: ezb_af_device_desc_t = unsafe { ezb_af_create_device_desc() };

        // Add the endpoints to the stated device descriptor.
        ep_cfg.create_all_c(dev_desc)?;

        EzbError::wrap( unsafe {
            ezb_af_device_desc_register(dev_desc)
        })?;

        // Only now, initialize the endpoints.
        //
        // Note: If this can be called prior to registering the device, we should merge it to the
        //      '.create_all_c()' above. tbd. #ask
        //
        ep_cfg.init_additional_clusters_c();

        Ok(Self(dev_desc))
    }
}
