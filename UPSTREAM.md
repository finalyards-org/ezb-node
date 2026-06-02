# Upstream

*Things to eventually report to upstream projects.*

## `ezb_bdb_comm_mode_e` derives wrong doc

`esp_zigbee_lib` has this (separate docs for these two types) - `bdb.h`:

```
/**
 * @brief Base Device Behavior (BDB) commissioning capability.
 *
 */
typedef enum ezb_bdb_comm_capability_e {
    EZB_BDB_COMM_CAPABILITY_NONE                = 0x00, /*!< No commissioning capability */
    EZB_BDB_COMM_CAPABILITY_NETWORK_STEERING    = 0x01, /*!< Network steering capability */
    EZB_BDB_COMM_CAPABILITY_NETWORK_FORMATION   = 0x02, /*!< Network formation capability */
    EZB_BDB_COMM_CAPABILITY_FINDING_N_BINDING   = 0x04, /*!< Finding & binding capability */
    EZB_BDB_COMM_CAPABILITY_TOUCHLINK           = 0x08, /*!< Touchlink commissioning capability */
} ezb_bdb_comm_capability_t;

/**
 * @brief Base Device Behavior (BDB) operation mode.
 *
 */
 typedef enum ezb_bdb_comm_mode_e {
...
```

However, the bindgen-generated output has the same line: `tmp/bindings_0.rs`

```
/// @brief Base Device Behavior (BDB) commissioning capability.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ezb_bdb_comm_capability_e {
[...]
/// @brief Base Device Behavior (BDB) commissioning capability.
pub use self::ezb_bdb_comm_capability_e as ezb_bdb_comm_capability_t;
impl ezb_bdb_comm_mode_e {
[...]
```

- [ ] Check the behaviour when:
	- removing `tmp/bindings_0.rs`
	- rebuilding

- [ ] Try to find whether `bindgen` has a bug here, and isolate/explain it.

**Work-around**

Copying the description directly from C header.


## `UNREPORTBLE_ATTRIB`

Within `zcl_type.h`, this line has two typos of the same word:

```
    EZB_ZCL_STATUS_UNREPORTBLE_ATTRIB = 0x8cU,   /*!< Unreporttable attribute */
                           ^-- A missing                      ^-- excess t
```

While some fields are obviously shortened (`UNSUP`), this seems like something to fix.
