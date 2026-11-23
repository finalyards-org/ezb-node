# Demo 2

First read the `../../README.md`.

Then `../README.md`.

Then, here.

>For the commands provided below, remain in the `apps` folder.


## Requirements

- 1 pc ESP32-C6 devkit
- 1 pc Schneider Electric / Wiser [Door/window sensor CCT591011](https://www.zigbee2mqtt.io/devices/CCT591011_AS.html)

	![](.images/window-door-sensor.png)

- A pin (or pen) for pressing the Reset button


## Steps

### 1. Prepare the sensor

Bring the sensor close (~ 1m) to the devkit.

Find the Setup/Reset -button on the bottom side of it. Have a pin/pen ready.

### 2. Prepare the application

Printed on the device, you should see:

<!-- tbd. image -->

- "MAC address"
- "Install code"

This demo works with **pre-configured** install codes. This means you'll edit the codes into the `app.toml`; they get built into the binary and the devkit *only* allows entry to its network for the device(s) it knows. 

- Edit the `app.toml`:

	```
	[[node.pre_configured_devices]]
	log_name = "Sample Door/window sensor"
	mac_address = "4030 59FF FE9E 7D3D"
	install_code = "964B B40A 0684 F31D 3181 8DFD 0525 4897 337E"
	```

	>Note: You can have any number of such sections in the TOML. They form an array.


### 3. Launch the application

First, reset the sensor to make it enter a "binding state":

- **triple press** the "Setup/Reset" button (bottom side of the sensor)

	- sensor's LED should start blinking orange 🟧 🟧 🟧

```
$ just door-run
```

Once the device joins, the LED should remain green 🟩 for a moment.

On the coordinator log, you should see:

```
 I (475) ezb_node::node::zigbee_task: Going to set up the device and 1 endpoint(s)
  I (509) 2_door::ias_coordinator: Initialize Zigbee stack
  I (520) 2_door::ias_coordinator: Device started up in factory-reset mode.
  I (1673) 2_door::ias_coordinator: Formed network successfully: Extended PAN ID: fc:01:2c:ff:fe:f9:09:b4, PAN ID: 53475, Channel:11, 
  Short Address: 0x0000
  I (2247) 2_door::ias_coordinator: Network steering completed.
  I (2241) 2_door::ias_coordinator: Network 53475 is open for 180 seconds
  ...
  I (19678) 2_door::ias_coordinator: Zigbee APP signal: ZdoSignalDeviceUpdate { device_addr: IeeeAddr(4625295773332831549), short_addr:
  60951, status_X: 1, tc_action_X: 0, parent_short: 0 }
  I (20034) 2_door::ias_coordinator: New device commissioned or rejoined: 0xee17 (40:30:59:ff:fe:9e:7d:3d)
  I (20035) 2_door::ias_coordinator: Writing our IEEE address (fc:01:2c:ff:fe:f9:09:b4) as IAS CIE address to 0xee17, ep 1
  ...
  I (25054) 2_door::ias_coordinator: Zigbee APP signal: ZdoSignalDeviceAuthorized { device_addr: IeeeAddr(4625295773332831549), 
  short_addr: 60951, type_X: 1, status_X: 0 }
```

*tbd. Still misses the actual joining.*

### 4. Test the sensor

*tbd.*


## Demo without flashing

As with demo `1`, also here once the flashing is done, the setup should be resilient.

- switch off the devkit (coordinator)

You can power it off a USB charger, or power bank (of course you won't see the logs this way). <!-- *..but we intend to have the multicolor LED show status and events.*-->


<!-- tbd. jatka, kun LED:llä voi nähdä ovien aukeamisen.
-->


## References

- ["Wiser Window/Door sensor"](https://productinfo.se.com/wiser_home/wiser-window-door-sensor_wiser_home_device-user-guide/English/Wiser%20Window_Door%20Sensor_Wiser_Home_Device%20user%20guide_0000802479.xml/$/WiserWindow_DoorSensor_Wiser_HomeCPT_0000802539) (Schneider Electric)

	- ["Led indications"](https://productinfo.se.com/wiser_home/wiser-window-door-sensor_wiser_home_device-user-guide/English/Wiser%20Window_Door%20Sensor_Wiser_Home_Device%20user%20guide_0000802479.xml/$/LEDIndicationsCPT_0000661246#LEDIndications-C4E8250E)

