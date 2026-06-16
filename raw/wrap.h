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
  // ezb_zha_color_dimmer_switch_config_t

// Device type specific defaults
//
// These are presented as macros in the C API. Moving them on turned out to be a challenge (but a solution was found
// via C files and '--wrap-static-fns'). The aim is to allow e.g. '::default()' in Rust to present these (the capital
// words aren't exported).
//
//|1: This did not work; 'bindgen' passes it as 'extern "C"' but there's no way (is there?) to link it.
//|1: const ezb_zha_color_dimmable_light_config_t EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG = EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG();

//|2: Does not create _anything_ in the output. :|
//|2: #define EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG \
//|2:   ((ezb_zha_color_dimmable_light_config_t)EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG())

//|3: Generates 'extern "C"' - and we can do linking via '--wrap-static-fns'.
inline static ezb_zha_color_dimmable_light_config_t wrap_EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG(void) {
    return (ezb_zha_color_dimmable_light_config_t)EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG();
}
inline static ezb_zha_color_dimmer_switch_config_t wrap_EZB_ZHA_COLOR_DIMMER_SWITCH_CONFIG(void) {
    return (ezb_zha_color_dimmer_switch_config_t)EZB_ZHA_COLOR_DIMMER_SWITCH_CONFIG();
}

// Note: Some types have custom initialization macros (like "color dimmable light"), whereas others fall back to the
//      common macro ("color dimmer switch" does). This is kept on the C side, completely.

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
