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
