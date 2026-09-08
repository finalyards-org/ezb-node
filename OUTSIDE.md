# Outside questions

## `esp-zigbee-sdk`

The version 2.0 examples have transitioned from using a separate `zb_storage` flash partition for persistency to using `nvs` (which is already there).

Is there discussion somewhere as to:

- what this means (for an application writer)?
- what is the recommended way further

	- are there still use cases (other than backwards compatibility, updating devices) for keeping the persistence on a separate partition?

I am making a green field solution and can go all-in with the `nvs` approach. Just want to understand the implications.

