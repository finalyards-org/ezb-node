/*
* By providing "views" to the 'Config', we compartmentalize the access but can also shift some of the complexity
* of dealing with the 'esp-zigbee-lib' APIs away from the 'Node' code.
*
* These are all internal details.
*/
mod platform_device_view;
pub(crate) use platform_device_view::PlatformDeviceView;

mod device_descriptor_view;
pub(crate) use device_descriptor_view::DeviceDescriptorView;
