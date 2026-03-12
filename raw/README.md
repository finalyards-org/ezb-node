# `esp-zb-raw`

Low level C/Rust interfacing of the application level [`esp-zigbee-sdk`](https://github.com/espressif/esp-zigbee-sdk) APIs.

<!-- tbd. Excalidraw schematic
-->

## Requirements

See above.

## Preparation

## Steps

```
$ ESP_IDF_SYS_ROOT_CRATE=$(pwd) \
  cargo build --release -vv
```

This is needed (only) if you are using a global target folder - which is necessary for performance when running under Multipass VM <sub>[it's a long story](...)</sub>.

```
$ 
```

>Note: This command is good when you are developing the `raw` side. To just use this, let the upper level Cargo handle it. ☀️

## Testing

## References

