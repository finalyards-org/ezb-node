# Technical notes

Background on some technical aspects that are *important* but might not be immediately obvious.

## Two-layer structure

The import works like a funnel:

```
{all esp-zigbee-lib symbols, C side}

	`--- Makefile generates `tmp/bindings_0.rs` ---v

{symbols bindings.rs sees}

	`--- published by src/lib.rs ---v

{symbols API level sees}
```

This means:

- `tmp/bindings_0.rs` has a *subset* of all possible symbols. This is mainly to keep its size workable!
- `bindings.rs` can have access to any C side symbols (but they need to be opted in in the `Makefile` steering the `bindgen` step)
	- `bindings.rs` can *override* symbols with its own. This is not much done (the aim is to have `raw` not change the abstraction level) but it's possible.
- symbols aimed for the API layer must *still be exposed* by `src/lib.rs`. This gives us control on the outside footprint of the library.


## Expanding types

`bindings.rs` reads the generated binding is using `import!`. This has the advantage that it can *tail* the types with implementations of e.g. `Default`, `From` and `Into` traits. 

Such extensions do not change the abstraction of the said types (leaving that to the API level) but they can e.g. hide low level complexities (example: converting IEEE addresses from a C `union` to Rust unpacked `u64`).

