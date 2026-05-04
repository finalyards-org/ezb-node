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

// Bring in 'nvs_flash' ESP-IDF C component. This because it allows us to do 1:1 code parity with C examples.
// Was NOT able to find the 'nvs_flash_init()' via 'esp-idf-svc::sys'; that's strange!
//
// Note: We might later do this in the 'esp-idf-svc' way; then we can remove the header here (and the Makefile).
//
#include "nvs_flash.h"
#include "esp_zigbee.h"

// #later
//#include "ezbee/zcl/cluster/ota_upgrade.h"

#include "ezbee/secur.h"

// tbd. #ifdef if 'ep_color_dimmable_light' featured.
//#include "esp_zigbee_ha_standard.h" // 1.x
#include "ezbee/zha.h"

// ColorDimmableLight
//
// Manifest 'ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG()' macro into a 'const', so we can bring it to Rust.
//
// tbd. use '#ifdef' to conditionally bake this in only if Rust 'ep_color_dimmable_light' featured.
//?2.0 const ezb_zha_color_dimmable_light_config_t ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG = ESP_ZB_DEFAULT_COLOR_DIMMABLE_LIGHT_CONFIG();
