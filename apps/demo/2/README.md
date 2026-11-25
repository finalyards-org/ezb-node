# Demo 2

First read the [`/apps/README`](../../README.md).

Then [`/docs/devices/Wiser_window_door_sensor`](../../../docs/devices/Wiser_window_door_sensor.md).

Then, here.

>For the commands provided below, remain in the `apps` folder.


## Requirements

- 1 pc ESP32-C6 devkit
- 1 pc Schneider Electric / Wiser [Window/door sensor CCT591011](../../../docs/devices/Wiser_window_door_sensor.md)

	![](.images/wiser-door-sensor.png)


## Steps

### 1. Prepare

#### 1.1 The sensor

- Bring it close (~1m) to the devkit.

	Find the Setup/Reset -button on the bottom side of the sensor. Keep a pen around for pressing the button.

#### 1.2 The application

Printed on the device, you should see:

- "MAC address"
- "Install code"

This demo works with **pre-configured** install codes. This means you'll copy the codes into the `app.toml`; they get built into the binary and the devkit *only* allows entry to its network for the device(s) it knows. 

- Edit the `app.toml`:

	```
	[[node.pre_configured_devices]]
	log_name = "Sample Door/window sensor"
	mac_address = "4030 59FF FE9E 7D3D"
	install_code = "964B B40A 0684 F31D 3181 8DFD 0525 4897 337E"
	```

	>Note: You can have any number of such sections in the TOML. They form an array.

### 2. First launch

You can skip these on later launches. Both the devkit and the sensor store their network binding information in non-volatile memory, i.e. it carries over restarts.

If you *want* to start fresh, this is how to do it!

#### 2.1 Erase the devkit

```
$ espflash erase-flash
```

>Without this, the Zigbee network is constructed in a different way, based on the information stored in the non-volatile memory.
>
>For debugging, it is sometimes necessary to erase the flash.


#### 2.2 Reset the sensor

There are two ways to reset the sensor.

- **triple press** the "Setup/Reset" button (bottom side of the sensor)

	- sensor's LED should start blinking orange 🟧 🟧 🟧

	This is a "binding mode". It lasts 30s.

- **triple press** the button; then **long press** (~10s), until the LED turns RED. Release.

	This is a factory reset.

### 3. Launch

```
$ just door-run
```

Once the device joins, the LED should remain green 🟩 for a moment.

>If the device did NOT join, **triple-press** the button to get it into a binding state. Wait for ~1min.

On the coordinator log, you should see:

```
I (20061) 2_door::ias_coordinator: Device authorized: 0x70cc (40:30:59:ff:fe:9e:7d:3d)
I (20067) 2_door::ias_coordinator: Writing our IEEE address (fc:01:2c:ff:fe:f9:09:b4) as IAS CIE address to 0x70cc, ep 1
I (20556) 2_door::ias_coordinator: Write Attributes response: WriteAttrResp { info: CommonInfo { err: None, dst_ep: 1, cluster_id: IasZone, cluster_role: Client }, header: CommandHeader { src_addr: Short(28876), dst_addr: Short(0), src_ep: 1, dst_ep: 1, cluster_id: IasZone, profile_id_X: 260, fc_X: 8, manuf_code: None, tsn: 0, rssi: -68, cmd_id: 4 }, variables: [Success] }
I (21092) ezb_node::node::zigbee_task: ZCL core action not supported (skipped): EZB_ZCL_CORE_REPORT_ATTR_CB_ID
I (21569) 2_door::ias_coordinator: Zone Enroll Request: zone type 0x0015, manufacturer 0x105e; accepting as zone 0
I (22090) 2_door::ias_coordinator: Zone 0: closed (zone status 0x0030)
I (22091) 2_door::ias_coordinator: Received ZCL Default Response, status: success
D (22350) ezb_node::node::zigbee_task: Received: NwkSignalPermitJoinStatus { is_opened_for: Some(Duration { ticks: 180000000 }) }
I (22355) 2_door::ias_coordinator: Network 38293 is open for 180 seconds
I (84413) 2_door::ias_coordinator: Zone 0: OPEN (zone status 0x0031)
I (87015) 2_door::ias_coordinator: Zone 0: closed (zone status 0x0030)
I (89362) 2_door::ias_coordinator: Zone 0: OPEN (zone status 0x0031)
I (90704) 2_door::ias_coordinator: Zone 0: closed (zone status 0x0030)
...
```

Here, obviously, I've opened and closed the door/window sensor.

### 4. Test the sensor manually

Above, we tested that a fresh sensor

- does join the network
- does report its status

There are more test cases to do:

- restart the devkit; does the sensor re-bind and function?
- power off the sensor (pretend to change its battery); does it re-bind and function?

#### 4.1 Battery reporting

The sensor also provides (as any battery powered Zigbee devices do) a `PowerConfig` cluster. You can occasionally see messages like:

```
I (3632817) 2_door::ias_coordinator: Battery remaining: 86 %
I (3632837) 2_door::ias_coordinator: Battery voltage: 2.8 V
```

## References

- ["Wiser Window/Door sensor"](https://productinfo.se.com/wiser_home/wiser-window-door-sensor_wiser_home_device-user-guide/English/Wiser%20Window_Door%20Sensor_Wiser_Home_Device%20user%20guide_0000802479.xml/$/WiserWindow_DoorSensor_Wiser_HomeCPT_0000802539) (Schneider Electric)
