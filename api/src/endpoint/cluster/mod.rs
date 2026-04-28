/*
*
*/

mod basic;
mod color_dimmable_light;

/*
* 'AnyCluster' is a wrap around 'esp_zigbee_lib's 'esp_zb_attribute_list_t' - a list of attributes.
*
* Rust traits can add their own methods above this, to support specific clusters.
*/

pub struct AnyCluster(*mut esp_zb_attribute_list_t);

