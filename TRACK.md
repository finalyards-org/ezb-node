# Track


## `esp-idf-hal` compatibility with ESP-IDF 6.x

Asked to:

- ["Document state of supported ESP-IDF versions in the README.md"](https://github.com/esp-rs/esp-idf-hal/issues/595)

ESP-IDF 6.0 is a major cleanup. It might take time for `esp-idf-hal` to catch up. Stay in 5.5.x.



<!-- hidden; they are valid, but not not very urgent. We have decent fixes for them.

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



## `esp-idf-sys`: using `picolib`

- [ ]["Figure out how to properly support picolibc (IDF 6.0+)"](https://github.com/esp-rs/esp-idf-sys/issues/410)

Work-around:

- Use `newlibc` for now. Needs an `sdkconfig.defaults` key - see PR.


## `zigbee-rs`

[https://github.com/zigbee-rs/zigbee-rs](https://github.com/zigbee-rs/zigbee-rs)

Being a full-Rust Zigbee stack, this would bridge IEEE [`esp-radio`](https://github.com/esp-rs/esp-hal/tree/main/esp-radio) with Zigbee applications, without the `esp-idf-*` ecosystem. i.e. applications could be bare metal, using the normal `esp-hal` APIs.

- [ ] Keep an eye on the progress


## Deprecation warnings (with `cargo build -vv`)

- ["Tracking Issue: Used deprecated headers in esp-idf-sys"](https://github.com/esp-rs/esp-idf-sys/issues/312)

	Not so bad any more?
-->