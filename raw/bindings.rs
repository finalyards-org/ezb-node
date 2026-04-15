#![allow(non_camel_case_types)]
#![allow(unused)]
    // Disable warnings of unused entries, and imports in 'tmp/bindings_0.rs'.

/*
* Gathering the bindgen-generated binding like this allows us to attach
* 'Default' (and/or other traits) to its types.
*
* We can see this (converting C uninitialized struct parts to Rust-friendly defaults)
* as being part of the stated aim of "1-to-1 C-Rust" interface.
*/
use core::mem::MaybeUninit;

include!("tmp/bindings_0.rs");

impl Default for esp_zb_platform_config_t {
    fn default() -> Self {
        esp_zb_platform_config_t {
            radio_config: esp_zb_radio_config_t {
                radio_mode: esp_zb_radio_mode_t::ZB_RADIO_MODE_NATIVE,
                radio_uart_config: esp_zb_uart_config_t::nada()
            },
            host_config: esp_zb_host_config_t::default()
        }
    }
}

impl esp_zb_uart_config_t {
    fn nada() -> Self {
        // Not useful setting the random fields; for the tail two we don't even know how to.
        let un = MaybeUninit::zeroed();
        unsafe{ un.assume_init() }

        /***
        uart_config_t {
            baud_rate: 0,
            data_bits: uart_word_length_t::UART_DATA_8_BITS,
            parity: uart_parity_t::UART_PARITY_DISABLE,
            stop_bits: uart_stop_bits_t::UART_STOP_BITS_1,
            flow_ctrl: uart_hw_flowcontrol_t::UART_HW_FLOWCTRL_DISABLE,
            rx_flow_ctrl_thresh: 0,
            __bindgen_anon_1: {
                let un = MaybeUninit::zeroed();
                unsafe{ un.assume_init() }
            },
            flags: {
                let un = MaybeUninit::zeroed();
                unsafe{ un.assume_init() }
            }
            __bindgen_anon_1: uart_config_t__bindgen_ty_1 {
                source_clk: uart_sclk_t::UART_SCLK_DEFAULT
            },
            flags: uart_config_t__bindgen_ty_2 {
                _bitfield_align_1: [],
                _bitfield_1: [],
                __bindgen_padding_0: []
            }
        }***/
    }
}

//  typedef struct {
//      esp_zb_host_connection_mode_t   host_connection_mode;   /*!< The host connection mode */
//      esp_zb_uart_config_t            host_uart_config;       /*!< The uart configuration to host */
//  } esp_zb_host_config_t;
//
impl Default for esp_zb_host_config_t {
    fn default() -> Self {
        esp_zb_host_config_t {
            host_connection_mode: esp_zb_host_connection_mode_t::ZB_HOST_CONNECTION_MODE_NONE,
            host_uart_config: esp_zb_uart_config_t::nada(),
        }
    }
}
