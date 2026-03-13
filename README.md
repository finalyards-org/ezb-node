# Zigbee on ESP32 with Rust

## Motivation

This repo aims to provide the possibility to create, in Rust, for ESP32 MCUs:

- Zigbee controller
- Zigbee router
- Zigbee custom end device

Wifi ([`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio)) and BLE ([`trouBLE`](https://github.com/embassy-rs/trouble)) are already Rust friendly comms platforms.


## About ESP-IDF

ESP-IDF is an ecosystem, based on the C language SDK of the same name. We need it because the `esp-zigbee-sdk` is made using ESP-IDF. What it means for a Rust project is:

- FreeRTOS running underneath; the Rust code runs as one of its tasks. This is not "bare metal".
- for peripherals, use `esp-idf-hal`. `esp-hal` cannot be used, since it manages peripherals directly (not having FreeRTOS in the loop).

>This should be doable. *Eventually* it would be neat to have a non-idf Zigbee Rust implementation. That would need a Rust native Zigbee stack to be created, e.g. on top of the `esp-radio` IEEE 802.15.4 low layers.

### esp-idf-svc

Pulls in the whole ESP-IDF SDK, as part of the Rust compilation. It takes time and disk space (some 4GB) but is a one-time effort. It provides headers and libraries that `esp-zigbee-sdk` relies upon.

### Which ESP-IDF to use?

We will <strike>likely opt for the one used by `esp-zigbee-sdk`</strike> take the latest supported by both `esp-zigbee-sdk` and `esp-idf-svc`.

|||
|---|---|
|6.0|not supported by `esp-idf-svc`|
|5.5.3|default of `esp-idf-svc`<sub>[source](https://github.com/esp-rs/esp-idf-svc/blob/master/.cargo/config.toml#L10)</sub>|
|5.4.x||
|5.3.2|recommended by `esp-zigbee-sdk` (11-Mar-26)|

### `esp-idf-svc` is a community effort

Yes.

<!-- whisper
And they [misspell "Community"](https://github.com/esp-rs/esp-idf-svc?tab=readme-ov-file#commuity-effort) - intentionally or not...
-->

That is a concern, if we bet the whole project on it. But on the other side, let's just drive and see whether the road takes anywhere!! <font size=+3>🛣️</font>

## Folder structure

|||
|---|---|
|`examples`|You may start here - how to use the Rust API.|
|`main`|The Rust API level|
|`raw`|Bridging of `esp-zigbee-sdk` C API's to Rust|


## Requirements

- ESP32-C6 devkit
- Rust installed
	<!-- tbd. give instructions here that show the right toolchain etc.-->
- &gt;7 GB disk space

	The ESP-IDF environment will get downloaded to:
	
	- `~/.espressif`
	- `~/...`

- `bindgen` CLI

	```
	$ cargo install --locked bindgen-cli
	```
	
	>Note: There is also an apt package (`sudo apt install bindgen`) but that seems to lag behind (0.66 vs. 0.72.1 at the time of writing).

- Some of these:

	```
	$ sudo apt-get install git wget flex bison gperf python3 python3-pip python3-venv cmake ninja-build ccache libffi-dev libssl-dev dfu-util libusb-1.0-0
	```

	>Note: Don't install all of the above! If something were to fail, add from that list and retry.

- GNU `make`

Note that ESP-IDF **may not be globally installed** - it would mess with the `esp-idf-sys`.

<!--
Developed with:
- bindgen 0.72.1
-->

## Preparation

### Submodules

Load git submodules - we get the C sources that way.

```
$ git submodule update --init
```

### Do you use Multipass?

If not, please comment out - or remove - these lines in `.cargo/config.toml`:

```
ESP_IDF_TOOLS_INSTALL_DIR = "out"
[...]
CARGO_WORKSPACE_DIR = { value = "", relative = true }
```

>These are needed because - with Multipass - using a normal `target` folder within a shared project folder (a handy setup for editing in IDE but building in a VM) is *tremendously slow*. However, `esp-idf-sys` needs to be told about this arrangement. If you do not use Multipass, it's probably best to remove the above lines.


## Steps

### Build the `raw`

```
$ cd raw
$ cargo build --release -vv
[...]
```

This should get built automatically, as a dependency (of `main` and/or examples), but it's a good idea to do it manually.

The build takes longer than normally, and downloads e.g. a full ESP-IDF (C language) toolchain. If things go wrong, you want to know. Adding the `-vv` ("very verbose") flag allows you to see things are progressing.


### What next?

See either the `examples` folder - for practical projects - or `main` and `raw` for implementation details.

<!-- #later
## Using in your own projects

*tbd.*
-->

## References

- [ESP Zigbee SDK Programming Guide](https://docs.espressif.com/projects/esp-zigbee-sdk) (Espressif docs)

- [Espressif IoT Development Framework](https://github.com/espressif/esp-idf) (GitHub)

	The C API that `esp-zigbee-sdk` builds upon.
