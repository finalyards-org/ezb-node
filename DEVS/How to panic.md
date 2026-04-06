# How to panic

The author tried a couple of ways, settling to this:

## Panic unwind with self-made handler

**`.cargo/config.toml`**

```
[unstable]
build-std = ["std"]
```

**`Cargo.toml`**

```
[dev-dependencies]
esp-idf-sys = { workspace = true, features = ["alloc_handler", "binstart"], default-features = false }
```

= without panic handler.

**`examples/common/panic_handler.rs`**

Code that gets called on panic. Uses `esp_idf_sys::esp_rom_printf` to output the information.

### Pros and cons

This does what we want:

```
PANIC: panicked at x/examples/light.rs:46:5:
not yet implemented
```

- [x] Provides a clear indication on where the panic happened.
- [x] Provides the text
- [x] Does not enter a reset-loop

A reset loop could be favourable in long-lasting, release grade products. In those cases, we should also gather information of the crash online - silent restarts are immature.

In development, a reset loop is a nuisance.

## Other options?

Likely, yes. This one is Good Enough For Now - Safe Enough To Try!

