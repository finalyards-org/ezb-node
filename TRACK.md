# Track


## `esp-zigbee-sdk` ESP-IDF 6.0 compatibility

[https://github.com/espressif/esp-zigbee-sdk](https://github.com/espressif/esp-zigbee-sdk)

- [ ] There is no tracking item for ESP-IDF 6.0 compatibility, which was [released Mar-26](https://github.com/espressif/esp-idf/releases).

Currently (23-Mar-26) build of `esp-zigbee-sdk` examples fails like this (see DEVS/* for how to set up):

**light**

```
/home/xx/Temp/esp-zigbee-sdk/examples/esp_zigbee_HA_sample/HA_color_dimmable_light/managed_components/espressif__led_strip/src/led_strip_rmt_dev.c:11:10: fatal error: driver/rmt_tx.h: No such file or directory
   11 | #include "driver/rmt_tx.h"
      |          ^~~~~~~~~~~~~~~~~
compilation terminated.
```

**switch**

```
In file included from /home/xx/Temp/esp-zigbee-sdk/examples/common/switch_driver/src/switch_driver.c:19:
/home/xx/Temp/esp-zigbee-sdk/examples/common/switch_driver/include/switch_driver.h:17:10: fatal error: driver/gpio.h: No such file or directory
   17 | #include "driver/gpio.h"
      |          ^~~~~~~~~~~~~~~
compilation terminated.
```

<!--
- [ ] Consider opening an issue (maybe only once we have a Rust binding available...).
-->

## `esp-idf-sys` with ESP-IDF 6.0

- [ ] ["Add (some) support for ESP-IDF v6.0"](https://github.com/esp-rs/esp-idf-sys/pull/408)
	

## `zigbee-rs`

[https://github.com/zigbee-rs/zigbee-rs](https://github.com/zigbee-rs/zigbee-rs)

Being a full-Rust Zigbee stack, this would bridge IEEE [`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio) with Zigbee applications, without the `esp-idf-*` ecosystem. i.e. applications could be bare metal, using the normal `esp-hal` APIs.

>Also, it would take closed source components away from the stack (ZBOSS).

- [ ] Keep an eye on the progress
