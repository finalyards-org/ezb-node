# Using `menuconfig`

The `sdkconfig` file (within the version control) defines what ESP-IDF behaviour gets compiled and linked in.

You don't need to do anything about it for the examples, but in case you are curious...


## Preparation

There could be a way to utilize `menuconfig` from the checkout (of ESP-IDF) that `esp-idf-sys` has done (to the `~/.espressif` folder, by this repo's default).

However, it's not obvious.

Also, good to realize that you *should not* install `idf.py` (i.e. the ESP-IDF command line tools) manually in the environment you also use for Rust `esp-idf-sys`. Keep them separate.

The author works like this:

- has a *separate VM* that's for C exercises, and can be used for running `menuconfig`

	- the same source folder is shared with both the Rust and ESP-IDF/C VM's.

This allows him to edit the `sdkconfig` (for Rust builds) from the ESP-IDF/C VM.

### Installation

See [`ESP-IDF - Setting up.md`](ESP-IDF%20-%20Setting%20up.md) for instructions. 

>[IMPORTANT] Make sure you use the *same version number* (e.g. 5.5.4) for both the Rust and the ESP-IDF/C side!

```
$ idf.py --version
ESP-IDF v5.5.4-dirty
```

>Note: If you need to *update* your ESP-IDF, the above document has a section ("Update") on that, as well! :)

#### Three routes further

The `google.ai` AI suggested three ways to run `menuconfig` within a Rust project.

>Please consult it, or another AI helper, for up-to-date details?

1. **Via `cargo-pio`**

	```
	$ cargo install cargo-pio
	```

	```
	$ cargo pio espidf menuconfig
	```

	>[WARN] The author did not try these commands.

2. **Dummy CMake project**

	Create `CMakeLists.txt`:
	
	```
	cmake_minimum_required(VERSION 3.5)
	include($ENV{IDF_PATH}/tools/cmake/project.cmake)
	project(dummy-project)
	```

	..and an empty `main/CMakeLists.txt`:
	
	```
	$ install -d main
	$ touch main/CMakeLists.txt
	```

3. **vscode-esp-idf-extension**

	Seems to have a built-in `menuconfig` editor?
	
	>The author did not try.


## Steps
 
You launch `menuconfig` by:

```
$ idf.py menuconfig
```

![](.images/menuconfig-screen.png)


## Hints





## Differences to not having an `sdkconfig`

The settings the author squeezed.

### Before

```
App/part. size:    388,672/4,128,768 bytes, 9.41%
```

### After


Differences to not 