# Apps

Runnable programs to showcase the `ezb-node`.

```
├── demo
│   ├── 1
│   │   ├── light/...
│   │   └── switch/...
│   └── 2
│       ├── app.toml
│       ├── ias_coordinator.rs
│       └── main.rs
├── partitions.csv
├── sdkconfig.defaults
└── src/...
```

>Note: `src` has tools common to all the demos. It's there only so that we can share code among the apps.


## Requirements

||ESP32&#8209;C6 devkits|Zigbee devices used|
|---|---|---|
|**Demo 1** - light and switch|2 pc|none|
|**Demo 2** - door/window sensor|1 pc|Schneider Electric / Wiser [Door/window sensor CCT591011](https://www.zigbee2mqtt.io/devices/CCT591011_AS.html) |

>📜 For detailed information about each of the demos, see their respective READMEs.

### Setting up `espflash`

To flash the applications on your devkit(s), you'll need the `espflash` tool.


>You can also use `idf.py` if you have the target device directly connected (and not using a VM). If so, use the one provided by the `esp-idf-sys` installation:
>
>```
>$ find ~/.espressif/ -name idf.py
/home/lima/.espressif/esp-idf/v6.0.3/tools/idf.py
>```
>You might also need to go through other hoops. See `_mc/README.md` for inspiration.

<!-- #r: was
<details><summary>Alternatives to `espflash`</summary>
>The author prefers `espflash` because there's a remoting tool for it: he can keep the devkit and the development machine **physically separate**.
>
>`idf.py` is the flashing tool you'd use with ESP-IDF normally. You may use it, but *do not install a separate version* on your system - that confuses `esp-idf-sys`. You can find one by:
>
>```
>$ find ~/.espressif/ -name idf.py
/home/lima/.espressif/esp-idf/v6.0.3/tools/idf.py
>```
>
>You might need to go through other hoops. See `_mc/README.md` for inspiration...
</details>
-->

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

Once you see that, we can proceed to flashing and running.


## Build and run

These steps are pretty similar across all (both?) the demos. 

⚠️ Be aware that **any information you have on the devkit will be erased** by the flashing. This includes prior Zigbee network pairings.

Let's go!


### Build

```
$ just lb
```

This builds the application, and its dependencies.

>If you haven't already, it also downloads the whole ESP-IDF toolchain from the Internet.


### Run

```
$ just lr
[...]
```

This launches a certain demo.

For other demos, variate the commands (e.g. `db`, `dr` for demo 2). See `Justfile` for details.

```
$ just --list
Available recipes:
    default

    [door]
    door-build          # 2-door [alias: db]
    door-run            # [alias: dr]

    [light]
    lbx
    light-build         # 1-light [alias: lb]
    light-run           # [alias: lr]

    [switch]
    switch-build        # 1-switch [alias: sb]
    switch-run          # [alias: sr]
```

<!-- tbd. How to proceed from here?
-->


## next

You might want to check the specific instructions for a certain demo:

- [demo/1/README.md](demo/1/README.md)
- [demo/2/README.md](demo/2/README.md)

