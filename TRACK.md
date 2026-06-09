# Track


## Cargo: ability to end `.cargo/config.toml` chain

- ["Fine-grain control of Config.toml discovery"](https://github.com/rust-lang/cargo/issues/7887) (`cargo` GitHub)
- ["Cargo config search meta issue"](https://github.com/rust-lang/cargo/issues/9769) (GitHub)

The `config` project is separate from the workspace (for good reason; it does not need ESP32 target). 

However, even when separated, `cargo` keeps looking up the directory chain for `.cargo/config.toml` and overriding that seems tricky (a mild term, perhaps not possible?).

### Solution A.

Could just:

```
- ezb-node			# workspace
- ezb-node-config
```

Solves the problem, but logically `ezb-node-config` is an essential part of `ezb-node`. It deserves to be attached.

### Solution B.

Do not have a top-level `.cargo/config.toml`.

We could do this with symbolic links. Just distribute a file to all `api`, `apps`, `raw` (the workspace sub-folders) *without* it being a top tier `.cargo/config.toml`. Considering.

>Edit: Went with this. Works.


<!-- resolved
## `nightly` vs. `newlib`

- ["Build failing on latest nightly: SIGKILL not found in libc (unix::process)"](https://github.com/esp-rs/esp-idf-sys/issues/419) (GitHub; not the upstream issue)

A mismatch between Rust `nightly` (roughly 2026-05-15), and the `newlib` used in `esp-idf-sys`. 

- [ ] Once there's a fix, remove the `nightly` pin in `rust-toolchain.toml`.
-->


## `esp-zigbee-sdk` ESP-IDF 6.0 compatibility

- ["IDF 6 support"](https://github.com/espressif/esp-zigbee-sdk/issues/816) (GitHub)

	The issue is already closed (and someone reports 2.0.1 to work). However:

	```
	$ rm -rf ~/.espressif
	$ cargo clean
	```
	```
	$ cd raw
	$ cargo build --release -vv --features esp32c6
	[...]
  Cloning into '/home/ubuntu/.espressif/esp-idf/v6.0.1/components/openthread/openthread/third_party/mbedtls/repo'...
  From https://github.com/Mbed-TLS/mbedtls
   * branch            e185d7fd85499c8ce5ca2a54f5cf8fe7dbe3f8df -> FETCH_HEAD
  Submodule 'framework' (https://github.com/Mbed-TLS/mbedtls-framework) registered for path 'components/openthread/openthread/third_party/mbedtls/repo/framework'
  Cloning into '/home/ubuntu/.espressif/esp-idf/v6.0.1/components/openthread/openthread/third_party/mbedtls/repo/framework'...
  From https://github.com/Mbed-TLS/mbedtls-framework
   * branch            457996474728cb8e968ed21953b72f74d2f536b2 -> FETCH_HEAD
  From https://github.com/protobuf-c/protobuf-c
   * branch            abc67a11c6db271bedbb9f58be85d6f4e2ea8389 -> FETCH_HEAD
  From https://github.com/ThrowTheSwitch/Unity
   * branch            bf560290f6020737eafaa8b5cbd2177c3956c03f -> FETCH_HEAD
  Using esp-idf v6.0.1 at '/home/ubuntu/.espressif/esp-idf/v6.0.1'

  thread 'main' (8717) panicked at /home/ubuntu/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/embuild-0.33.1/src/espidf.rs:740:10:
  called `Result::unwrap()` on an `Err` value: unknown variant `win-arm64`, expected one of `linux-i686`, `linux-amd64`, `linux-armel`, `linux-arm64`, `macos`, `macos-arm64`, `win32`, `win64` at line 542 column 23
```	

	- [ ] Which crate is the reason here???
	- [ ] Any issues with "win-arm64" in:
		- [x] `esp-zigbee-sdk` (no)
		- [~] `esp-idf-sys`
		- [ ] `mbedtls`
		- [ ] `embuild`

<!-- earlier
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
-->

<!--
- [ ] Consider opening an issue (maybe only once we have a Rust binding available...).
-->


## `esp-idf-sys`: using `picolib`

- [ ] ["Figure out how to properly support picolibc (IDF 6.0+)"](https://github.com/esp-rs/esp-idf-sys/issues/410)


## `zigbee-rs`

[https://github.com/zigbee-rs/zigbee-rs](https://github.com/zigbee-rs/zigbee-rs)

Being a full-Rust Zigbee stack, this would bridge IEEE [`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio) with Zigbee applications, without the `esp-idf-*` ecosystem. i.e. applications could be bare metal, using the normal `esp-hal` APIs.

>Also, it would take closed source components away from the stack (ZBOSS).

- [ ] Keep an eye on the progress


## Deprecation warnings (with `cargo build -vv`)

- ["Tracking Issue: Used deprecated headers in esp-idf-sys"](https://github.com/esp-rs/esp-idf-sys/issues/312)

