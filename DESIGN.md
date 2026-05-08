# Design notes

## Rust features do not apply to `raw`

While one can switch on/off use features, e.g. `ep_...`, this mechanism extends only to the API, not the RAW level.

The main reason for this is that we get the `esp-zigbee-lib` as a monolith, black box library. There are not ways to reduce its footprint, e.g. based on the Controller/Router/EndDevice nature of the eventual node - or by "shutting off" certain clusters of device types. We will always build with the same library.

This means there's almost no upside to bringing features down to the `raw` level; it would only add complexity.

On the API side, however, it makes sense to have features. This leads to:

- better code clarity/readability: one can see sections that only matter to a particular node type / device type
- ability to selectively deprecate features, with confidence that other use cases are not affected

Keep this in mind when reading the code. API has features, RAW brings everything (that API needs) in.

