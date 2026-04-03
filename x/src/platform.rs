//
// Platform: 'include/platform.h'
//

use core::mem::zeroed;

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

pub struct PlatformConfig(
    esp_zb_platform_config_t
);

impl Default for PlatformConfig {
    /*
    * These defaults are based on the "Light bulb" and "Light switch" examples (of 'esp-zigbee-sdk').
    * In C, the defaults are application specific.
    *
    * Idea: We could employ a builder approach, where using the '::default()' as such is outlawed.
    *       THis would reflect the C side well - needing every application to define their own, e.g.
    *           <<
    *               PlatformConfig::default()
    *                   .withRadioMode(NATIVE)
    *                   .withConnectionMode(NONE)   // now the build would be allowed to be used
    *           <<
    */
    fn default() -> Self {
        let o = esp_zb_platform_config_t {
            radio_config: esp_zb_radio_config_t {
                radio_mode: esp_zb_radio_mode_t::ZB_RADIO_MODE_NATIVE,
                radio_uart_config: uart_empty()
            },
            host_config: esp_zb_host_config_t {
                host_connection_mode: esp_zb_host_connection_mode_t::ZB_HOST_CONNECTION_MODE_NONE,
                host_uart_config: uart_empty()
            }
        };
        Self(o)
    }
}

impl Into<esp_zb_platform_config_t> for PlatformConfig {
    fn into(self) -> esp_zb_platform_config_t {
        self.0
    }
}

/*
* UART configuration fields when not in use.
*/
fn uart_empty() -> esp_zb_uart_config_t {
    unsafe { zeroed() }
}

/***
/*
* Only providing the "native" radio mode, for now.
*
* C: <<
*       typedef enum {
*           ZB_RADIO_MODE_NATIVE   = 0x0,      /*!< Use the native 15.4 radio */
*           ZB_RADIO_MODE_UART_RCP = 0x1,      /*!< UART connection to a 15.4 capable radio co - processor (RCP) */
*       } esp_zb_radio_mode_t;
*   <<
*/
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

/*
* Only providing the "NONE" host connection mode, for now.
*
* C: <<
*       typedef enum {
*           ZB_HOST_CONNECTION_MODE_NONE       = 0x0, /*!< Disable host connection */
*           ZB_HOST_CONNECTION_MODE_CLI_UART   = 0x1, /*!< CLI UART connection to the host */
*           ZB_HOST_CONNECTION_MODE_RCP_UART   = 0x2, /*!< RCP UART connection to the host */
*       } esp_zb_host_connection_mode_t;
*   <<
*/
pub enum HostConnectionMode {
    /// Disable host connection
    NONE,
    /// CLI UART connection to the host
    #[cfg(false)]
    CLI_UART,
    /// RCP UART connection to the host
    #[cfg(false)]
    RCP_UART,
}

impl Default for HostConnectionMode {
    fn default() -> Self {
        HostConnectionMode::NONE
    }
}

impl Into<esp_zb_host_connection_mode_t> for HostConnectionMode {
    fn into(self) -> esp_zb_host_connection_mode_t {
        match self {
            HostConnectionMode::NONE => esp_zb_host_connection_mode_t::ZB_HOST_CONNECTION_MODE_NONE,
            #[cfg(false)]
            HostConnectionMode::CLI_UART => esp_zb_host_connection_mode_t::ZB_HOST_CONNECTION_MODE_CLI_UART,
            #[cfg(false)]
            HostConnectionMode::RCP_UART => esp_zb_host_connection_mode_t::ZB_HOST_CONNECTION_MODE_RCP_UART,
        }
    }
}
***/
