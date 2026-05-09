use alloc::collections::BTreeMap;
use crate::config::Config;
use crate::EndpointConfig;
use crate::utils::PascalString;

/**
* A view to 'Config' used in initializing initializing the 'DeviceDescriptor' (= endpoints).
*/
pub(crate) struct DeviceDescriptorView<'a>(&'a Config);

impl<'a> DeviceDescriptorView<'a> {

    pub(crate) fn expand(&self) -> {

        let manufacturer_name: &'static PascalString = self.0.manufacturer_name;
        let model_identifier: &'static PascalString = self.0.model_identifier;

        for ep in self.0.endpoints.values() {
            todo!()
        }
    }

}
