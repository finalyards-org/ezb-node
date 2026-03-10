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

- Around 4GB of free disk space

<!--
	|||
	|---|---|
	|`esp-idf` (git checkout)|1.9GB|
	|`~/.espressif` (toolchains)|1.4GB|
-->

## Steps

### Set up `esp-idf` 5.5

```
$ install -d ~/bin
```

The author likes placing `esp-idf` under `~/bin`, but that's just a personal preference. Use any path you like, but realize you'll keep the clone around as long as you are using the tool.

```
$ git clone --branch v5.5 --depth 1 --recursive https://github.com/espressif/esp-idf.git ~/bin/esp-idf
```

>Taking a shallow clone uses up some 1.9GB (full likely lots more).

#### ..with MCU specific tools

```
$ ~/bin/esp-idf/install.sh esp32c6
```

This creates `~/.espressif` and downloads tools underneath there. Naming the target explicitly saves you disk space.

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
ESP-IDF v5.5
```

## What next?

You now have `esp-idf` headers and libraries available under `~/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20241119/riscv32-esp-elf/`.


### Within a project folder

```
$ idf.py set-target esp32c6
```

Although we only installed one target, this seems to be necessary.


### Builds

```
$ idf.py build
[...]
```

The output is in `build/*.elf`. You can flash it with standard tools like `probe-rs`:

```
$ probe-rs run build/green.elf
[...]
```

### Uninstall (optional)

Just wipe the folders:

- `bin/esp-idf`
- `~/.espressif`

