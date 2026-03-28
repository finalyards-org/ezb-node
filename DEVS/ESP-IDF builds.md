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
[esp-idf-sys 0.37.2]         "raw",
[esp-idf-sys 0.37.2]     ),
[esp-idf-sys 0.37.2] }
```

>Press Ctrl-S to stop the output so you can study such a section.

1. `esp_idf_tools_install_dir:` [...] `Global`

	This shows that tools are being installed to `~/.espressif`.
	
	The tool installation *will not* be cleared with `cargo clean` (as it would if this is `Out`). Suitable for development use.

2. `esp_idf_sdkconfig:`

	We haven't defined one, but letting `esp-idf-sys` to generate such.
	
	>The author does not currently fully comprehend the `sdkconfig` workflow. If you know more, input, opinions and PRs are appreciated!!
	
3. `esp_idf_sdkconfig_defaults:`

	Provides **defaults** to ESP-IDF configuration, but **if there is a component somewhere alongside the build chain that defines a config, its value gets used**.
	
	This means you cannot "force" e.g. WLAN to be off (though we'd like). Consider the contents of such a file as very indicative, not authoritive.
	
	>In general, `cargo` builds use only the **topmost build environment**. Dependencies bring their code and `build.rs` - but not the env.vars. or build metadata. This is a design decision by cargo and we need to live with it. The ESP-IDF toolchain matches rather poorly with it (and/or the author hasn't figured this out, yet!!!).
	
4. `mcu:`

	Self-evident.
	
5. `esp_idf_version:`

	Shows the ESP-IDF version you wish to use.
	
	Keep an eye on this, casually. The `esp-idf-sys` toolchain defaults to 5.2 (at the moment, Mar'26) - if you see that the configuration is somehow faulty.
	
```
cargo:warning=the crate given by `ESP_IDF_SYS_ROOT_CRATE` does not exist in this workspace
[esp-idf-sys 0.37.2]     esp_idf_sys_root_crate: Some(
[esp-idf-sys 0.37.2]         "raw",
[esp-idf-sys 0.37.2]     ),
```

You should not see this - but taking it here as a sample on how build warnings/errors would show in the log.

```
[esp-idf-sys 0.37.2] pip 24.0 from /home/ubuntu/esp-zb/.embuild/espressif/python_env/idf5.5_py3.12_env/lib/python3.12/site-packages/pip (python 3.12)
```

Note the `.embuild` created *under* the project folder.

>This is because (in this build) the `esp_idf_tools_install_dir` was NOT DEFINED. That led to using `.embuild/espressif` within the project folder.

Other options:

- `esp_idf_tools_install_dir: "out"`

	```
	[esp-idf-sys 0.37.2] Cloning into '/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-c5e8818c8dd375d3/out/espressif/esp-idf/v5.5.3'...
	```

	The tooling goes to `{Cargo output folder}/espressif/`

- `esp_idf_tools_install_dir: "global"`

	```
	... 
	```
	
	<!-- tbd. try, paste a line above -->
	
	The tooling goes to `~/.espressif/`.

	For using `"global"`, you also need to:

	```	
	[target.'cfg(target_os = "espidf")']
	linker = "ldproxy"
	```
	
	have `ldproxy` CLI installed (`cargo install ldproxy`).

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

In general, **`ESP-IDF` build chain** has an **uncomftably high tolerance for configuration mistakes**. In Rust, things normally break when there are ambiguities. Not so here!

So watch your build output. Grep it for the word "warning".

If a project builds, it does not mean it build in the **intended** way.



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

>NOTE: Having color coding in the output would help a lot. Unfortunately, the errors don't usually get any highlighting.

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

The ESP-IDF build treats non-existent keys in `sdkconfig.defaults` with a **warning**, not an error.

This can cause quite some time to be wasted.

Ideally, we'd build with these being flagged out. Until then, do this:

- `sdkconfig.json` within the output folder carries the configuration that actually made it:

	```
	{target folder}/riscv32imac-esp-espidf/release/build/esp-idf-sys-c5e8818c8dd375d3/out/build/config/sdkconfig.json
	```

	Grep that file, after a build, to see whether your intended values made it - or whether those keys are even listed.


