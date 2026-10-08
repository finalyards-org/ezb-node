# Demo 1 - Light and Switch

---

>The demo is 1-to-1 the same as the C side `examples/home_automation_devices` > `color_dimmable_light` and `color_dimmer_switch` pair. This allows us to compare the implementations, execute them across (one node C, another Rust), and if something is broken, debug with the C code as a reference.

---

<!-- tbd. Consider a picture of two devkits, powered on their own, LED showing
-->

This demo uses two ESP32-C6 boards:

|name|Zigbee&nbsp;role|provides|
|---|---|---|
|Light|controller|establishing the network; providing access to its multicolored LED|
|Switch|router|finding the light and steering it|


Once you press the `BOOT` button on the Switch device, the light of the Light device should change hue and brightness.

>🚨 WARNING!  The LED light on ESP32-C6 can be **BRIGHT**. To protect your eye sight:
>
>- do not bring the device near your face
>- do not stare directly at it
>- place e.g. a paper sticker or rubber pad to diffuse the light!

## Preparation

Have two ESP32-C6 ready and connected to your computer.

```
$ espflash board-info
[...]
# it should ask you which one to use, and show its info.
```

## Flash the Light

```
$ just lr
[...]
```

It does not matter which device you flash first - just remember which you chose.

>Note: The designation for the device can be either `ttyUSB{n}` or `ttyASU{n}`, depending on which port of the devkit you used. Both should work.

## Flash the Switch

```
$ just sr
[...]
```

This time, pick the other device. :)

## Button test

Press `BOOT` on the Switch device. Did the light on the "Light bulb" react?

This proves Zigbee connection works. 


## Next..

Once flashed, the boards can operate on your own.

You can detach them from your computer. Charge them e.g. off a power bank! 🔋 

When restarting, the network should get auto-created and joined, and the button should still work!

