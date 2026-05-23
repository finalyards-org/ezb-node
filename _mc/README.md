# Understanding `menuconfig`

This `_mc` folder is not used by the rest of the repo.

The purpose is to be able to study and fine tune the ESP-IDF *configuration* used in building the Rust projects.

<!-- tbd. image

-->

## Roles of files

**`sdkconfig`**

This is the *output* file of an `esp-idf-sys` build, under `target`:

```
$ find ~/target -name sdkconfig
/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-d65dd9544361a2df/out/sdkconfig
```
>Note: Your `target` might be in the local folder, without a tilde.

It declares the *exact set of configuration* used in the build. It is not always what you expect, and therefore it is *important* that you know how to deal with it, either from the command line, or with the `menuconfig` tool.

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

These are the entries concerning virtual file system (`VFS`).

Note all the `y`'s (on). Disabled entries are mentioned only in passing, as a comment (`is not set`). There are no absolute `=n` denials (there can be those in the input, more about that later).

>Exercise: Try the same for `ZB_` (should find some; Zigbee specific), `WIFI` and so forth.


**`sdkconfig.defaults`**

Your project carries a `sdkconfig.defaults` file. This, alongside built-in defaults by `esp-idf-sys`, and transitive component requirements, define the eventual set of flags and values in a `sdkconfig`.

|what|who brings|can use `=n`|
|---|---|---|
|1. MCU defaults|`esp-idf-sys`|yes|
|2. `sdkconfig.defaults`|your project|yes, to override MCU defaults; but they can change to `=y` in later stage|
|3. transitive requirements|other components|no. They can turn `=n` into `=y`<!--, usually via `select`-->.|

*Table 1. Stages of producing an `sdkconfig`.* 

As you can see, it's a bit wobbly. What you mean by including `=n` in `sdkconfig.defaults` is not really "no", merely a "optioN", a *wish*:

>If it's fine for all (components), I, the author of the project, would *prefer* this setting to be off.

What happens on conflict?  The build does not stop. There's not even a warning. Your (option-like) `=n` gets turned into a `=y` in the final output. You have been warned (..or not).

<!-- R (not quite true/badly said)
>Note: This mess is due to ESP-IDF using the `=n` as a convention. On the contrary, KConfig itself (in other context) prefers only mentioning the enables, and leaving "nones" out, as comments. This is how the output `sdkconfig` looks like.
-->

## Enter `menuconfig`

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

<!--
>Note: The author uses separate VM's for the Cargo (`esp-idf-sys`) and manual ESP-IDF sessions.
-->

### Prep

```
$ idf.py set-target esp32c6
```

This is **important**. You need to set the target MCU within the project folder.

>`_mc` is akin to a traditional ESP-IDF project folder.


### Launching

You can either copy the `sdkconfig` file (from `target` output, see above) **to the `_mc` folder** (not project root, where the build could pick it up), or point to it with the `ESP_IDF_SDKCONFIG` env.var:

```
$ ESP_IDF_SDKCONFIG={path-to}/sdkconfig idf.py menuconfig
```

If the launch is good, you will see:

```
NOTICE: [1/2] espressif/esp-zigbee-lib (2.0.1)
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

If you are using Multipass VM, you can speed up the commands by **more than 10x** by:

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

**Warning:**

`/tmp` is cleared at VM restarts. Do this:

```
$ install -d /tmp/managed_components
```

If you don't, there's a Python error ahead!

> `/tmp/mc-build` seems to get automatically generated.

### `idf.py reconfigure`

If you "mess up", or e.g. edit the project files (within `_mc`), you can refresh the configuration seen by:

```
$ idf.py reconfigure
```

This was necessary in development.
