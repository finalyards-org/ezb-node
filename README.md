# Zigbee on ESP32-C6 with Rust

## Motivation

This repo aims to provide the possibility to create, in Rust, for ESP32(-C6) MCU:

- Zigbee controller
- Zigbee router
- Zigbee custom end device

Wifi ([`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio)) and BLE ([`trouBLE`](https://github.com/embassy-rs/trouble)) are already Rust friendly comms platforms, but for Zigbee there does not seem to be a working solution.

### Focus

This repo aims at *forward looking* development. This means:

- using latest underlying versions of libraries (`esp-zigbee-lib`; ESP-IDF 6.x *once possible*)
- active focus on Zigbee 3.0; interest in Zigbee 4.0
- low or no interest in legacy

Supported devices:

- [ ] Door/window sensor


**Non-focus**

The following Zigbee features are not in the focus of this project:

- Groups
- Scenes
- Green power (harvested energy end devices)
- Touchlink

<!-- tbd. make into a table, with ZCL specifications listed.

support: intended | not planned
status: working, WIP, wish
-->

### Value

With Rust, we can make a whole lot better APIs than with C. Less code. Better IDE support (narrow interfaces instead of everything being flat and global).


## About ESP-IDF

[ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/v6.0.2/esp32c6/get-started/index.html) is an ecosystem, based on the C language SDK of the same name. We need it because the `esp-zigbee-sdk` (currently best supported Zigbee library for the ESP32's) is made using it. What that means for a Rust project is:

- The **application binary** is **tied to the ESP-IDF ecosystem**. The end application must decide (or at least, configure):
	- ESP-IDF version to use
	- other build environment configurations
- FreeRTOS running underneath; the Rust code runs as one of its tasks.
- For peripherals, use `esp-idf-hal`, not `esp-hal`. The latter cannot be used, since it manages peripherals directly (not via an RTOS).

This is elaborate but doable! *Eventually* it would be neat to have a non-idf Zigbee Rust implementation. That would need a Rust native Zigbee stack to be created, e.g. on top of the `esp-radio` IEEE 802.15.4 support.

### esp-idf-sys

Pulls in the whole ESP-IDF SDK, as part of the Rust compilation. It takes time and disk space (some 4..5GB) but is *mostly* a one-time effort. It provides headers and libraries that `esp-zigbee-sdk` relies upon.

### Which ESP-IDF to use?

The author aims at maintaining this towards the *latest stable release*.

>TL;DR In practise, we work with 5.5.x.

The hurdle keeping us away from ESP-IDF 6.x is in `esp-idf-hal`. As indicated [here](https://github.com/esp-rs/esp-idf-hal/issues/595), it gives errors on: `adc, i2s, twai, i2c, ledc, pcnt, rmt, gptimer`.

||||
|---|---|---|
|6.1|latest (8-Sep-26)|`esp-idf-sys` ok; `esp-idf-hal` NOT; `esp-zigbee-sdk` likely not (cannot try)|
|5.5.5||works with slightly modified git `esp-idf-sys`, git `esp-idf-hal`, stock `esp-zigbee-sdk`|

### `esp-idf-sys` is a community effort

Yes. That is a concern, if we bet the whole project on it. But on the other side, let's just drive and see whether the road takes anywhere!!


## Layers

<!-- tbd.
![](.images/two-story-bus.png)

Prompt: "...
-->

### Lower level

`raw` is a 1-to-1 mapping from Rust to the underlying C functions, structs and enums.

We *do not change abstractions* at this level, but we do introduce filtering: only elements needed by the higher API layer are exposed.

We *can* add Rust traits and methods to C-originating structs. This still does not change the abstraction.

### API level

`api`. Here the emphasis is in *providing a Rust native experience*. Abstractions *are* provided. The aim is to *not leak C functions/structures through* - which would limit our future maneuverability for the project's API.

### App level

`apps` contains our example apps. This should provide a template for your own project to emulate.

- `apps/demo/light`
- `apps/demo/switch`


<!-- tbd. an app for working with outside sensor; add when ready!
-->


## Requirements

- A devkit, e.g. [ESP32-C6-DevKitM-1](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html)
- Rust installed
- 10 GB disk space

---

>Recommendation: 
>
>Use Lima VM and [Edge VM](|5.5.5|released|
) (GitHub) for a development environment already geared towards the Rust + ESP-IDF setup. Includes most of the tools listed below.  
>
>In addition (see below):
>
>- bindgen CLI
>- `just` and `jq`

---

- C compilers and `bindgen` CLI

	```
	$ sudo apt install build-essential clang
	[...]

	$ cargo install --locked bindgen-cli
	```

	>Note: DO NOT use the bindgen CLI `apt` package - it lags behind the `cargo` one.

- ESP-IDF requirements:

	>[Espressif docs](https://docs.espressif.com/projects/esp-idf/en/release-v5.5/esp32c6/get-started/linux-macos-setup.html#for-linux-users) lists 15 `apt` dependencies. However, not all of these are really required. 
	
	The author installed:
	
	```
	$ sudo apt install git python3 python3-venv cmake pkg-config
	```

- Note that ESP-IDF **must not be globally installed** - it would mess with the `esp-idf-sys`.

- GNU `make`

	```
	$ sudo apt install make
	```

- `ldproxy`

	We use `ESP_IDF_TOOLS_INSTALL_DIR = "global"` by default, so you'll need:
	
	```
	$ cargo install --locked ldproxy
	```

- `just` and `jq` (optional; for easier demo launch)

	```
	$ sudo apt install just jq
	```

- `espflash` (optional; for flashing the demos)

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


## Preparation

### The `esp_idf_tools_install_dir` config

We're using `global`, which places the ESP-IDF toolchain (about 5.5GB) in `~/.espressif/`. 

This is deemed good, because:

- doing `cargo clean` will not require you to re-download those tools (saves time and power) 

However, since Cargo projects *usually* are confined to do output within their project folders (and `target`, which may have been moved), **it is great to be aware of this**..


<!-- no need to recommend
**Change to `out` (optional)**

You can change the type to `out`. In such a case, the tools are downloaded to within your `target` folder - and cleaned with it.

To change this, edit `.cargo/config.toml`:

```
ESP_IDF_TOOLS_INSTALL_DIR = "out"
```

>Note: The build script (`build.rs`) is not necessarily up to using `out`. You might need to fix that.
-->


## Steps

### Study the source code

- `apps/demo/`

	Sample applications. Note how `sdkconfig.defaults` - the file that defines ESP-IDF build configuration - is part of the application.

- `api`

	The API layer, providing a Rust interface to Zigbee.

- `raw`

	The 1-to-1 C/Rust interface to `esp_zigbee_sdk`, an ESP-IDF C library.

- `config`

	A non-embedded Rust library for turning TOML configuration into Rust. Further simplifies the applications, since declarative configuration is now out of the code. See `apps/demo/**/app.toml`.

---

Many of the subprojects have a soft link to `.cargo/config.espidf.toml`. This allows us to do development in different layers, while keeping changes to the said TOML only in one place. We cannot make a single `.cargo/config.toml` because `config` is not an ESP-IDF project.

---

### Build some (optional)

You can also skip directly to the "demo" section (next). These are useful for understanding the build layers, and for debugging problems in a build.

Note: Each layer has their own `README` with more specific information.


**1. Raw**

```
$ cd raw
$ cargo build --release --features esp32c6
[...]
```

**2. Api**

```
$ cd ../api
```

```
$ just build
```

**3. Apps**

```
$ cd ../apps
```

```
$ just door-build
```

If the builds succeeded, you are ready to run the created binaries on ESP32-C6 devkits.

## Run demos

Have a look at the `apps` folder and run some demos.

Instructions are within [`apps/README.md`](./apps/README.md).


<!-- #later; perhaps do it in `docs/`?
## Using in your own projects

*tbd.*
-->

## Other

### Cleanup

Additional to normal `cargo clean`, there are tools in the `~/.espressif` folder (5..6 GB per each ESP-IDF version you've tried). 

You can wipe the folder (it's a kind of cache for `esp-idf-sys`).

```
$ rm -rf ~/.espressif
```

## References

- [Espressif Zigbee SDK 2.x](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32c6/introduction.html) (Espressif docs; ESP32-C6)

	- "Zigbee Pro R23"
	- "Zigbee Cluster Library (ZCL) v8"

	- [API Reference](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32/api-reference/index.html) (Espressif docs)

<!--
- [Espressif IoT Development Framework](https://github.com/espressif/esp-idf) (GitHub)

	The C API that `esp-zigbee-sdk` builds upon.
-->

- [Partition Tables](https://docs.espressif.com/projects/esp-idf/en/v6.0/esp32c6/api-guides/partition-tables.html) (ESP-IDF API Guides)
