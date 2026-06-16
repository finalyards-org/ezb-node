# ESP-IDF builds

There are a *lot* of things that could be said about building a Rust project with ESP-IDF backend support (i.e. FreeRTOS).

Here are some. Note that these are "developer notes" - not intended to be a full documentation or tutorial. Not necessarily in order. Hope they help a bit.


## Observe the build output

```
$ cargo build --release -vv
```

When you see the build output, pay attention to certain parts of it. Especially here:

```
[esp-idf-sys 0.37.2] Build configuration: BuildConfig {
[esp-idf-sys 0.37.2]     esp_idf_tools_install_dir: Some(
[esp-idf-sys 0.37.2]         Global,
[esp-idf-sys 0.37.2]     ),
[esp-idf-sys 0.37.2]     esp_idf_sdkconfig: None,
[esp-idf-sys 0.37.2]     esp_idf_sdkconfig_defaults: Some(
[esp-idf-sys 0.37.2]         [
[esp-idf-sys 0.37.2]             "sdkconfig.defaults",
[esp-idf-sys 0.37.2]         ],
[esp-idf-sys 0.37.2]     ),
[esp-idf-sys 0.37.2]     mcu: Some(
[esp-idf-sys 0.37.2]         "esp32c6",
[esp-idf-sys 0.37.2]     ),
[esp-idf-sys 0.37.2]     native: NativeConfig {
[esp-idf-sys 0.37.2]         esp_idf_version: Some(
[esp-idf-sys 0.37.2]             Tag(
[esp-idf-sys 0.37.2]                 "v5.5.3",
[esp-idf-sys 0.37.2]             ),
[esp-idf-sys 0.37.2]         ),
[esp-idf-sys 0.37.2]         esp_idf_repository: None,
[esp-idf-sys 0.37.2]         esp_idf_cmake_generator: None,
[esp-idf-sys 0.37.2]         idf_path: None,
[esp-idf-sys 0.37.2]         extra_components: [],
[esp-idf-sys 0.37.2]         esp_idf_components: None,
[esp-idf-sys 0.37.2]         esp_idf_component_manager: None,
[esp-idf-sys 0.37.2]     },
[esp-idf-sys 0.37.2] cargo:warning=the crate given by `ESP_IDF_SYS_ROOT_CRATE` does not exist in this workspace
[esp-idf-sys 0.37.2]     esp_idf_sys_root_crate: Some(
[esp-idf-sys 0.37.2]         "ezb-node-raw",
[esp-idf-sys 0.37.2]     ),
[esp-idf-sys 0.37.2] }
```

>Press Ctrl-S to stop the output so you can study such a section.

1. `esp_idf_tools_install_dir:` [...] `Global`

	This shows that tools are being installed to `~/.espressif`.
	
	The tool installation *will not* be cleared with `cargo clean` (as it would if this is `Out`). Suitable for development use.

2. `esp_idf_sdkconfig:`

	Not sure, in which cases one would provide their own `sdkconfig`. We provide the `sdkconfig.defaults` (next).
	
3. `esp_idf_sdkconfig_defaults:`

	Provides **defaults** to ESP-IDF configuration. Note that this is (by convention) used to place things "on", but **cannot be used for banning options** - if there is a component somewhere alongside the build chain that enables a config, it does get pulled in.
	
	>This means you cannot "force" e.g. WLAN to be off (though we'd like).
	
	>In general, `cargo` builds use only the **topmost build environment**. Dependencies bring their code and `build.rs` - but not the env.vars. or build metadata. This is a design decision by cargo and we need to live with it. The ESP-IDF toolchain matches rather poorly with it (and/or the author hasn't figured this out, yet!!!).
	
4. `mcu:`

	Self-evident.
	
5. `esp_idf_version:`

	Shows the ESP-IDF version you wish to use.
	
	Keep an eye on this, casually. If you see something other than you expected, the configuration is somehow faulty.

>Note: Pay special attention to the `sdkconfig.defaults` field. When we use "relative" paths (see `.cargo/config.espidf.conf`), there may be absolute **non-existing** paths here. Spot them in time and you'll save your sorries, later!
>
>```
>[esp-idf-sys 0.37.2]     esp_idf_sdkconfig: Some(
>[esp-idf-sys 0.37.2]         "/home/ubuntu/ezb-node/apps/sdkconfig", # NOT INTENDED
>[esp-idf-sys 0.37.2]     ),
>[esp-idf-sys 0.37.2]     esp_idf_sdkconfig_defaults: Some(
>[esp-idf-sys 0.37.2]         [
>[esp-idf-sys 0.37.2]             "/home/ubuntu/ezb-node/apps/raw/sdkconfig.defaults",   # <-- NO SUCH THING!
>[esp-idf-sys 0.37.2]         ],
>[esp-idf-sys 0.37.2]     ),
>```

### Warnings in the output
	
```
cargo:warning=the crate given by `ESP_IDF_SYS_ROOT_CRATE` does not exist in this workspace
[esp-idf-sys 0.37.2]     esp_idf_sys_root_crate: Some(
[esp-idf-sys 0.37.2]         "...",
[esp-idf-sys 0.37.2]     ),
```

Keep an eye open for such! You should not see these warnings - it would actually be an error:

**Note that `esp-sys-idf` is overly tolerant on errors! It just spits warnings, uses some defaults, and keeps going!!** This feels very unusual for a Cargo project, because in Rust, it's normally "explicit over implicit".


### `ldproxy`
	
Have `ldproxy` CLI installed: `cargo install ldproxy`.


### `warning: unknown kconfig symbol`

```
[esp-idf-sys 0.37.2] warning: unknown kconfig symbol 'ESP_WIFI_STA_SUPPORT' assigned to 'n' in /home/ubuntu/esp-zb/sdkconfig.defaults
[esp-idf-sys 0.37.2] warning: unknown kconfig symbol 'COEX_ENABLED' assigned to 'n' in /home/ubuntu/esp-zb/sdkconfig.defaults
[esp-idf-sys 0.37.2] warning: unknown kconfig symbol 'HTTPD_ENABLED' assigned to 'n' in /home/ubuntu/esp-zb/sdkconfig.defaults
[esp-idf-sys 0.37.2] warning: unknown kconfig symbol 'MQTT_ENABLED' assigned to 'n' in /home/ubuntu/esp-zb/sdkconfig.defaults
[esp-idf-sys 0.37.2] warning: unknown kconfig symbol 'FATFS_ENABLED' assigned to 'n' in /home/ubuntu/esp-zb/sdkconfig.defaults
[esp-idf-sys 0.37.2] warning: unknown kconfig symbol 'LCD_ENABLED' assigned to 'n' in /home/ubuntu/esp-zb/sdkconfig.defaults
[esp-idf-sys 0.37.2] warning: unknown kconfig symbol 'USB_ENABLED' assigned to 'n' in /home/ubuntu/esp-zb/sdkconfig.defaults
```

Watch out for this kind of warnings in the build output.

Again, the `ESP-IDF` build chain has an **uncomftably high tolerance for configuration mistakes**. If a configuration key is misspelled, it is simply ignored - with a warning like above. 

So watch your build output. Grep it for the word "warning".

👉 If a project builds, it does not mean it built in the **intended** way.



## Single-threaded build

Errors that fail a build are not necessarily at the tail of the console output.

This is because ESP-IDF build tools use multiple threads. Look for "Error" or "FAIL" somewhere in your console output for details.

Example:

```
  CMake Error: The source "/home/ubuntu/.espressif/esp-idf/v5.5.3/components/bootloader/subproject/CMakeLists.txt" does not match the source "/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-c5e8818c8dd375d3/out/espressif/esp-idf/v5.5.3/components/bootloader/subproject/CMakeLists.txt" used to generate cache.  Re-run cmake with a different source directory.
[...50 lines...]
  command did not execute successfully, got: exit status: 1

  build script failed, must exit now
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

>NOTE: Having color coding in the output would help a lot. Unfortunately, the errors/warnings within the ESP-IDF portion don't often get any highlighting.

- Is there a way to force single-threaded build?

	```
	$ NINJAFLAGS = "-j1" {...build command...}
	```
	
	```
	$ CMAKE_BUILD_PARALLEL_LEVEL=1 {...build command...}
	```
	
	<!-- Likely not needed; Rust/Cargo:
	```
	$ CARGO_BUILD_JOBS=1 {...build command...}
	```
	-->
	
	```
	$ ESP_IDF_SYS_CMAKE_GENERATOR="Unix Makefiles" {...build command...}
	```
	
	Try those.	


## Qualifying `sdkconfig.defaults`

See `_mc/` for tools and info on how to:

- detect which configuration keys are available
- what their values after a build have become

This helps you to pick the right keys for `sdkconfig.defaults` and hopefully avoid components that are not needed.


## `components_esp32c6.lock`

This file is used by `esp-idf-sys` builds

```
$ more components_esp32c6.lock 
dependencies:
  espressif/esp-zigbee-lib:
    component_hash: 823ee1604d896bdcfd3d5dbe2b1eb84fdf089cffcbbe50e46d01d647861710ba
    dependencies:
    - name: idf
      require: private
      version: '>=5.0'
    source:
      registry_url: https://components.espressif.com/
      type: service
    version: 2.0.0
  idf:
    source:
      type: idf
    version: 5.5.4
direct_dependencies:
- espressif/esp-zigbee-lib
manifest_hash: 4a9a19333e88ba7873691c6603cbb1190f6a997bc299616cd41835923c76de09
target: esp32c6
version: 2.0.0
```

## `project_description.json`: `CONFIG_ONLY´ or `LIBRARY`

```
/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-6fedde765f63b3e3/out/build/project_description.json
```

If the `sdkconfig.defaults` file is not properly found, build proceeds but the Zigbee component is marked as `CONFIG_ONLY` (whatever that means?).

Symptoms:

- `esp_zigbee_lib` symbols are not found at linkage

Cure:

- observe what the build configuration states about `sdkconfig.defaults`
- fix the path

