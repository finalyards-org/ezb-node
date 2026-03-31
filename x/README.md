# `esp-zb`

The upper level Rust adaptation.

- What features are brought further, from `raw` 1:1 APIs.
- How they are brought further.


## Steps

```
$ cargo build --release -vv
```

Builds the library.

The build takes longer than normally, since it downloads a full ESP-IDF (C language) toolchain. Adding the `-vv` ("very verbose") flag allows you to see things are progressing.

>For troubleshooting, or plain curiosity, check the documents under `DEVS/` folder.

### Run an example

```
$ cargo run --release --example a -vv
[...]
INFO - Hello, world! esp_zigbee_sdk: 1.6.8
I (260) main_task: Returned from app_main()
```

