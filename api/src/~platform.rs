//
// Platform: 'include/platform.h'
//
// NOTE: We don't actually need the whole "platform" thing, at all. The focus of the project is
//      solely in 'RadioMode::NATIVE'. Yet, was a good exercise in how to convert the structs. ;)
#![cfg(false)]

use crate::raw::{
    esp_zb_platform_config,
    esp_zb_platform_config_t,
    esp_zb_radio_mode_t,
    esp_zb_radio_config_t,
    esp_zb_host_connection_mode_t,
    esp_zb_uart_config_t,
    esp_zb_host_config_t,
};

/*
* Design: We provide an opaque cover over the 'esp_zb_platform_config_t'. This way, we can gradually
*       provide more functionality if application level needs it.
*/

/* Things we do NOT intend to support (but C code has unconditional):
*   - RCP (radio co-processor)
*/

/**
* Only providing the radio mode, for now.
*/
// typedef struct {
//     esp_zb_radio_mode_t     radio_mode;         /*!< The radio mode */
//     esp_zb_uart_config_t    radio_uart_config;  /*!< The uart configuration to RCP */
// } esp_zb_radio_config_t;
//
pub struct RadioConfig {
    radio_mode: RadioMode,
}

impl Into<esp_zb_radio_config_t> for RadioConfig {
    fn into(self) -> esp_zb_radio_config_t {
        esp_zb_radio_config_t {
            radio_mode: self.radio_mode.into(),
            radio_uart_config: esp_zb_uart_config_t::default()  // dummy
        }
    }
}

/**
* Only providing the "native" radio mode, for now.
*/
// typedef enum {
//     ZB_RADIO_MODE_NATIVE   = 0x0,      /*!< Use the native 15.4 radio */
//     ZB_RADIO_MODE_UART_RCP = 0x1,      /*!< UART connection to a 15.4 capable radio co - processor (RCP) */
// } esp_zb_radio_mode_t;
//
pub enum RadioMode {
    /// Use the native 15.4 radio
    NATIVE,
    /// UART connection to a 15.4 capable radio co - processor (RCP)
    #[cfg(false)]
    UART_RCP
}

impl Default for RadioMode {
    fn default() -> Self {
        RadioMode::NATIVE
    }
}

impl Into<esp_zb_radio_mode_t> for RadioMode {
    fn into(self) -> esp_zb_radio_mode_t {
        match self {
            RadioMode::NATIVE => esp_zb_radio_mode_t::ZB_RADIO_MODE_NATIVE,
            #[cfg(false)]
            RadioMode::UART_RCP => esp_zb_radio_mode_t::ZB_RADIO_MODE_UART_RCP,
        }
    }
}


