# Demo 1

You'll need:

- two ESP32-C6 devkits

	The devices will be flashed/reset in the demo, so if you take them from an existing network, you'll need to reset and rejoin them afterwards.

- `espflash` 

<!--
Author has:
- espflash v4.4.0
-->


It is assumed that both the boards are connected to the same computer. 

>[HINT] Once flashed you don't generally need data connection to the boards (i.e. you can just power them off, run the demo while powering from charging outlets).


## General prep

```
$ espflash board-info
[2026-04-03T17:05:04Z INFO ] Detected 2 serial ports
[2026-04-03T17:05:04Z INFO ] Ports which match a known common dev board are highlighted
[2026-04-03T17:05:04Z INFO ] Please select a port
❯ /dev/ttyUSB1 - CP2102N USB to UART Bridge Controller
  /dev/ttyUSB0 - CP2102N USB to UART Bridge Controller
```

This allows you to catch the names of the boards:

- `/dev/ttyUSB1` 
- `/dev/ttyUSB0`

>Note: Depending on the port you plugged the cable in (in the devkit side), these can be `/dev/ttyACM0`. It does not matter.

Remember the names. Now, let's clear the boards.

Press `Ctrl-C`.

```
$ espflash erase-flash --port /dev/ttyUSB0
...
[2026-04-03T17:17:02Z INFO ] Erasing Flash...
[2026-04-03T17:17:03Z INFO ] Flash has been erased!
```

```
$ espflash erase-flash --port /dev/ttyUSB1
```

It is important also the non-volatile memory of the boards gets reset. Zigbee programs keep their network binding information stored over restarts.


## Steps

>In the below commands, we assign `/dev/ttyUSB0` to the light and `/dev/ttyUSB1` to the switch. Use names that you discovered, above.

<p />

>❗️WARNING!! The light intensity of the demo MAY SERIOUSLY HURT YOUR EYES!! Please use shades or stick a piece of paper on top of the LED! 😎🚨

<!-- author's note:
Did not want to change the intensity, to keep the examples 1-to-1 with the C code, for comparison.
-->

Good.

```
$ cd x
```

#### Light

```
$ cargo build --release -vv --example light
[...]
```

<!-- maybe not
>Confirm the output (optional):
>
>```
>$ file target/riscv32imac-esp-espidf/release/examples/light
/home/ubuntu/target/riscv32imac-esp-espidf/release/examples/light: ELF 32-bit LSB executable, UCB RISC-V, RVC, soft-float ABI, version 1 (SYSV), statically linked, not stripped
>```
-->

```
$ espflash flash --port /dev/ttyUSB0 --monitor ~/target/riscv32imac-esp-espidf/release/examples/light
[...]
********************** OUTPUT HERE <<<<<
```

><font color=orange size="+3">tbd. command output </font>

You can let the code run (detach by **XXXXXXXXXX**).


#### Switch

The same for this fellow.

```
$ cargo build --release -vv --example switch
```

```
$ espflash flash --port /dev/ttyUSB1 --monitor ~/target/riscv32imac-esp-espidf/release/examples/switch
[...]
********************** OUTPUT HERE <<<<<
```

#### Demo! 🎉🎉🎉

Press the `BOOT` button on the switch device.

The color and intensity of the LED should change.

>This is the exact same demo as ESP-ZIGBEE-SDK has, but their's written in ESP-IDF and C. This is completely coded in Rust.


<!-- 
tbd. To be continued with:

- using a commercial switch
- using a commercial LED bulb

---

### Demo 1a - commercial light (optional)

Zigbee is a vendor neutral standard. Thus, you should be able to exchange the light - and switch - parts of the above demo with off-the-shelf, commercial products.

<font color=orange size="+3"> WIP...</font> Come back, later.
-->

<!-- #later

#### Requirements

- *one* ESP32-C6
- one commercial Zigbee color bulb (e.g. [Hue 1100W color](https://www.philips-hue.com/fi-fi/p/hue-white-and-color-ambiance-a60-e27-alylamppu-1-100/8720169365735))

### Demo 1b - commercial switch (optional)


#### Requirements

- *one* ESP32-C6
- one commercial Zigbee switch (e.g. [Sonoff Orb 4-in-1](https://sonoff.tech/en-eu/products/sonoff-orb-4-in-1-zigbee-smart-scene-button-snzb-01m?srsltid=AfmBOookVMWsYkqSApOH5bwjckWCmZsqG0jbQOcRx4pvaIHA9CKrb08b))

-->
