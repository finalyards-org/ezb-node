# `esp-node-raw`

C/Rust interfacing level of [`esp-zigbee-lib`](https://github.com/espressif/esp-zigbee-sdk) APIs.

<!-- tbd. Excalidraw schematic
-->

## Foreword

You don't normally need to deal with this folder: the higher parts (`api`, `apps`) bring it in as a dependency.

Cases where you might want to spend time here:

- developing the C/Rust binding itself
- troubleshooting build problems

## Approach

We use the `bindgen` CLI explicitly, via a `Makefile`. The other alternative would be to integrate `bindgen` into the `esp-idf-sys` build stage (it does support creating bindings).

Pros/cons of the current approach:

|||
|---|---|
|**Pro**|
||Allows fine grained control of the `bindgen` stage.|
|**Con**|
||Requires CLI tools (`bindgen`, `make`) to be installed for *any* build (also those deriving from us, `api` and `apps`).|


## Requirements

- `bindgen` CLI
- GNU `make` >= 4.3

<!-- Author uses:
- bindgen 0.72.1
-->

## Build

```
$ cargo build --release -vv --features esp32c6
```


## Advanced

Alongside the build output, you should see this:

```
Build configuration: BuildConfig {
    esp_idf_tools_install_dir: Some(
        Global,
    ),
    esp_idf_sdkconfig: None,
    esp_idf_sdkconfig_defaults: Some(
        [
            "/home/ubuntu/esp-zb/sdkconfig.defaults",
        ],
    ),
    mcu: Some(
        "esp32c6",
    ),
    native: NativeConfig {
        esp_idf_version: Some(
            Tag(
                "v5.5.4",
            ),
        ),
        esp_idf_repository: None,
        esp_idf_cmake_generator: None,
        idf_path: None,
        extra_components: [
            ExtraComponent {
                component_dirs: [],
                remote_component: Some(
                    RemoteComponent {
                        name: "espressif/esp-zigbee-lib",
                        version: "1.*",
                        git: None,
                        path: None,
                        service_url: None,
                    },
                ),
                bindings_header: None,
                bindings_module: None,
                manifest_dir: "/home/ubuntu/esp-zb/raw",
            },
            ExtraComponent {
                component_dirs: [],
                remote_component: Some(
                    RemoteComponent {
                        name: "espressif/esp-zboss-lib",
                        version: "1.*",
                        git: None,
                        path: None,
                        service_url: None,
                    },
                ),
                bindings_header: None,
                bindings_module: None,
                manifest_dir: "/home/ubuntu/esp-zb/raw",
            },
        ],
        esp_idf_components: Some(
            [
                "cxx",
                "newlib",
                "freertos",
                "esp_hw_support",
                "heap",
                "log",
                "soc",
                "hal",
                "esp_rom",
                "esp_common",
                "esp_system",
                "riscv",
                "main",
                "driver",
                "nvs_flash",
                "wear_levelling",
                "fatfs",
                "spiffs",
            ],
        ),
        esp_idf_component_manager: None,
    },
    esp_idf_sys_root_crate: Some(
        "esp-zb-raw",
    ),
}
Using managed esp-idf repository: RemoteSdk { repo_url: None, git_ref: Tag("v5.5.4") }
```

Let's go through some of these things.

```
    esp_idf_tools_install_dir: Some(
        Global,
    ),
```

The ESP-IDF tooling is installed in `~/.espressif`. See ´.cargo/config.toml` (project root) for more info.

```
    esp_idf_sdkconfig: None,
```

We don't steer the location of the `sdkconfig` file. (The author never got this to work the way he preferred.)

```
    esp_idf_sdkconfig_defaults: Some(
        [
            "/home/ubuntu/esp-zb/sdkconfig.defaults",
        ],
```

The `sdkconfig.defaults` file provides *defaults* to certain ESP-IDF *components*. These steer what gets compiled, and the whole configuration output in `sdkconfig`.

>`sdkconfig` should be edited _only_ with `menuconfig` tool. More about it elsewhere (`DEVS/`, `mc/`).

```
    native: NativeConfig {
        esp_idf_version: Some(
            Tag(
                "v5.5.4",
            ),
        ),
        [...]
```

<!--
5.5.4 is currently (Apr'26) the latest ESP-IDF compatible with `esp-zigbee-sdk`. We'll move on to whichever is the latest supported one.
-->
5.5.5 is currently (Aug'26) the latest ESP-IDF compatible with `esp-zigbee-sdk`.

>Each ESP-IDF version installs their own tools, in our case to `~/.espressif`.

```
        extra_components: [
            ExtraComponent {
                component_dirs: [],
                remote_component: Some(
                    RemoteComponent {
                        name: "espressif/esp-zigbee-lib",
                        version: "1.*",
                        git: None,
                        path: None,
                        service_url: None,
                    },
                ),
                bindings_header: None,
                bindings_module: None,
                manifest_dir: "/home/ubuntu/esp-zb/raw",
            },
            ExtraComponent {
                component_dirs: [],
                remote_component: Some(
                    RemoteComponent {
                        name: "espressif/esp-zboss-lib",
                        version: "1.*",
                        git: None,
                        path: None,
                        service_url: None,
                    },
                ),
                bindings_header: None,
                bindings_module: None,
                manifest_dir: "/home/ubuntu/esp-zb/raw",
            },
        ],
```

Important that these exist!! Shows that the ESP-IDF system has picked up `esp-zigbee-lib` and its ZBOSS dependency. These are the Zigbee libraries we want to work with.

>The components are downloaded to:
>
>`{target path}/riscv32imac-esp-espidf/release/build/esp-idf-sys-85c99a2b1216aeac/out/managed_components`
>
>Hint: To find the path, do `find ~/target -name managed_components`. 
>You can also see the `.BUILD_ENV` file (generated by `build.rs`) for its `DEP_ESP_IDF_ROOT` line.

```
        esp_idf_components: Some(
            [
                "cxx",
                "newlib",
                "freertos",
                "esp_hw_support",
                "heap",
                "log",
                "soc",
                "hal",
                "esp_rom",
                "esp_common",
                "esp_system",
                "riscv",
                "main",
                "driver",
                "nvs_flash",
                "wear_levelling",
                "fatfs",
                "spiffs",
            ],
        ),
```

This array is populated if the build constraints the number of ESP-IDF components. We'd do this simply to speed up the build - and to reduce the size of the output binary.

The array can also be empty.

```
        esp_idf_component_manager: None,
```

Not sure what that means. Things work without.


## Running `manual`

```
$ make manual
```

This creates the binding output:

```
$ file tmp/bindings_0.rs 
tmp/bindings_0.rs: ASCII text, with very long lines (340)
```

That file is used within the Rust portion of this crate.


<!--R
### `global` vs. `out`

This is a setting that affects where the build output files (Espressif tools, in particular) are placed.

Steered in `.cargo/config.toml`:

```
ESP_IDF_TOOLS_INSTALL_DIR = "out"
#ESP_IDF_TOOLS_INSTALL_DIR = "global"
```

>As mentioned in the comments in that file:

**`out`**: keep tools within the Rust `target`; `cargo clean` also clears tool selection (and rebuild means they will be downloaded and installed again).

**`global`**: keep tools apart, in `~/.espressif`. This is useful if you change deep build settings (e.g. test the `sdkconfig` generation) but don't need the tools to be reset each time.

>For release, it's likely best to keep it as `out`. This way, a dependency causing our build will not create folders outside of the usual build target.
-->