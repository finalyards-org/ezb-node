/*
* EndpointConfig
*/
use esp_zb_raw::esp_zb_endpoint_config_t;

pub struct EndpointConfig {
    ep_id: u8,
    app_profile_id: u16,
    app_device_id: u16,
}

impl Into<esp_zb_endpoint_config_t> for EndpointConfig {
    fn into(self) -> esp_zb_endpoint_config_t {

        let mut o: esp_zb_endpoint_config_t = esp_zb_endpoint_config_t::default();
            //
            o.endpoint = self.ep_id;
            o.app_profile_id = self.app_profile_id;
            o.app_device_id = self.app_device_id;
        o
    }
}
