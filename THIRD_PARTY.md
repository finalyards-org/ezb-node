# 3rd party

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

><small>*`esp-zigbee-sdk` ..=2.1.0*</small>

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
    EZB_BDB_MODE_INITIALIZATION      = 0x01,    /*!< BDB initialization mode */
	[...]
} ezb_bdb_comm_mode_t;
```

However, the bindgen-generated output has the same line for both: `tmp/bindings_0.rs`

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

- [ ] Ask @claude to analyse this
- [ ] Does `bindgen` have a bug here, and isolate/explain it.

**Work-around**

Copying the description directly from C header.


## `UNREPORTBLE_ATTRIB`

><small>*`esp-zigbee-sdk` ..=2.1.0*</small>

```
    EZB_ZCL_STATUS_UNREPORTBLE_ATTRIB = 0x8cU,   /*!< Unreporttable attribute */
                           ^-- A missing                      ^-- excess t
```

>Note how there are *two* typos on the same line.

While some fields are obviously shortened (`UNSUP`), this seems like something to fix.


## `esp-zigbee-sdk`: could have third party examples

All the [examples](https://github.com/espressif/esp-zigbee-sdk/tree/main/examples) are between two ESP32 devices.

This is good for the point that no outside hardware is needed, but

..wouldn't it be greater to have at least one example binding with a commercial device?

Such a device can be "anything" (relatively common, well available globally, e.g. Sonoff). The point is to *expose the code* needed on the ESP32 side to tie with a commercial end device.


## `esp-zigbee-sdk`: observations on doc comments

```
// @return ESP_OK on success, error code otherwise.`
esp_zigbee_launch_mainloop
```

This function never returns, right?  Therefore, it would never give `ESP_OK`, nor is there a "success".

Or.. is that the code if the loop is terminated from outside; the comment could be improved.


## `esp-zigbee-sdk`: more detailed doc comment for `ezb_bdb_open_network`

Currently:

```
/**
 * @brief Open Zigbee network
 *
 * @param[in] permit_duration Zigbee network open time
 * @return
 *      - EZB_ERR_NONE: on success
 *      - EZB_ERR_NO_MEM: not memory
 *      - EZB_ERR_FAILED: on failed
 */
ezb_err_t ezb_bdb_open_network(uint8_t permit_duration);
```

*From version 2.1.0*

This can be improved, and *may* be missing vital information (role of `0xff`). 

1. mention the delay is in seconds
2. "Zigbee network open time" can be rephrased to be more explicit.

	>Open Zigbee network for joining
	>...
	>`permit_duration` (in seconds): The time the network will be available.

3. Is `0xff` a special value??

	Elsewhere, `esp-zigbee-lib/includes/ezbee/zdo/zdo_nwk_mgmt.h` has this (no, I did not find it, an agent did!):

	```
	    uint8_t permit_duration; /*!< Duration in seconds for which joining is permitted. 0x00 = disable joining, 0xFF = enable joining indefinitely */
	```

	If this field is related, does it mean that providing `0xff` above would place the device in "joining indefinitely" state?
	
	If so, that deserves to be mentioned in the doc comments.

	**Work-around**
	
	In my code, I ban `0xff` as a value.

>Note:
>
>These are kind of marginal, because all use cases in the repo are for 180s (3 min). But, still..


## `esp-zigbee-sdk`: note on ownership of pointer content

Especially for making Rust bindings, it would be great to know when a C level API code holds onto a pointed-to memory, after the call.

For example:

```
/**
 * @brief Trust Center: Add the install code for remote device with type.
 *
 * @param[in] address  The pointer to the 64-bit extended address of the remote device.
 * @param[in] ic_type  The install code type, @ref ezb_secur_ic_type_e.
 * @param[in] ic       The pointer to the data of install code.
 *
 * @return - EZB_ERR_NONE: on success; failed otherwise.
 */
ezb_err_t ezb_secur_ic_add(const ezb_extaddr_t *address, ezb_secur_ic_type_t ic_type, const uint8_t *ic);
```

The author's *guess* is that `address` (4 bytes) gets copied, but we cannot know, for sure.

The convention (covering *all* of the library APIs) could be:

- If a memory is *not* used after the call, mention it to be *by value*.

Another option could be to do this *per type*, defining in the declaration of `ezb_extaddr_t` that it's always going to be used by-value.

>For `ezb_extaddr_t`, in particular, a third option would be to move away from the current `union` approach to only `uint64` (which would be passed by value).


## `esp-zigbee-sdk`: more constness would be welcome

Now, using the C API from Rust, we need to use `mut` pointers, when there's nothing mutable about them. For example:

```
	value: (&raw mut value).cast(),
```

Instead, the library could use C `const` pointers. This would better describe the interface to users in C, as well.

**Work-around**

Not really.

It's all about cutting away complexity, and the right place to do the change is within the library.


## `esp-zigbee-sdk`: How to say `ezb_zcl_cluster_desc_t` may be `NULL`?

><small>*`esp-zigbee-sdk` ..=2.1.0*</small>

```
/**
 * @brief Get a cluster descriptor from an endpoint descriptor.
 *
 * @param[in] ep_desc    The endpoint descriptor to get the cluster descriptor from.
 * @param[in] cluster_id The identifier of the cluster to get the cluster descriptor for.
 * @param[in] role       The role of the cluster to get the cluster descriptor for.
 * @return The pointer to the cluster descriptor. See @ref ezb_zcl_cluster_desc_t, or EZB_INVALID_ZCL_CLUSTER_DESC if not found.
 */
ezb_zcl_cluster_desc_t ezb_af_endpoint_get_cluster_desc(const ezb_af_ep_desc_t ep_desc, uint16_t cluster_id, uint8_t role);
```

It's not very clear that the return code needs to be checked for null-ness. 

I checked:

```
#define EZB_INVALID_ZCL_CLUSTER_DESC EZB_INVALID_DESC
```

..and:

```
#define EZB_INVALID_DESC NULL /*!< Invalid descriptor. */
```

Now I know that a `NULL` must be checked againts.

As a programmer, I wonder what added value the indirection gains.

**Suggestions:**

- consider the `EZB_INVALID[_...]_DESC` usefulness.
- for this function:

	```
	@return The pointer to the cluster descriptor; `NULL` if not found.
	```

>**Addendum**
>
>In addition, `esp-zigbee-sdk` already itself compares to `NULL` (`cli_cmd_zcl.c`):
>
>```
>    if (cluster == NULL) {
>```

