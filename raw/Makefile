#
# Makefile
#
# Launched by 'build.rs' but also usable stand-alone (debugging, development).
#
# Handles:
#	- compiling the 'esp-zigbee-sdk' C code
#	- creating Rust interfaces for it
#
# Requires:
#	- bindgen
#	- esp-idf-sys headers, libs
#

include .BUILD_ENV
	# DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH=/tmp/rust-target/riscv32imac-esp-espidf/release/build/esp-idf-sys-853a2667a8c024cb/out/espressif/esp-idf/v5.5.3
	# DEP_ESP_IDF_ROOT=/tmp/rust-target/riscv32imac-esp-espidf/release/build/esp-idf-sys-853a2667a8c024cb/out
	# ESP_IDF_TOOLS_INSTALL_DIR = "{global|out}"
  # MCU=esp32c6

_ESP_IDF_PATH:=${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}
_SDKCONFIG:=${DEP_ESP_IDF_ROOT}/sdkconfig
_SDKCONFIG_H:=${DEP_ESP_IDF_ROOT}/build/config/sdkconfig.h

# Be a little flexible on the '.cargo/config.toml' setup.
# Note: Is there a way we could just get this from 'build.rs' env, directly? (especially "esp-14.2.0_20251107" is now fragile)
ifeq (${ESP_IDF_TOOLS_INSTALL_DIR},out)
	_ESP_IDF_TOOLCHAIN_PATH:=${DEP_ESP_IDF_ROOT}/espressif/tools/riscv32-esp-elf/esp-14.2.0_20251107/riscv32-esp-elf
else ifeq (${ESP_IDF_TOOLS_INSTALL_DIR},global)
	_ESP_IDF_TOOLCHAIN_PATH:=~/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20251107/riscv32-esp-elf
else
	$(error Unexpected 'ESP_IDF_TOOLS_INSTALL_DIR': ${ESP_IDF_TOOLS_INSTALL_DIR})
endif

_TARGET:=${MCU}

_ESP_ZIGBEE_SDK=../.sub/esp-zigbee-sdk
ESP_ZIGBEE_LIB=${_ESP_ZIGBEE_SDK}/components/esp-zigbee-lib

_INCLUDE:=${ESP_ZIGBEE_LIB}/include
_TMP:=./tmp

all:
	@false

# The targets that 'build.rs' normally builds (for debugging, in case make fails)
manual: ${_TMP}/bindings.rs
	@echo "\nManual build succeeded."

# Eventually (?) 'zb_vendor.h' should be generated based on configuration from the *Rust program* using us, but
# that's later.
#
# Example (courtesy Copilot):
#		<<
#		#define ZB_MANUFACTURER_CODE 0x1234
#   #define ZB_VENDOR_CLUSTER_ID 0xFC00
#   #define ZB_VENDOR_ENDPOINT   1
#		<<
#
${_TMP}/zb_vendor.h:
	@echo "#pragma once" > $@
	@echo "#define ZB_MANUFACTURER_CODE 0x1234" >> $@
	@echo "#define ZB_VENDOR_CLUSTER_ID 0xFC00" >> $@
	@echo "#define ZB_VENDOR_ENDPOINT   1" >> $@

#---
# Generate Rust
#
# Generate ’tmp/bindings.rs’. MANUAL CHANGES TO IT WILL BE LOST; modify this instead.
#
${_TMP}/bindings.rs: wrap.h ${_INCLUDE}/*.h $(_OTHER_INCS) Makefile ${_SDKCONFIG_H} ${_TMP}/zb_vendor.h \
	| bindgen # _check_sdkconfig	todo: enabled '_check_sdkconfig' to see that the config is actually acted upon
	bindgen $< \
		-- \
		-D __riscv -D __riscv_xlen=32 --sysroot ${_ESP_IDF_TOOLCHAIN_PATH} \
		\
	  -I. -I${_INCLUDE} -I${_TMP} -Ifake \
		-I $(dir ${_SDKCONFIG_H}) \
		-I ${_ESP_IDF_PATH}/components/esp_common/include \
		-I ${_ESP_IDF_PATH}/components/hal/include \
		-I ${_ESP_IDF_PATH}/components/soc/${_TARGET}/include \
		-I ${_ESP_IDF_PATH}/components/esp_driver_uart/include \
		-I ${_ESP_IDF_PATH}/components/esp_hw_support/include \
		-I ${_ESP_IDF_PATH}/components/freertos/FreeRTOS-Kernel/include \
		-I ${_ESP_IDF_PATH}/components/freertos/config/include \
		-I ${_ESP_IDF_PATH}/components/freertos/config/riscv/include \
		-I ${_ESP_IDF_PATH}/components/freertos/FreeRTOS-Kernel/portable/riscv/include \
		-I ${_ESP_IDF_PATH}/components/riscv/include \
		-I ${_ESP_IDF_PATH}/components/soc/${_TARGET}/register \
		-I ${_ESP_IDF_PATH}/components/esp_rom/include \
		-I ${_ESP_IDF_PATH}/components/heap/include \
		-I ${_ESP_IDF_PATH}/components/esp_system/include \
		-I ${_ESP_IDF_PATH}/components/ieee802154/include \
		-I ${_ESP_IDF_PATH}/components/newlib/platform_include \
		-I ${_ESP_IDF_TOOLCHAIN_PATH}/riscv32-esp-elf/include \
	  > $@

# Test for the above (for manual development, only); CAN BE LET GO
EXP_x5: tmp/uld_raw5.rs
	grep -q API_REVISION $<
	grep -q vl_init $<
	grep -q vl_get_ranging_data $<
	! grep -q 'static VL_FIRMWARE' $<
	! grep -q vl_is_alive $<
	! grep -q vl_get_resolution $<
	! grep -q VL_RdByte $<
	! grep -q 'these field, except ' $<
	@#
	@echo ""
	@echo "Yay!"

	#grep -q 'pub enum PowerMode' $<

#---
# Check the quality of 'sdkconfig'
#		- it should reflect the 'sdkconfig.defaults'
#
_check_sdkconfig: ${_SDKCONFIG}
	@( \
		grep -xq 'CONFIG_ESP_WIFI_ENABLED=n' $< && \
 		grep -xq 'CONFIG_FATFS_ENABLED=n' $< && \
 			true ) || ( \
	  echo >&2 "ERROR: 'sdkconfig.defaults' IS NOT REFLECTED in the actual configuration: ${_SDKCONFIG}"; false \
	)

# Check that required tools are installed
bindgen:
	@which bindgen >/dev/null || ( \
	  echo >&2 "ERROR: 'bindgen' CLI not detected. Please install via 'cargo install --locked bindgen-cli'."; false \
	)

#---
_clean:
	-rm ${_TMP}/bindings.rs

echo:
	@echo ${_SDKCONFIG}

# Remove any targets created if a build fails.
.DELETE_ON_ERROR:

.PHONY: all bindgen clang _clean echo
