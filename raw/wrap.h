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

#include "esp_zigbee_core.h"
    // brings in the rest

#include "esp_zigbee_attribute.h"
#include "esp_zigbee_cluster.h"
#include "esp_zigbee_endpoint.h"
//#include "esp_zigbee_ota.h"
//#include "esp_zigbee_secur.h"
//#include "esp_zigbee_trace.h"
#include "esp_zigbee_type.h"

#include "esp_zigbee_version.h"
    // #define ESP_ZB_VER_MAJOR 1
    // #define ESP_ZB_VER_MINOR 6
    // #define ESP_ZB_VER_PATCH 8

#define _STRINGIZE(x) #x
#define _TO_STR(x) _STRINGIZE(x)

const char* ESP_ZB_VER_STR = \
  _TO_STR(ESP_ZB_VER_MAJOR) "." \
  _TO_STR(ESP_ZB_VER_MINOR) "." \
  _TO_STR(ESP_ZB_VER_PATCH); // "1.6.8"
