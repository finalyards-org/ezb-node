/*
* Master header bringing in the C side headers we want to bind to.
*
* This also allows adding some C-side tunings:
*   - anchoring elusive '#define's to 'const' - which are visible to 'bindgen'.
*
* Usage:
*   <<
*   $ bindgen wrap.h
*   <<
*/
#pragma once

// Limit warnings in the 'cargo build -vv' output
//  - Unfortunately, this is not where those warnings come from. It's likely ESP-IDF compilation.
//#pragma clang diagnostic ignored "-Wexpansion-to-defined"

#include "esp_zigbee.h"

// #later
//#include "ezbee/zcl/cluster/ota_upgrade.h"

#include "ezbee/secur.h"

// tbd. #ifdef if 'ep_color_dimmable_light' featured.
//#include "esp_zigbee_ha_standard.h" // 1.x
#include "ezbee/zha.h"

//R#include "esp_zigbee_version.h"
    // const char *esp_zigbee_get_version_string(void)

// ColorDimmableLight
//
// Manifest 'ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG()' macro into a 'const', so we can bring it to Rust.
//
// tbd. use '#ifdef' to conditionally bake this in only if Rust 'ep_color_dimmable_light' featured.
//?2.0 const ezb_zha_color_dimmable_light_config_t ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG = ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG();
