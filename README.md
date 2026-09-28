# Zigbee on ESP32-C6 with Rust

## Motivation

This repo aims to provide the possibility to create, in Rust, for ESP32-C6 MCU:

- Zigbee controller - Zigbee router
- Zigbee custom end device

Wifi ([`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio)) and BLE ([`trouBLE`](https://github.com/embassy-rs/trouble)) are already Rust friendly comms platforms, but for Zigbee there does not seem to be a working solution.

>### Bare metal Zigbee
>
>[zigbee-rs](https://github.com/zigbee-rs/zigbee-rs) is definitely interesting to watch. It will eventually offer the protocol stack on top of the hardware radio interface. It's not, as of Sep'26 (version 0.1.0), a viable production choice, but once it is, it takes away the complexities of: ESP-IDF, C/Rust bindings. How beautiful will it be! :) 🌼🌞
>
>Until then, please read on.

### Focus

This repo aims at *forward looking* development. This means:

- using latest underlying versions of libraries (`esp-zigbee-lib`; ESP-IDF 6)
- active focus on Zigbee 3.0; interest in Zigbee 4.0
- low or no interest in legacy

Supported devices:

- [ ] Door/window sensor

<!-- #whisper intended:
- fire alarm (Wiser/Schneider)
- moisture (Third Reality)
-->

**Non-focus**

The following Zigbee features are not in the focus of this project:

- Groups
- Scenes
- Green power (harvested energy)
- Touchlink

<!-- tbd. make into a table, with ZCL specifications listed.

support: intended | not planned | tested devices
status: working, WIP, wish
-->

### Value

With Rust, we can make a whole lot better APIs than with C. Less code. Better IDE support (narrow interfaces instead of everything being flat and global).


## About ESP-IDF

[ESP-IDF](https://docs.espressif.com/projects/esp-idf/en/release-v6.0/esp32c6/get-started/index.html) is an ecosystem, based on the C language SDK of the same name. We need it because the `esp-zigbee-sdk` (currently best supported Zigbee library for the ESP32's) is made using it. What that means for a Rust project is:

- The **application binary** is **tied to the ESP-IDF ecosystem**. The end application must decide (or at least, configure):
	- ESP-IDF version to use
	- other build environment configurations
- FreeRTOS running underneath; the Rust code runs as one of its tasks.
- For peripherals, use `esp-idf-hal`, not `esp-hal`. The latter cannot be used, since it manages peripherals directly (not via an RTOS).

>This is elaborate but doable! The good part is that `esp-zigbee-sdk` (C side) is supported by Espressif, and their version 2 moved away from ZBoss, allowing *them* to steer the whole protocol stack (sure, it's still not open source, but it's got action!!).

### esp-idf-sys

Pulls in the whole ESP-IDF SDK, as part of the Rust compilation. It takes time and disk space (some 5GB) but is mostly a one-time effort. It provides headers and libraries that `esp-zigbee-sdk` relies upon.

### esp-idf-svc

This sits on top of the `-sys` and provides us with services (thus, the name) that allow Rust `std` API's to be used. 

We need to use it. ESP-IDF and Embassy (`async`) don't work together, if we don't.


### Which ESP-IDF to use?

The author aims at maintaining this towards the *latest stable release*.

|||status|
|---|---|---|
|6.1|latest (8-Sep-26)|`esp-idf-sys` ok; `esp-idf-hal` NOT; `esp-zigbee-sdk`*|
|6.0.3|||

>*tbd. update the report; does `esp-idf-hal` work with 6.0.3; 6.1???*

<!--r
|5.5.5||works with slightly modified git `esp-idf-sys`, git `esp-idf-hal`, stock `esp-zigbee-sdk`|
-->

<small>
`|*|`: `esp-zigbee-sdk` is being used - in the C level - together with ESP-IDF 6.1, so the chances are it will not be a limiting factor for upgrade.
</small>

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

- `apps/demo/1/light`
- `apps/demo/1/switch`
- `apps/demo/2`

	Reading a Door/Window sensor.

>Note: `esp-zigbee-sdk` examples *do not include examples using external devices*. All the examples are "ESP32 to ESP32". The author finds this somewhat strange and surprising.

<!-- tbd. an app for working with outside sensor; add when ready!
-->


## Requirements

- A devkit, e.g. [ESP32-C6-DevKitM-1](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32c6/esp32-c6-devkitm-1/user_guide.html)
- Rust installed
- 10 GB disk space

In reality, the recommendation is to use [EdgeVM](https://github.com/finalyards/EdgeVM) (GitHub) and Lima VM's for development.

This gives you a stable, reproducible footing, with most of the underlying tools already installed.

>If you prefer native (Ubuntu Linux) development, have a look at `edge-vm/project.yaml` in that repo, to see what dependencies you should install. Thanks!

**Additional installs**

- C compilers and `bindgen` CLI

	```
	$ sudo apt install build-essential clang
	[...]

	$ cargo +stable install --locked bindgen-cli
	```

	>Note: DO NOT use the bindgen CLI `apt` package - it lags behind the `cargo` one.

- `just` and `jq` (optional; for easier demo launch)

	```
	$ sudo apt install just jq
	```


## Steps

### Study the source code

- `apps/demo/`

	Sample applications. Note how `sdkconfig.defaults` - the file that defines ESP-IDF build configuration - is part of the application.

- `api`

	The API layer, providing a Rust interface to Zigbee.

- `raw`

	The 1-to-1 C/Rust interface to `esp_zigbee_sdk`, an ESP-IDF C library.

- `config`

	A library for turning TOML configuration into Rust source code, and for reading such into the `api`. Simplifies the applications, since declarative configuration is now out of the code. See `apps/demo/**/app.toml`.

---

Many of the subprojects have a soft link to `.cargo/config.espidf.toml`. This allows us to do development in different layers, while keeping changes to the said TOML only in one place. We avoid a single `.cargo/config.toml` because `config` is not an ESP-IDF project. This way, it is not affected.

---

### Build some (optional)

You can also skip directly to the "demo" section (next). These are useful for understanding the build layers, and for debugging problems in a build.

>Note: Each layer has their own `README` with more specific information.


**1. Raw**

```
$ cd raw
$ cargo build --release -vv --features esp32c6
[...]
```

This level, when first built, downloads the wild 5GB of ESP-IDF tooling. The `-vv` allows you to see something is happening, without it the build will seem rather stalled for some minutes.

For further builds, it will be faster.

If you ever need to clean something up, fully, also `rm -rf ~/.espressif`. That's where the ESP-IDF tools are collected.


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

If the builds succeed, you are ready to run the created binaries on ESP32-C6 devkits.

## Run demos

Have a look at the `apps` folder and run some demos.

```
$ cd apps
$ just door-run
[...]
```

That expects you to have `espflash` properly set up, and some other things. Please follow the instructions within [`apps/README.md`](./apps/README.md).


<!-- #later; perhaps do it in `docs/`?
## Using in your own projects

*tbd.*
-->


## Other

### Cleanup

Additional to normal `cargo clean`, there are tools in the `~/.espressif` folder (5..6 GB per each ESP-IDF version you've tried). 

You can wipe the folder (it's a kind of cache):

```
$ rm -rf ~/.espressif
```

## References

- [Espressif Zigbee SDK 2.x](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32c6/introduction.html) (Espressif docs; ESP32-C6)

	- "Zigbee Pro R23"
	- "Zigbee Cluster Library (ZCL) v8"

	- [API Reference](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32/api-reference/index.html) (Espressif docs)

- [Partition Tables](https://docs.espressif.com/projects/esp-idf/en/v6.0/esp32c6/api-guides/partition-tables.html) (ESP-IDF API Guides)
