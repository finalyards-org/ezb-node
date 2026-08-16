# Demo 2 - commercial door/window switch


In this demo, we use a commercial [Schneider Electric Wiser Window/Door sensor](https://www.se.com/sg/en/product/CCT591011_AS/window-door-sensor-wiser-white/).

ZCL clusters employed:

|||
|---|---|
|`ias_zone` (0x0500)|
|`power_config` (0x...)|

Door opening is part of Intruder Alert System (IAS) profile in the Zigbee world.


## Preparation

Start looking for a network:

- Press the "Setup / Reset" button at the back of the sensor

	- 3 times short (<0.5s) within 1.5s

	- LED starts blinking 🟧

		The device is looking for a network to join. 

- Start the coordinator

	```
	$ just dr
	[...]
	```

	Check the log. If the launch happened fast enough, the door sensor joins the network.

	>Notice:
	>	
	>Once the connection is gained, the LED shows a moment in 🟩.


## Additional

**Factory reset**

Press the "Setup / Reset" button **4** times, holding it down for 10s on the 4th.


## References

- ["ESP32C6 coordinator: Unable to receive attribute reports from devices"](https://github.com/espressif/esp-zigbee-sdk/issues/42) (GitHub Issue, Jun'23; closed)
