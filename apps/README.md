# Examples

Some runnable programs to showcase the `ezb-node`.

```
├── demo
│   ├── 1
│   │   ├── light
│   │   └── switch
│   └── 2.door
│       ├── README.md
│       ├── app.toml
│       ├── light_coordinator.rs
│       └── main.rs
├── partitions.csv
├── sdkconfig.defaults
└── src 		# common tools
```

## Requirements

||ESP32-C6 devkits|Zigbee devices|
|---|---|---|
|**Demo 1** - light and switch|2 pc|none|
|**Demo 2** - door/window sensor|2 pc|1 window/door sensor|

### Setting up `espflash`

To flash the applications on your devkit(s), you'll need the `espflash` tool.

>If you connect directly with a cable to the devkit, and have Python installed, you *may* be able to use `idf.py` for this. Do so if you are more comfortable with it than `espflash`. But DO NOT install the ESP-IDF toolchain manually. Instead, you'll find `idf.py` under
>`~/.espressif/esp-idf/v5.5.5/tools/idf.py`.
>
>The author prefers `espflash` because there's a proxying tool for it; he can keep the devkit and the development machine **physically separated**. (You'd like as well, especially if doing motor controls with 12V! ⚡️⚡️)
>
>`probe-rs` is a *third* tool used for flashing. It cannot be used with ESP-IDF applications for reasons that ... the author forgot.

**So you proceed** to install `espflash`:

```
$ cargo install espflash --locked
```

.. **or use** the [`probe-rs-remote`](https://github.com/finalyards-org/probe-rs-remote) remoting solution, where the actual `espflash` runs on e.g. a Raspberry Pi. Now you can easily use a VM for the builds.

>`probe-rs-remote` can be used for proxying calls to either (or both) `probe-rs` and `espflash`.

**It pings?**

Once you have `espflash` installed, and cables connected, try:

```
$ espflash board-info
```

If you have two devkits connected, you are asked which one to target:

```
❯ /dev/ttyUSB1 - CP2102N USB to UART Bridge Controller
  /dev/ttyUSB0 - CP2102N USB to UART Bridge Controller
```

If you only have one, you get straight to the output:

```
Chip type:         esp32c6 (revision v0.2)
Crystal frequency: 40 MHz
Flash size:        4MB
Features:          WiFi 6, BT 5
MAC address:       fc:01:2c:f9:04:38

Security Information:
=====================
Flags: 0x00000000 (0)
Key Purposes: [0, 0, 0, 0, 0, 0, 12]
Chip ID: 13
API Version: 0
Secure Boot: Disabled
Flash Encryption: Disabled
SPI Boot Crypt Count (SPI_BOOT_CRYPT_CNT): 0x0
```

Hope it looks that good!


## Build and run

The steps are pretty similar to all demos (`1.light`, `1.switch`, `2`). 

⚠️ Just be aware that **any information you have on the devkit will be erased** by the flashing. This includes prior Zigbee network pairings.

🟢 Let's go!


### Build

>We use a `Justfile` to keep the commands short. You can open it to see the actual `cargo` commands used.

```
$ just lb
```

This builds the application, and its dependencies:

- `raw` binding to (C language) `esp-zigbee-lib` 2.0
- `api` = Rust API to the above

If you haven't already, it also downloads the whole ESP-IDF toolchain from the Internet. 

It may take 5..6 GB of disk space.


### Run

```
$ just lr
[...]
```


## Demos

Continue to the READMEs in the folders:

- [1/README.md](1/README.md)
- [2.door/README.md](2.door/README.md)

- ...

