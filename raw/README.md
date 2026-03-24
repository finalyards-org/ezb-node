# `esp-zb-raw`

Low level C/Rust interfacing of [`esp-zigbee-sdk`](https://github.com/espressif/esp-zigbee-sdk) APIs.

<!-- tbd. Excalidraw schematic
-->

## Requirements

- `bindgen` CLI
- GNU `make`

>Please consult the parent folder for more details.

<!-- Author uses:
- bindgen 0.72.1
-->

## Steps

```
$ cargo build --release -vv --features esp32c6
```

>Note: This command is good when you are developing the `raw` side. To just use this, let the upper level Cargo handle it. ☀️

<!--
## Testing

...tbd. Rust tests; examples
-->

## Advanced

### Clean everything

In developing, the ESP-IDF side may get out of sync. To have a clean slate:

```
$ cargo clean
$ rm -rf ~/.espressif
```

### `global` vs. `out`

This is steered in the `.cargo/config.toml`. The idea is that in released (checked out) code the mode would be `out`.

```
ESP_IDF_TOOLS_INSTALL_DIR = "out"
#ESP_IDF_TOOLS_INSTALL_DIR = "global"
```

See the comments in that file for the differences.
