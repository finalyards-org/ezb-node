# `esp-zb`

The upper level Rust adaptation.

- What features are brought further, from `raw` 1:1 APIs.
- How they are brought further.


## Steps

```
$ cargo build --release -vv
```

The build takes longer than normally, and downloads e.g. a full ESP-IDF (C language) toolchain. If things go wrong, you want to know. Adding the `-vv` ("very verbose") flag allows you to see things are progressing.

>For troubleshooting, or plain curiosity, check the documents under `DEVS/` folder.


### Run an example

```
$ cargo run --release --example a
```
