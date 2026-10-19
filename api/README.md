# `ezb-node`

This is the API layer for your applications.

**Focus**

- What features are brought further, from `raw` 1:1 APIs.
- How they are expressed in Rust.
- Consumes the Zigbee node configuration (`config`) and initializes the node by it.

**Non-focus**

- (place holder)

## Steps

```
$ just build
```

Builds the library.

## Using in your application

### Initializing flash

```
	// should do the same as C 'nvs_flash_init()'
    let a = EspDefaultNvsPartition::take()?;

	// Do not drop the handle
	core::mem::forget(a);
```

>If you wish to use a partition other than `"nvs"` (this is likely needed only for backwards compatibility), you should also:
>
>```
>	// Initialize a named partition; should be like C 'nvs_flash_init_partition(name)'.
>	let b = EspCustomNvsPartition::take(name)?;
>	
>	// Do not drop the handle
>	core::mem::forget(b);
```

### Launch the node

```
    let zb_node = IasCoordinator::new()
        .unwrap_or_else(|e| panic!("Initialization of Zigbee failed: {:?}", e));

    zb_node.run() .await;
```

See the samples at `apps/`. 

