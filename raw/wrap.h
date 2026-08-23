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
  // ezb_zha_device_id_t (and its enum values)

#include "ezbee/zdo.h"
  // zdo_dev_srv_disc.h: service discovery

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
#if 1   // retire these at some point; no longer used (2.0 API change implications; not trusting their device types)
inline static ezb_zha_color_dimmable_light_config_t wrap_EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG(void) {
    return (ezb_zha_color_dimmable_light_config_t)EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG();
}

// NOTE: Both 'ezb_zha_color_dimmer_switch_config_t' and 'ezb_zha_configuration_tool_config_t' (and maybe some more?)
//      are typedef's to the same 'ezb_zha_common_device_config_t'.
//
//      Also, it means their '..._CONFIG()' macros are aliases of 'EZB_ZHA_COMMON_DEVICE_CONFIG()'.
//
//      We do NOT need to know about this here. But once we assign '::default()' to these types, it won't work
//      (multiple implementations).
//
inline static ezb_zha_color_dimmer_switch_config_t wrap_EZB_ZHA_COLOR_DIMMER_SWITCH_CONFIG(void) {
    return (ezb_zha_color_dimmer_switch_config_t)EZB_ZHA_COLOR_DIMMER_SWITCH_CONFIG();
}
//inline static ezb_zha_configuration_tool_config_t wrap_EZB_ZHA_CONFIGURATION_TOOL_CONFIG(void) {
//    return (ezb_zha_configuration_tool_config_t)EZB_ZHA_CONFIGURATION_TOOL_CONFIG();
//}
#endif

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

// Without this, not all 'EZB_ERR_*' values make it to the Rust side.
// There is no such 'ezb_err_e' in the C API, though 'ezb_err_t' is used. Range is 0..=19, and the -1.
// Value range is
typedef enum {
    NONE           = EZB_ERR_NONE,  // 0
    FAIL           = EZB_ERR_FAIL,  // -1
    NO_MEM         = EZB_ERR_NO_MEM,
    INV_ARG        = EZB_ERR_INV_ARG,
    INV_STATE      = EZB_ERR_INV_STATE,
    INV_SIZE       = EZB_ERR_INV_SIZE,
    NOT_FOUND      = EZB_ERR_NOT_FOUND,
    NOT_SUPPORTED  = EZB_ERR_NOT_SUPPORTED,
    TIMEOUT        = EZB_ERR_TIMEOUT,
    ABORT          = EZB_ERR_ABORT,
    BUSY           = EZB_ERR_BUSY,
    NOT_FINISHED   = EZB_ERR_NOT_FINISHED,
    NOT_ALLOWED    = EZB_ERR_NOT_ALLOWED,
    PARSE          = EZB_ERR_PARSE,
    EMPTY_DATA     = EZB_ERR_EMPTY_DATA,
    DROP           = EZB_ERR_DROP,
    SECURITY       = EZB_ERR_SECURITY,
} ezb_err_e;
