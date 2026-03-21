# Setting up ESP-IDF globally

---

You don't need to install ESP-IDF on your system - the `esp-idf-svc` crate takes care of installing a copy and using it, when necessary.

This file is notes by the author. Just in case you need to (e.g. running the C samples).

---

The `esp-zigbee-sdk` C code base needs the `esp-idf` toolchain to be installed. Here are short instructions.

Based on:

- [Developing with ESP Zigbee SDK](https://docs.espressif.com/projects/esp-zigbee-sdk/en/latest/esp32c6/developing.html) (Espressif docs)

Changes to above:

- We use a shallow git clone

>Note: Espressif leads you to "ESP-IDF Installation Manager (EIM)", but that's not needed.

---

## Requirements

- Ubuntu Linux

	```
	$ sudo apt install python3-venv
	```

	```
	$ sudo apt install cmake
	```

- `espflash` either directly or [via remoting](https://github.com/finalyards/probe-rs-remote)

	```
	$ cargo install espflash --locked
	```
	
	You can use `idf.py flash` if the device in question is directly connected to your development machine.

Installing takes around 4..5GB of disk space.

<!--
	|||
	|---|---|
	|`esp-idf` (git checkout)|1.9GB|
	|`~/.espressif` (toolchains)|1.4GB|
-->


## Installation

### Set up `esp-idf` 5.5.3

<!-- tbd.
As of March 2026, 6.0 is just released. 
-->

```
$ install -d ~/bin
```

The author likes placing `esp-idf` under `~/bin`, but that's just a personal preference. Use any path you like, but realize you'll keep the clone around as long as you are using the tool.

```
$ git clone --branch v5.5.3 --depth 1 --recursive https://github.com/espressif/esp-idf.git ~/bin/esp-idf
```

>Warn: ESP-IDF tag `v5.5` does NOT mean "latest 5.5.x" but 5.5.0. Do not use it.

<p />

>Taking a shallow clone uses up some 1.9GB (full likely lots more).

#### ..with MCU specific tools

```
$ ~/bin/esp-idf/install.sh esp32c6
```

This creates `~/.espressif` and downloads tools underneath there. 

>NOTE: Naming the target(s) explicitly saves plenty of disk space. The default is `all` Espressif MCUs.

### Set up the shell environment

```
$ . ~/bin/esp-idf/export.sh

Checking "python3" ...
Python 3.12.3
"python3" has been detected
Activating ESP-IDF 5.5
Setting IDF_PATH to '/home/ubuntu/bin/esp-idf'.
* Checking python version ... 3.12.3
* Checking python dependencies ... OK
* Deactivating the current ESP-IDF environment (if any) ... OK
* Establishing a new ESP-IDF environment ... OK
* Identifying shell ... bash
* Detecting outdated tools in system ... OK - no outdated tools found
* Shell completion ... Autocompletion code generated

Done! You can now compile ESP-IDF projects.
Go to the project directory and run:

  idf.py build
```

This needs to be done *separately* each time you intend to use `idf.py`. It sets the env.vars, paths etc. Now your shell is an "esp-idf shell".

>Note: The author does not recommend adding the `export.sh` command to `.bashrc`. That might feel tempting, but it's best to keep your default shell environment tidy.


### Test

```
$ idf.py --version
ESP-IDF v5.5.3
```


## Using

You now have `esp-idf` headers and libraries available under:

```
$ tree -L 1 ~/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20251107/riscv32-esp-elf/ 
/home/ubuntu/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20251107/riscv32-esp-elf/
├── bin
├── include
├── lib
├── libexec
├── package.json
├── picolibc
├── riscv32-esp-elf
└── share
```


### Within a project folder

```
$ idf.py set-target esp32c6
```

Although we only installed one target, this seems to be necessary.


### Builds

```
$ IDF_MINIMAL_BUILD=1 \
  CMAKE_BUILD_TYPE=Release idf.py build
[...]
[ 98%] Linking CXX executable color_light_bulb.elf
[...]
```

>Note: Asking for the minimal build (and release build) are not working. We'd like to. `#help! 🛟`

<p />

>Note: A release build likely happens faster than a debug one. <!-- tbd. measurements? -->

```
$ file build/*.elf
build/color_light_bulb.elf: ELF 32-bit LSB executable, UCB RISC-V, RVC, soft-float ABI, version 1 (SYSV), statically linked, with debug_info, not stripped
```

### Erasing NVRAM

Some READMEs request you to clear the NVRAM before running a sample. If you are using Multipass VM (and not something like USBIP), you cannot simply `idf.py erase-flash`.

The author uses [`probe-rs-remote`](https://github.com/finalyards/probe-rs-remote) - that also remotes the `espflash` command. Use this:

```
$ espflash erase-flash
```


### Flashing

With a locally connected device (ESP32-C6 devkit), you proceed with: 

```
$ idf.py flash monitor
```

If you do this, skip to next section.

---

If you are using Multipass VM (and not USBIP), you can use `espflash` (remoted, as mentioned above, see `probe-rs-remote`) to flash and run the output.

```
$ espflash write-bin 0x0      build/bootloader/bootloader.bin
$ espflash write-bin 0x8000   build/partition_table/partition-table.bin
$ espflash write-bin 0x10000  --monitor build/color_light_bulb.bin
[...]
I (338) main_task: Started on CPU0
I (338) main_task: Calling app_main()
I (338) phy_init: phy_version 331,5b89037,Mar  3 2025,16:01:12
I (388) phy: libbtbb version: ec2ecba, Mar  3 2025, 16:01:27
I (398) main_task: Returned from app_main()
[...]
```


### Uninstall (optional)

Just wipe the folders:

```
$ cd
$ rm -rf bin/esp-idf .espressif
```
