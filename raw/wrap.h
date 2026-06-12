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

#include "esp_zigbee.h"

// #later
//#include "ezbee/zcl/cluster/ota_upgrade.h"

#include "ezbee/secur.h"

#include "ezbee/zha.h"
  // ezb_zha_color_dimmable_light_config_t

// Manifest device type specific configurations (macros in C) to 'const' (or something accessible in Rust!).
// Further used by 'bindings.rs' to make them 'Default' for said type.
//
// This did not work; 'bindgen' passes it as 'extern "C"' but it's in no archive to be linked.
const ezb_zha_color_dimmable_light_config_t EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG = EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG();

// This does not create _anything_ in the output. :|
// tbd. disable 'clang' warning: macro-redefined
//#define EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG \
//  (((ezb_zha_color_dimmable_light_config_t))EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG())

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
  // "2.0.1"
