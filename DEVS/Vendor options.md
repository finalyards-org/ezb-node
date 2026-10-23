# Multi-vendor strategy

We're currently putting weight on `esp-zigbee-sdk` (and ESP32-C6's, ESP-IDF toolchain).

## Risks

- `esp-zigbee-sdk` is partly closed source

	This puts a toll to understanding the innards of the library, and might prove risky. 
	
	On the other hand, other companies seem to be using the stack and complaints/issues get resolved fairly quickly.
	
	This is definitely a good starting place for the project.


## Alternatives 

- [`zigbee-rs`](https://github.com/zigbee-rs/zigbee-rs) is a **Rust native Zigbee stack**

	It would, in the case of ESP32-C6, run on top of the IEEE radio library (`no_std`, unlike ESP-IDF).
	
	This is a **welcome direction** - doing things without ESP-IDF is way simpler than with it (no RTOS; no C libraries; no bindgen, ...).
	
	It might take a time, though.


The `zigbee-rs` might be the right long-term solution. One of the benefits is that it would make the Zigbee stack **vendor neutral**, allowing e.g. use of Espressif and Nordic devkits, without needing to implement two inner APIs in this project.


- Nordic (nRF)

	An alternative silicon (devkit) vendor.
	
	Not sure what their Zigbee / Rust offering is, but *once* the `zigbee-rs` takes wind, it should be easy :) to try it out.

---

The immediate aim is to get things moving. For that, a semi-closed source stack with a dedicated C-language development SDK, and bindgen, are quite enough. If anything, future will hold welcome simplification. 

<!--
---
Once we were to have two different kinds of devkits, also e.g. their antenna reception can be compared (with or without an external antenna).
-->
