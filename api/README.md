# `ezb-node`

The Rust adaptation layer.

- What features are brought further, from `raw` 1:1 APIs.
- How they are brought further.

In practice, you'll use this from your code, or the `examples/`.

## Steps

```
$ cargo build --release -vv --features coordinator
```

Builds the library.

The build takes longer than normally, since it downloads a full ESP-IDF (C language) toolchain. Adding the `-vv` ("very verbose") flag allows you to see things are progressing.

>For troubleshooting, or plain curiosity, check the documents under `DEVS/` folder.
