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

### `global` vs. `out`

This is a setting that affects where the build output files (Espressif tools, in particular) are placed.

Steered in `.cargo/config.toml`:

```
ESP_IDF_TOOLS_INSTALL_DIR = "out"
#ESP_IDF_TOOLS_INSTALL_DIR = "global"
```

>As mentioned in the comments in that file:

**`out`**: keep tools within the Rust `target`; `cargo clean` also clears tool selection (and rebuild means they will be downloaded and installed again).

**`global`**: keep tools apart, in `~/.espressif`. This is useful if you change deep build settings (e.g. test the `sdkconfig` generation) but don't need the tools to be reset each time.

>For release, it's likely best to keep it as `out`. This way, a dependency causing our build will not create folders outside of the usual build target.
