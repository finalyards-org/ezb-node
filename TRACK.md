# Track


<!--r No-one cared to comment.

## `esp-idf-hal` compatibility with ESP-IDF 6.x

- [ ] ["Document state of supported ESP-IDF versions in the README.md"](https://github.com/esp-rs/esp-idf-hal/issues/595)

ESP-IDF 6.0 is a major cleanup. It might take time for `esp-idf-hal` to catch up. Stay in 5.5.x.
-->

## `esp-zigbee-sdk`: partitions changed too quietly; need more data!

- [ ] ["Migration from zb_storage to nvs"](https://github.com/espressif/esp-zigbee-sdk/discussions/907) (GitHub Discussions)

"They" just changed partitions without really opening the larger story...

Or telling us developers what we should do.

More details needed!

--- <!-- was: -->

The version 2.0 examples have transitioned from using a separate `zb_storage` flash partition for persistency to using `nvs` (which is already there).

Is there discussion somewhere as to:

- what this means (for an application writer)?
- what is the recommended way further

	- are there still use cases (other than backwards compatibility, updating devices) for keeping the persistence on a separate partition?

I am making a green field solution and can go all-in with the `nvs` approach. Just want to understand the implications.
