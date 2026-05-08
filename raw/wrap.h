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

// Manifest device type specific configurations (macros in C) to 'const'.
// Further used by 'bindings.rs' to make them 'Default' for said type.
//
const ezb_zha_color_dimmable_light_config_t EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG = EZB_ZHA_COLOR_DIMMABLE_LIGHT_CONFIG();

//R // Force these to be 'u8'; in C header they are uncasted and thus end up in Rust as 'u32',
//R // though C function parameters for them take 'uint8_t'.
//R //
//R #define EZB_ZCL_CLUSTER_SERVER ((uint8_t)EZB_ZCL_CLUSTER_SERVER) // 1
//R #define EZB_ZCL_CLUSTER_CLIENT ((uint8_t)EZB_ZCL_CLUSTER_CLIENT) // 2
