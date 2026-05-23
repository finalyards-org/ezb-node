# Health

Is everything fine with the repo?

With Rust projects, this is normally easy. But ESP-IDF building and use of `bindgen` bring some concerns. This file aims to list certain health checks that can occasionally be done - even if things compile - to see they remain optimal.



## Size of `raw/tmp/bindings_0.rs`

```
$ wc -l raw/tmp/bindings_0.rs 
40620 raw/tmp/bindings_0.rs
```

This should **not be tens of thousands** (as pictured).

Having unnecessary *stuff* in the bridging slows the build, but also your IDE. The creation should follow the `--allow*` and `--block*` directives in the `raw/Makefile` and generally not carry APIs not used.

>tbd. Could automatically test in `raw/Makefile` against this.

<!-- #unfinished
### How to debug

```
$ bindgen --version
bindgen 0.72.1
```
-->

## How to know if a build is a success?


```
$ cd raw
$ cargo build --release -vv
[...]
    Finished `release` profile [optimized] target(s) in 2m 56s
```

Well, that gives some assurance.

### Components within the build output

```
[esp-idf-sys 0.37.2] Built components: riscv, esp_driver_gpio, esp_timer, esp_pm, mbedtls, bootloader, esptool_py, partition_table, esp_app_format, esp_bootloader_format, app_update, esp_partition, efuse, bootloader_support, esp_mm, spi_flash, esp_system, esp_common, esp_rom, hal, log, heap, soc, esp_security, esp_hw_support, freertos, newlib, pthread, cxx, esp_ringbuf, esp_driver_pcnt, esp_driver_gptimer, esp_psram, esp_driver_spi, esp_driver_mcpwm, esp_driver_ana_cmpr, esp_driver_i2s, sdmmc, esp_driver_sdmmc, esp_driver_sdspi, esp_driver_sdio, esp_driver_dac, esp_driver_bitscrambler, esp_driver_rmt, esp_driver_tsens, esp_driver_sdm, esp_driver_i2c, esp_driver_uart, esp_driver_ledc, esp_driver_parlio, esp_driver_usb_serial_jtag, esp_driver_twai, driver, wear_levelling, esp_vfs_console, vfs, fatfs, nvs_flash, spiffs, esp_coex, esp_event, lwip, esp_netif_stack, esp_netif, wpa_supplicant, esp_wifi, esp_phy, esp_hal_ieee802154, ieee802154, console, openthread, espressif__esp-zigbee-lib, main
```

Wrote that so one can compare.

### Check the `sdkconfig`

Within the `apps` project:

```
$ cat `just echo-sdkconfig-path` | grep _ZB_
CONFIG_ZB_ENABLED=y
# CONFIG_ZB_SDK_1xx is not set
CONFIG_ZB_ZCZR=y
# CONFIG_ZB_ZED is not set
# CONFIG_ZB_ZGPD is not set
CONFIG_ZB_RADIO_NATIVE=y
# CONFIG_ZB_RADIO_SPINEL_UART is not set
# CONFIG_ZB_DEBUG_MODE is not set
```

This matches with what we like in the `sdkconfig.defaults`.

>Note: The above command is an optional developer tool. Install `just` and `jq` to use it.

