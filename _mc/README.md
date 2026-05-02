# Understanding `menuconfig`

This `_mc` folder is not used by the rest of the repo.

The purpose is to be able to study and fine tune the ESP-IDF *configuration* used in building the Rust projects.

<!-- tbd. image
-->

## Roles of files

**`sdkconfig.defaults`**

This file gives the *intention* of the author, for which ESP-IDF capabilities should be activated. **It DOES NOT DIRECTLY STEER** the build: one cannot disable components with it, only pull them in, or configure them.

>Note: The "cannot disable" is a convention held by ESP-IDF. The KConfig format itself allows setting things off (`n`), but such lines will not matter, if some other component says otherwise.

**`sdkconfig`**

This is the *output* file somewhere under `target`:

```
$ find ~/target -name sdkconfig
/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-d65dd9544361a2df/out/sdkconfig
```

>Note: Your `target` might be in the local folder, without a tilde.

This lists *all* the configuration for ESP-IDF and has thousands of lines. **IT IS IMPORTANT** that you understand how to query it manually - and how to potentially edit it, with `menuconfig` tool.

```
$ cat `find ~/target -name sdkconfig` | grep VFS_
CONFIG_FATFS_VFS_FSTAT_BLKSIZE=0
CONFIG_VFS_SUPPORT_IO=y
CONFIG_VFS_SUPPORT_DIR=y
CONFIG_VFS_SUPPORT_SELECT=y
CONFIG_VFS_SUPPRESS_SELECT_DEBUG_OUTPUT=y
# CONFIG_VFS_SELECT_IN_RAM is not set
CONFIG_VFS_SUPPORT_TERMIOS=y
CONFIG_VFS_MAX_COUNT=8
CONFIG_VFS_SEMIHOSTFS_MAX_MOUNT_POINTS=1
CONFIG_VFS_INITIALIZE_DEV_NULL=y
```

>Note all the `y`'s (on). The one configuration not used is only mentioned as a comment: "`is not set`".

Here we manually checked that the `VFS` (virtual file system) configuration.

>Try the same for `ZB_` (should find some; Zigbee specific), `WIFI` and so forth.

## Using `menuconfig`

`menuconfig` is an ESP-IDF tool that allows you to *hierarchically view* a set of configurations.

Using it:

- ensures you would not make a typo, typing config entry names
- allows you to get an understanding of the *dynamic nature* of the configuration


### Requirements

- A working ESP-IDF 5.5.4 installation

	Make sure it's the same version as the one `esp-idf-sys` brings in.
	
	```
	$ idf.py --version
	ESP-IDF v5.5.4
	```

	>Note: The author uses separate VM's for the Rust (`esp-idf-sys`) and ESP-IDF environments.


### Prep

```
$ idf.py set-target esp32c6
```

This is **important**. You need to set the target MCU within the project folder.

>`_mc` is akin to a traditional ESP-IDF project folder.


### Launching

You can either copy the `sdkconfig` file (from `target` output, see above) to the `_mc` folder, or point to it with the `ESP_IDF_SDKCONFIG` env.var.

```
$ ESP_IDF_SDKCONFIG={path-to}/sdkconfig idf.py menuconfig
```

If the launch is good, you will see:

```
NOTICE: [1/2] espressif/esp-zigbee-lib (2.0.0)
NOTICE: [2/2] idf (5.5.4)
```

This means the external components have been included in the configuration.

![](.images/menuconfig-screen.png)

The interesting part is often under `Component config`.

- 👉 Learn to use the keyboard commands:

	- `C` "toggles show-name mode"
	- `/` "jump to symbol"

It should be fairly easy from here...

>Hint: Most of the interesting stuff takes place under `Component config`.


## Advanced

### 🤔Hint: speed up `menuconfig` on Multipass VM's

If you are using Multipass VM, you can speed up the commands by **10..x** by:

```
$ install -d /tmp/mc-build
$ ln -s /tmp/mc-build build
```

This keeps the build files (26MB of them) within the VM's filesystem.

```
$ install -d /tmp/managed_components
$ ln -s /tmp/managed_components managed_components
```

Does the same for the `esp-zigbee-lib` component (329MB).

>Together, these changed the author's `menuconfig` launch from crawling to light weight.


### `idf.py reconfigure`

If you "mess up", or e.g. edit the project files (within `_mc`), you can refresh the configuration seen by:

```
$ idf.py reconfigure
```

This was necessary in development.
