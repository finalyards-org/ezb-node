#![cfg(feature = "ep_color_dimmable_light")]

use crate::raw::{
    esp_zb_color_dimmable_light_ep_create,
    esp_zb_color_dimmable_light_cfg_t,
    esp_zb_cluster_list_t,
    esp_zb_endpoint_config_t,
    esp_zb_color_dimmable_light_clusters_create,
    ESP_ZB_AF_HA_PROFILE_ID,
    ESP_ZB_HA_COLOR_DIMMABLE_LIGHT_DEVICE_ID,
};

use super::Profile;

#[allow(non_camel_case_types)]
pub struct ColorDimmableLight(*mut esp_zb_cluster_list_t);

impl ColorDimmableLight {
    fn new(/*cfg: &ColorDimmableLightConfig*/) -> Self {

        let mut light_cfg = esp_zb_color_dimmable_light_cfg_t::default();
        let cluster_list = unsafe {
            esp_zb_color_dimmable_light_clusters_create(&mut light_cfg)
        };
            // "This contains basic, identify, groups, scenes, on-off, level, color cluster as server side."

        Self(cluster_list)
    }

}

impl Profile<{ ESP_ZB_AF_HA_PROFILE_ID }, { ESP_ZB_HA_COLOR_DIMMABLE_LIGHT_DEVICE_ID }> for ColorDimmableLight {}

/**
* @brief Zigbee HA standard color dimmable light device config value.
*/
//  C defaults:
//  {
//       .basic_cfg =
//           {
//               .zcl_version = ESP_ZB_ZCL_BASIC_ZCL_VERSION_DEFAULT_VALUE,
//               .power_source = ESP_ZB_ZCL_BASIC_POWER_SOURCE_DEFAULT_VALUE,
//           },
//       .identify_cfg =
//           {
//               .identify_time = ESP_ZB_ZCL_IDENTIFY_IDENTIFY_TIME_DEFAULT_VALUE,
//           },
//       .groups_cfg =
//           {
//               .groups_name_support_id = ESP_ZB_ZCL_GROUPS_NAME_SUPPORT_DEFAULT_VALUE,
//           },
//       .scenes_cfg =
//           {
//               .scenes_count = ESP_ZB_ZCL_SCENES_SCENE_COUNT_DEFAULT_VALUE,
//               .current_scene = ESP_ZB_ZCL_SCENES_CURRENT_SCENE_DEFAULT_VALUE,
//               .current_group = ESP_ZB_ZCL_SCENES_CURRENT_GROUP_DEFAULT_VALUE,
//               .scene_valid = ESP_ZB_ZCL_SCENES_SCENE_VALID_DEFAULT_VALUE,
//               .name_support = ESP_ZB_ZCL_SCENES_NAME_SUPPORT_DEFAULT_VALUE,
//           },
//       .on_off_cfg =
//           {
//               .on_off = ESP_ZB_ZCL_ON_OFF_ON_OFF_DEFAULT_VALUE,
//           },
//       .level_cfg =
//           {
//               .current_level = ESP_ZB_ZCL_LEVEL_CONTROL_CURRENT_LEVEL_DEFAULT_VALUE,
//           },
//       .color_cfg =
//           {
//               .current_x = ESP_ZB_ZCL_COLOR_CONTROL_CURRENT_X_DEF_VALUE,
//               .current_y = ESP_ZB_ZCL_COLOR_CONTROL_CURRENT_Y_DEF_VALUE,
//               .color_mode = ESP_ZB_ZCL_COLOR_CONTROL_COLOR_MODE_DEFAULT_VALUE,
//               .options = ESP_ZB_ZCL_COLOR_CONTROL_OPTIONS_DEFAULT_VALUE,
//               .enhanced_color_mode = ESP_ZB_ZCL_COLOR_CONTROL_ENHANCED_COLOR_MODE_DEFAULT_VALUE,
//               .color_capabilities = 0x0008,
//           },
pub struct ColorDimmableLightConfig {
    // deviations from default here (if any)
}

impl Default for ColorDimmableLightConfig {
    fn default() -> Self { Self{} }
}

impl Into<esp_zb_color_dimmable_light_cfg_t> for ColorDimmableLightConfig {
    fn into(self) -> esp_zb_color_dimmable_light_cfg_t {
        let mut x = esp_zb_color_dimmable_light_cfg_t::default();

        // ..could modify the fields here
        x.into()
    }
}

