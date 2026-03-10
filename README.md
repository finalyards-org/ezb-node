# Zigbee for ESP32 in Rust

## Motivation

While Wifi and BLE are nicely covered in Rust <sub>`|1|` `|2|`</sub>, Zigbee is not (Mar'26).

This repo aims to provide the possibility to create:

- Zigbee controller
- Zigbee router
- Zigbee custom end device

..using Embassy Rust code.

<small>
`|1|`: https://github.com/esp-rs/esp-hal/tree/main/esp-radio <br />
`|2|`: https://github.com/embassy-rs/trouble
</small>

## Folder structure

|||
|---|---|
|`examples`|You may start here - how to use the Rust API.|
|`main`|The Rust API level|
|`raw`|Bridging of `esp-zigbee-sdk` C API's to Rust|


## Requirements

- ESP32-C6 devkit

- Rust installed

	>*tbd. give instructions here that show the right toolchain etc.*

- `bindgen` CLI

	```
	$ cargo install --locked bindgen-cli
	```
	
	>Note: There is also an apt package (`sudo apt install bindgen`) but that seems to lag behind (0.66 vs. 0.72.1 at the time of writing).
	

<!--
Developed with:
- esp-idf v.5.5
- bindgen 0.72.1
-->

## Preparation

Load git submodules - we get the C sources that way.

```
$ git submodule update --init
```

## Steps

See either the `examples` folder - for practical projects - or the others for implementation details.


## References

- [ESP Zigbee SDK Programming Guide](https://docs.espressif.com/projects/esp-zigbee-sdk) (Espressif docs)

- [Espressif IoT Development Framework](https://github.com/espressif/esp-idf) (GitHub)

	The C API that `esp-zigbee-sdk` builds upon.
