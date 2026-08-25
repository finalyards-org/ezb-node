# Demo 1 - Light and Switch

In this demo, we use two ESP32-C6 boards:

- Light; a Zigbee controller
- Switch; binds to the light

>The demo is 1-to-1 the same as the C side `examples/home_automation_devices` > `color_dimmable_light` and `color_dimmer_switch` pair. This allows us to compare the implementations, execute them across (one node C, another Rust), and if something is broken, debug with the C code as a reference.

Once you press the `BOOT` button on the "Switch" device, the light of the "Light bulb" device should change hue and brightness.

>🚨 WARNING!  The LED light on ESP32-C6 can be **BRIGHT**. To protect your eye sight:
>
>- do not bring the device near your face
>- do not stare directly at it
>- place e.g. a paper sticker to diffuse the light

### Preparation

Have two ESP32-C6 ready and connected to your computer.

```
$ espflash board-info
[...]
# it should ask you which one to use, and show its info.
```

### Flash the Light

```
$ just lr
[...]
```

Since there are two devices, `espflash` will ask you which one to flash. It does not matter - just remember which you chose.

>Note: The designation for the device can be either `ttyUSB{n}` or `ttyASU{n}`, depending on which port of the devkit you used. Also this does not matter.

### Flash the Switch

```
$ just sr
[...]
```

This time, pick the other device. :)

### Button test

Press `BOOT` on the Switch device. Did the light on the "Light bulb" react?

This proves Zigbee connection works. 

>Hint. Once flashed, the boards do not need to be connected to a computer. You can charge them from e.g. a power bank! 🔋

