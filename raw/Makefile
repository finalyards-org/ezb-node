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
	# ~DEP_ESP_IDF_ROOT=/tmp/rust-target/riscv32imac-esp-espidf/release/build/esp-idf-sys-853a2667a8c024cb/out
  # ~ESP_IDF_VERSION=v5.5.3
  # MCU=esp32c6

_ESP_IDF_PATH=${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}
#R _SDKCONFIG_H:=$(shell echo "${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}" | sed 's^espressif/esp-idf/v[0-9.]+$$^^')/sdkconfig.h
#R_SDKCONFIG_H:=$(patsubst %/espressif/%,%,${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH})
_SDKCONFIG:=${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}/../../../sdkconfig
_SDKCONFIG_H:=${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}/../../../build/config/sdkconfig.h

_TARGET:=${MCU}

	# We need these:
	#		- ${_ESP_IDF_PATH}/components/esp_common/include/esp_err.h
	#		- ${_ESP_IDF_PATH}/components/esp_coex/lib/esp32c6/libcoexist.a
	#		- path to 'sdkconfig.h'

_ESP_ZIGBEE_SDK=../.sub/esp-zigbee-sdk
ESP_ZIGBEE_LIB=${_ESP_ZIGBEE_SDK}/components/esp-zigbee-lib

_INCLUDE:=${ESP_ZIGBEE_LIB}/include
_TMP:=./tmp

all:
	@false

# The targets that 'build.rs' normally builds (for debugging, in case make fails)
manual: ${_TMP}/bindings.rs
	@echo "\nManual build succeeded."

${_TMP}/sdkconfig: ${_SDKCONFIG}
	@cp $< $@

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
	| bindgen
	bindgen $< \
	  -- -I. -I${_INCLUDE} -I${_TMP} -Ifake \
		-I $(dir ${_SDKCONFIG_H}) \
		-I ${_ESP_IDF_PATH}/components/esp_common/include \
		-I ${_ESP_IDF_PATH}/components/esp_system/include \
		-I ${_ESP_IDF_PATH}/components/esp_hw_support/include \
		-I ${_ESP_IDF_PATH}/components/esp_driver_uart/include \
		-I ${_ESP_IDF_PATH}/components/hal/include \
		-I ${_ESP_IDF_PATH}/components/freertos/FreeRTOS-Kernel/include \
		-I ${_ESP_IDF_PATH}/components/freertos/FreeRTOS-Kernel/portable/riscv/include \
		-I ${_ESP_IDF_PATH}/components/freertos/config/include \
		-I ${_ESP_IDF_PATH}/components/freertos/config/riscv/include \
		-I ${_ESP_IDF_PATH}/components/esp_rom/include \
		-I ${_ESP_IDF_PATH}/components/esp_rom/include \
		-I ${_ESP_IDF_PATH}/components/esp_rom/include \
		-I ${_ESP_IDF_PATH}/components/soc/${_TARGET}/include \
		-I ${_ESP_IDF_PATH}/components/soc/${_TARGET}/include/soc \
	  > $@

#	  RUST_LOG='warn,bindgen::ir=error' \
#	  BINDGEN_EXTRA_CLANG_ARGS="--target=$(_CLANG_TARGET)" \
#		bindgen $< \
#	    --allowlist-file wrap58.h \
#	    --allowlist-type 'VL_.+' \
#	    --allowlist-function 'vl_check_data_ready' \
#	    --allowlist-function 'vl_set_external_sync_pin_enable' \
#	    --allowlist-function 'vl_(?:(get)|(set))_power_mode' \
#	    --allowlist-function 'vl_get_ranging_data' \
#	    --allowlist-function 'vl_init' \
#	    --allowlist-function 'vl_set_(?:(resolution)|(ranging_frequency_hz)|(integration_time_ms)|(sharpener_percent)|(target_order)|(ranging_mode))' \
#	    --allowlist-function 'vl_st(?:(art)|(op))_ranging' \
#	    --allowlist-item 'API_REVISION' \
#	    \
#	    --use-core \
#	    --default-enum-style rust \
#	    --raw-line '#![allow(non_camel_case_types)]' \
#	    --raw-line '#![allow(non_snake_case)]' \
#		--raw-line 'use strum::FromRepr;' \
#		--no-copy 'VL_(?:(Configuration)|(Platform))' \
#		\
#	    -- -I. -I$(_C_SRC) \
#	  | sed 's/#[[]repr(u32)[]]/#[repr(u8)]/' \
#	  | sed 's/#[[]derive(Debug, Copy, Clone, Hash, PartialEq, Eq)/#[derive(FromRepr, Copy, Clone, Hash, PartialEq, Eq)/' \
#	  | sed 's/these field, except for the sensor address."/these fields."/' \
#	  | sed -E 's/^(pub enum PowerMode)/#[allow(dead_code)]\n\1/' \
#	  | sed -E 's/(vl_set_power_mode)/_\1/' \
#	  > $@

	# Note: 'sed' removes 'Debug' from the derived behaviours. This is intentional; 'defmt' uses 'Format'.

	# Note: 'vl_external_sync_pin_enable' is L8CX specific.

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
