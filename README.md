# Zigbee on ESP32 with Rust

## Motivation

This repo aims to provide the possibility to create, in Rust, for ESP32 MCUs:

- Zigbee controller
- Zigbee router
- Zigbee custom end device

Wifi ([`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio)) and BLE ([`trouBLE`](https://github.com/embassy-rs/trouble)) are already Rust friendly comms platforms.


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

### `esp-idf-{sys|svc}` are community efforts!

Yes. That is a concern, if we bet the whole project on it. But on the other side, let's just drive and see whether the road takes anywhere!!

<!-- r??
## Folder structure

|||
|---|---|
|`examples`|You may start here - how to use the Rust API.|
|`x`|The Rust API level|
|`raw`|Bridging of `esp-zigbee-sdk` C API's to Rust|
-->

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
	$ sudo apt install git wget python3 cmake libssl-dev libusb-1.0-0
	```

- Note that ESP-IDF **may not be globally installed** - it would mess with the `esp-idf-sys`.

- GNU `make`

	```
	$ sudo apt install make
	```

- `ldproxy` (optional)

	If you use `ESP_IDF_TOOLS_INSTALL_DIR = "global"` (described elsewhere), you'll need:
	
	```
	$ cargo install --locked ldproxy
	```
	
<!--
Developed with:
- bindgen 0.72.1
- ldproxy 0.3.4
-->

## Preparation

<!-- hidden; now happens automatically if the person has 'git' available.
### Submodules

Load git submodules - we get the C sources that way.

```
$ git submodule update --init
```
-->


### The `esp_idf_tools_install_dir` config

We better discuss this right up here.

You have two choices of selecting where `esp-idf-sys` (that we use) places the ESP-IDF tooling. Since this involves downloading some GB's, it's best to discuss it before the build.

Within the `.cargo/config.toml` and/or `raw/.cargo/config.toml` you'll find:

```
#ESP_IDF_TOOLS_INSTALL_DIR = "out"
ESP_IDF_TOOLS_INSTALL_DIR = "global"
```

**Out - clears tooling with `cargo clean`**

Places the ESP-IDF tooling in the Cargo output folder. This means `cargo clean` would remove not only build output, but the tooling as well. This can lead to extra delays and downloads, if you do `cargo clean` repeatedly.

>This is the setup for the project at large (in effect when you build `_`).

**Global - keeps tooling separate**

This uses the `~/.espressif` folder, **outside of what normally gets written to** when doing a Cargo build. That may be surprising.

If you wish to use `global` mode, you also need to:

- install the `ldproxy` CLI (`cargo install ldproxy`)
- have this section in `.cargo/config.toml`:

	```
	[target.'cfg(target_os = "espidf")']
	linker = "ldproxy"
	```

>This is the setup for the `raw` sub-project. If you do `cargo build --release -vv` *from within that folder* this mode is in effect.


**Default (neither defined)**

This pulls the ESP-IDF tooling to `.embuild/espressif` **within your project folder**. 

You can do this, but the author uses Multipass VM and pulling in 5GB of tooling to a shared folder slows things down. So he opts for either `out` or `global`, avoiding that problem.

**All options work**

It does not *really* matter, which option you choose. They all work.

- `out` is simplest and safest, but "forgets" the tooling if you were to do `cargo clean`. It is the default for the repo.
- `global` feels cleanest, but a) writes to your user home directory and b) requires extra setup and an install. This is why it's only used deeper in the repo.
- default would create a folder in the project directory, which the author cannot allow, only because he uses Multipass. You can surely use this mode as well, if you prefer.



## Kicking tyres

See `x` subproject for build instructions and running examples.


<!-- #later
## Using in your own projects

*tbd.*
-->

## References

- [ESP Zigbee SDK Programming Guide](https://docs.espressif.com/projects/esp-zigbee-sdk) (Espressif docs)

	- [API Reference](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32/api-reference/index.html) (Espressif docs)

- [Espressif IoT Development Framework](https://github.com/espressif/esp-idf) (GitHub)

	The C API that `esp-zigbee-sdk` builds upon.
