# Upstream

*Things to eventually report to upstream projects.*


## `esp-idf-sys` (0.37.2) renders relative paths wrong (not like it's documented)

>"If such a path is relative, it will be relative to the workspace directory." <sub>[source](https://github.com/esp-rs/esp-idf-sys/blob/master/BUILD-OPTIONS.md#esp_idf_sdkconfig_defaults-esp_idf_sdkconfig_defaults)</sub>

**‼️‼️THIS IS NOT TRUE !!!**

In observed reality, these lines, when built under the 'apps' folder:

```
           ESP_IDF_SDKCONFIG = { value = "sdkconfig", relative = true }
           ESP_IDF_SDKCONFIG_DEFAULTS = { value = "raw/sdkconfig.defaults", relative = true }
```

..cause such paths in the build output:

```
[esp-idf-sys 0.37.2]     esp_idf_sdkconfig: Some(
[esp-idf-sys 0.37.2]         "/home/ubuntu/ezb-node/apps/sdkconfig",
[esp-idf-sys 0.37.2]     ),
[esp-idf-sys 0.37.2]     esp_idf_sdkconfig_defaults: Some(
[esp-idf-sys 0.37.2]         [
[esp-idf-sys 0.37.2]             "/home/ubuntu/ezb-node/apps/raw/sdkconfig.defaults",
[esp-idf-sys 0.37.2]         ],
[esp-idf-sys 0.37.2]     ),
```

The path was not relative to the workspace, but the working folder.

**Work-around**

Since all folders where builds happen in this repo are "one step down" (`raw`|`api`|`apps`), we start the path with `..` to climb back.


## `esp-idf-sys` should *learn to give up*

>Related to above (relative paths rendered wrong?). 

IT WOULD HELP newcomers TREMENDOUSLY if `esp-idf-sys` terminated a build, if there's e.g. a non-existing path for either `sdkconfig` or `sdkconfig.defaults`. Why not?


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


