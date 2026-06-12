/*
* By providing "views" to the 'Config', we compartmentalize the access but can also shift some of the complexity
* of dealing with the 'esp-zigbee-lib' APIs away from the 'Node' code.
*
* These are all internal details.
*/
mod config_access;
pub(crate) use config_access::ConfigAccess;

//r mod device_descriptor_view;
//r pub use device_descriptor_view::*;

mod endpoint_creator;
pub(crate) use endpoint_creator::EndpointCreator;
