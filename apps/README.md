# Examples

Some runnable programs to showcase the `esp-zb`.

- Uses Embassy
- Uses `esp-idf-sys` and `esp-idf-hal` but not `esp-idf-svc`


## Steps

### Build

```
$ cargo build --release -vv --example light
```

Builds the:

- `raw` binding to (C language) `esp-zigbee-sdk`
- `x` Rust API
- a "Light bulb" application

### Run

```
$ cargo run --release -vv --example light
[...]
```


## Demo - Light Bulb

In this demo, we flash two ESP32-C6 boards:

- Light bulb
- Switch

Once you press the `BOOT` button on the "Switch" device, the light of the "Light bulb" device should change hue and brightness.

>🚨 WARNING!  The LED light on ESP32-C6 can be **BRIGHT**. To protect your eye sight:
>
>- do not bring the device near your face
>- do not stare directly at it
>- place e.g. a paper sticker to diffuse the light

### Preparation

Have two ESP32-C6 ready and connected to your computer.

### Flash the Light Bulb

```
$ cargo run --release -vv --example light
[...]
```

Since there are two devices, `espflash` will ask you which one to flash. It does not matter - just remember which you chose.

>Note: The designation for the device can be either `ttyUSB{n}` or `ttyASU{n}`, depending on which port of the devkit you used. Also this does not matter.

### Flash the Switch

```
$ cargo run --release -vv --example switch
[...]
```

This time, pick the other device.

### Button test

Press `BOOT` on the Switch device. Did the light on the "Light bulb" react?

This proves Zigbee connection works. 

>Hint. Once flashed, the boards do not need to be connected to a computer. You can charge them from any charger.

