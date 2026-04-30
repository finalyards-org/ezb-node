# Zigbee on ESP32 with Rust

## Motivation

This repo aims to provide the possibility to create, in Rust, for ESP32 MCUs:

- Zigbee controller
- Zigbee router
- Zigbee custom end device

Wifi ([`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio)) and BLE ([`trouBLE`](https://github.com/embassy-rs/trouble)) are already Rust friendly comms platforms, but for Zigbee there does not seem to be a working solution.

### Focus

This repo aims at *forward looking* development. This means:

- using latest underlying versions of libraries (yes, we know `esp-zigbee-lib` 2.0 is out)
- strong interest in Zigbee 4.0
- low interest in legacy

Likely *only some ZCL profiles* will be supported. <!--tbd. list here-->

### Value

With Rust, we can make a whole lot better APIs than with C. Less code. Better IDE support.


## About ESP-IDF

ESP-IDF is an ecosystem, based on the C language SDK of the same name. We need it because the `esp-zigbee-sdk` (currently best supported Zigbee library for the ESP32's) is made using it. What that means for a Rust project is:

- The **application binary** is **tied to the ESP-IDF ecosystem**. The end application must decide (or at least, configure):
	- ESP-IDF version to use
	- other build environment configurations

	>This is unfortunate, but unavoidable. Until the time there is a Rust Zigbee stack (the `esp-zigbee-sdk` uses ZBoss which closed source - the *whole* reason we need to jump rope with ESP-IDF in the first place). 

- FreeRTOS running underneath; the Rust code runs as one of its tasks.
- For peripherals, use `esp-idf-hal`, not `esp-hal`. The latter cannot be used, since it manages peripherals directly (not having/needing an RTOS).

This should be doable. *Eventually* it would be neat to have a non-idf Zigbee Rust implementation. That would need a Rust native Zigbee stack to be created, e.g. on top of the `esp-radio` IEEE 802.15.4 low layers.

### esp-idf-sys

Pulls in the whole ESP-IDF SDK, as part of the Rust compilation. It takes time and disk space (some 4-5GB) but is *mostly* a one-time effort. It provides headers and libraries that `esp-zigbee-sdk` relies upon.

### Which ESP-IDF to use?

The author aims at maintaining this towards the *latest stable release*, until ESP-IDF would no longer be required.

ESP-IDF 6.0 was released during the development, but is not yet (Mar'26) supported by `esp-idf-sys` and `esp-zigbee-sdk`. Once it is, the change to ESP-IDF 6.0 would be taken.

|||
|---|---|
|6.0|released; not supported by `esp-idf-sys`, `esp-zigbee-sdk`, <u>yet</u>.|
|5.5.3|default of `esp-idf-svc`<sub>[link](https://github.com/esp-rs/esp-idf-svc/blob/master/.cargo/config.toml#L10)</sub>; the version we use|
|5.3.2|recommended by `esp-zigbee-sdk` (11-Mar-26); but it works with 5.5.3|

### `esp-idf-sys` is a community effort

Yes. That is a concern, if we bet the whole project on it. But on the other side, let's just drive and see whether the road takes anywhere!!

We won't need `esp-idf-svc`, for example.

<!-- r??
## Folder structure

|||
|---|---|
|`examples`|You may start here - how to use the Rust API.|
|`x`|The Rust API level|
|`raw`|Bridging of `esp-zigbee-sdk` C API's to Rust|
-->

## Two layers

<!-- tbd.
![](.images/two-story-bus.png)

Prompt: "...
-->

### Lower level

`raw` aims to be a 1-to-1 mapping from Rust to the underlying C functions, structs and enums.

We *do not change abstractions* at this level, but we do introduce filtering: only elements needed by the higher API layer are exposed.

### API level

`x` (or `_`) is the API layer. Here the emphasis is in *providing a Rust native experience*. Abstractions *are* provided. The aim is to *not leak C functions/structures through* - which would limit our future maneuverability for the project's API.


## Requirements

- ESP32-C6 devkit
- Rust installed
	<!-- tbd. give instructions here that show the right toolchain etc.-->
- ~7 GB disk space
- `bindgen` CLI

	```
	$ cargo install --locked bindgen-cli
	```
	
	>Note: DO NOT use the `apt` package - it lags behind (0.66 vs. 0.72.1 at the time of writing).

- ESP-IDF requirements:

	["For Linux Users" (v5.5.3)](https://docs.espressif.com/projects/esp-idf/en/v5.5.3/esp32c6/get-started/linux-macos-setup.html#for-linux-users) (Espressif docs) lists 15 `apt` dependencies. 
	
	However, not all of these are really required. The author has these installed:
	
	```
	$ sudo apt install git wget python3 python3-venv cmake libssl-dev libusb-1.0-0 pkg-config
	```

- Note that ESP-IDF **may not be globally installed** - it would mess with the `esp-idf-sys`.

- GNU `make`

	```
	$ sudo apt install make
	```

- `ldproxy`

	We use `ESP_IDF_TOOLS_INSTALL_DIR = "global"` by default, so you'll need:
	
	```
	$ cargo install --locked ldproxy
	```

- `espflash` (optional)

	```
	$ cargo install --locked espflash
	```

	```
	$ espflash board-info
	[...]
	Chip type:         esp32c6 (revision v0.2)
	Crystal frequency: 40 MHz
	Flash size:        4MB
	Features:          WiFi 6, BT 5
	[...]
	```

	You'll need `espflash` to run the examples on ESP32-C6 devkits.

<!--
Developed with:
- bindgen 0.72.1
- ldproxy 0.3.4
- espflash 4.3.0
-->

## Preparation

### The `esp_idf_tools_install_dir` config

We're using `global`, which places the ESP-IDF toolchain (about 5.5GB) in `~/.espressif/`. 

This is deemed good, because:

- doing `cargo clean` will not require you to re-download those tools (saves time and power) 

However, since Cargo projects *usually* are confined to do output within their project folders (and `target`, which may have been moved), **it is great to be aware of this**..


**Change to `out` (optional)**

You can change the type to `out`. In such a case, the tools are downloaded to within your `target` folder - and cleaned with it.

To change this, edit `.cargo/config.toml`:

```
ESP_IDF_TOOLS_INSTALL_DIR = "out"
```


<!-- hide
**Out - clears tooling with `cargo clean`**

Places the ESP-IDF tooling in the Cargo output folder. This means `cargo clean` would remove not only build output, but the tooling as well. This can lead to extra delays and downloads, if you do `cargo clean` repeatedly.

**Global - keeps tooling separate**

This uses the `~/.espressif` folder, **outside of what normally gets written to** when doing a Cargo build. That may be surprising.

If you wish to use `global` mode, you also need to:

- install the `ldproxy` CLI (`cargo install ldproxy`)
- have this section in `.cargo/config.toml`:

	```
	[target.'cfg(target_os = "espidf")']
	linker = "ldproxy"
	```

**Default (neither defined)**

This pulls the ESP-IDF tooling to `.embuild/espressif` **within your project folder**. 

You can do this, but the author uses Multipass VM and pulling in 5GB of tooling to a shared folder slows things down. So he opts for either `out` or `global`, avoiding that problem.

**All options work**

It does not *really* matter, which option you choose. They all work.
-->


## Source code

Study the source code:

- `x`

	The API layer, providing a Rust interface to Zigbee.

- `raw`

	The 1-to-1 C/Rust interface to `esp_zigbee_sdk`, an ESP-IDF C library.

Build some examples that we'll use in the next section (demo).

```
$ cargo apps:light:build
[...]
```

```
$ cargo apps:switch:build
[...]
```

>Note: To see the longer commands, check out `.cargo/config.toml`.

If the builds succeeded, you are ready to run the created binaries on ESP32-C6 devkits.

## Demos

See [`docs/DEMO.md`](docs/DEMO.md) for instructions on how to run the demos:

- 1. Light bulb / switch demo with two ESP32-C6's
	- 1a. with a commercial light bulb
	- 1b. with a commercial switch


<!-- #later; perhaps do it in `docs/`?
## Using in your own projects

*tbd.*
-->

<!--
## Cleanup

Additional to normal cleanup (`cargo clean`) - if you kept the `"global"` build setting (see above):

```
$ rm -rf ~/.espressif
```
-->

## References

- [ESP Zigbee SDK Programming Guide](https://docs.espressif.com/projects/esp-zigbee-sdk) (Espressif docs)

	- [API Reference](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32/api-reference/index.html) (Espressif docs)

- [Espressif IoT Development Framework](https://github.com/espressif/esp-idf) (GitHub)

	The C API that `esp-zigbee-sdk` builds upon.

- [Partition Tables](https://docs.espressif.com/projects/esp-idf/en/v6.0/esp32c6/api-guides/partition-tables.html) (ESP-IDF API Guides)
