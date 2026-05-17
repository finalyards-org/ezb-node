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
//#include "nvs_flash.h"

#include "esp_zigbee.h"

// #later
//#include "ezbee/zcl/cluster/ota_upgrade.h"

#include "ezbee/secur.h"

#include "ezbee/zha.h"
  // ezb_zha_color_dimmable_light_config_t

// Manifest device type specific configurations (macros in C) to 'const'.
// Further used by 'bindings.rs' to make them 'Default' for said type.
//
const ezb_zha_color_dimmable_light_config_t EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG = EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG();

// Version
//
// Version 2.0 brings 'esp_zigbee_get_version_string()' but that's a function. It's nicer to expose this as a 'const'.
//
#define _STRINGIZE(x) #x
#define _TO_STR(x) _STRINGIZE(x)

const char *ESP_ZIGBEE_VER = \
  _TO_STR(ESP_ZIGBEE_VER_MAJOR) "." \
  _TO_STR(ESP_ZIGBEE_VER_MINOR) "." \
  _TO_STR(ESP_ZIGBEE_VER_PATCH);
  // "2.0.0"
