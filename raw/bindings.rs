#![allow(non_camel_case_types)]

/*
* Gathering the bindgen-generated binding like this allows us to attach
* 'Default' (and/or other traits) to its types.
*
* We can see this (converting C uninitialized struct parts to Rust-friendly defaults)
* as being part of the stated aim of "1-to-1 C-Rust" interface.
*/
use core::mem::MaybeUninit;

include!("tmp/bindings_0.rs");

impl Default for esp_zb_uart_config_t {
    fn default() -> Self {
        esp_zb_uart_config_t {
            port: uart_port_t::UART_NUM_0,
            rx_pin: gpio_num_t::GPIO_NUM_NC,
            tx_pin: gpio_num_t::GPIO_NUM_NC,
            uart_config: uart_config_t::default()
       }
    }
}

impl Default for uart_config_t {
    fn default() -> Self {
        let un = MaybeUninit::zeroed();
        unsafe{ un.assume_init() }

        // not useful setting the random fields; for the tail two we don't even know how to.
        #[cfg(false)]
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
            /***
            __bindgen_anon_1: uart_config_t__bindgen_ty_1 {
                source_clk: uart_sclk_t::UART_SCLK_DEFAULT
            },
            flags: uart_config_t__bindgen_ty_2 {
                _bitfield_align_1: [],
                _bitfield_1: [],
                __bindgen_padding_0: []
            }***/
        }
    }
}
