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

#include "ezbee/af.h"
  // EZB_INVALID_AF_EP_DESC

#include "ezbee/zcl/cluster/ias_ace_desc.h"
  // ezb_zcl_ias_ace_cluster_server_config_t

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

// 'esp-zigbee-lib' (2.0.4) defines these as an anonymous enum, leading them to become unrelated constants
// in 'bindgen'. We can give the enum a name (only using the values we need), leading to more bindable C code.
//
// Name is based on 'ezb_zha_device_id_t', which is the u16 alias for them in function parameters.
//
#if 0   //r: done elsewhere (bindings.rs)
typedef enum {
    /* Standard */

    /* Generic Devices */
    // EZB_ZHA_ON_OFF_SWITCH_DEVICE_ID              = 0x0000,
    // EZB_ZHA_LEVEL_CONTROL_SWITCH_DEVICE_ID       = 0x0001,
    // EZB_ZHA_ON_OFF_OUTPUT_DEVICE_ID              = 0x0002,
    // EZB_ZHA_LEVEL_CONTROLLABLE_OUTPUT_DEVICE_ID  = 0x0003,
    // EZB_ZHA_SCENE_SELECTOR_DEVICE_ID             = 0x0004,
    // EZB_ZHA_CONFIGURATION_TOOL_DEVICE_ID         = 0x0005,
    // EZB_ZHA_REMOTE_CONTROL_DEVICE_ID             = 0x0006,
    // EZB_ZHA_COMBINED_INTERFACE_DEVICE_ID         = 0x0007,
    // EZB_ZHA_RANGE_EXTENDER_DEVICE_ID             = 0x0008,
    // EZB_ZHA_MAINS_POWER_OUTLET_DEVICE_ID         = 0x0009,
    // EZB_ZHA_DOOR_LOCK_DEVICE_ID                  = 0x000A,
    // EZB_ZHA_DOOR_LOCK_CONTROLLER_DEVICE_ID       = 0x000B,
    // EZB_ZHA_SIMPLE_SENSOR_DEVICE_ID              = 0x000C,
    // EZB_ZHA_CONSUMPTION_AWARENESS_DEVICE_ID      = 0x000D,
    // EZB_ZHA_HOME_GATEWAY_DEVICE_ID               = 0x0050,
    // EZB_ZHA_SMART_PLUG_DEVICE_ID                 = 0x0051,
    // EZB_ZHA_WHITE_GOODS_DEVICE_ID                = 0x0052,
    // EZB_ZHA_METER_INTERFACE_DEVICE_ID            = 0x0053,

    /* Lighting Devices */
    // EZB_ZHA_ON_OFF_LIGHT_DEVICE_ID               = 0x0100,
    // EZB_ZHA_DIMMABLE_LIGHT_DEVICE_ID             = 0x0101,
    // EZB_ZHA_COLOR_DIMMABLE_LIGHT_DEVICE_ID       = 0x0102,
    // EZB_ZHA_ON_OFF_LIGHT_SWITCH_DEVICE_ID        = 0x0103,
    // EZB_ZHA_DIMMER_SWITCH_DEVICE_ID              = 0x0104,
    // EZB_ZHA_COLOR_DIMMER_SWITCH_DEVICE_ID        = 0x0105,
    // EZB_ZHA_LIGHT_SENSOR_DEVICE_ID               = 0x0106,
    // EZB_ZHA_OCCUPANCY_SENSOR_DEVICE_ID           = 0x0107,

    /* Closures Devices */
    //|EZB_ZHA_SHADE_DEVICE_ID                      = 0x0200,
    //|EZB_ZHA_SHADE_CONTROLLER_DEVICE_ID           = 0x0201,
    //|EZB_ZHA_WINDOW_COVERING_DEVICE_ID            = 0x0202,
    //|EZB_ZHA_WINDOW_COVERING_CONTROLLER_DEVICE_ID = 0x0203,

    /* HVAC Devices */
    //|EZB_ZHA_HEATING_COOLING_UNIT_DEVICE_ID       = 0x0300,
    //|EZB_ZHA_THERMOSTAT_DEVICE_ID                 = 0x0301,
    //|EZB_ZHA_TEMPERATURE_SENSOR_DEVICE_ID         = 0x0302,
    //|EZB_ZHA_PUMP_DEVICE_ID                       = 0x0303,
    //|EZB_ZHA_PUMP_CONTROLLER_DEVICE_ID            = 0x0304,
    //|EZB_ZHA_PRESSURE_SENSOR_DEVICE_ID            = 0x0305,
    //|EZB_ZHA_FLOW_SENSOR_DEVICE_ID                = 0x0306,
    //|EZB_ZHA_MINI_SPLIT_AC_DEVICE_ID              = 0x0307,

    /* Intruder Alarm System */
    IAS_CONTROL_INDICATING_EQUIPMENT_ID  = 0x0400 //EZB_ZHA_IAS_CONTROL_INDICATING_EQUIPMENT_ID,
    //|EZB_ZHA_IAS_ANCILLARY_CONTROL_EQUIPMENT_ID   = 0x0401,
    //|EZB_ZHA_IAS_ZONE_ID                          = 0x0402,
    //|EZB_ZHA_IAS_WARNING_DEVICE_ID                = 0x0403,

    /* Custom */
    //|EZB_ZHA_CUSTOM_GATEWAY_DEVICE_ID = 0xff00,
} ezb_zha_device_id_e;
#endif

#ifndef EZB_INVALID_AF_EP_DESC
# error "Something's wrong"
#endif
