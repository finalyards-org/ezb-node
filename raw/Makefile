#
# Makefile
#
# Launched by 'build.rs' but also usable stand-alone (for debugging, development).
#
# Handles:
#	- compiling the 'esp-zigbee-sdk' C code
#	- creating Rust interfaces for it
#
# Requires:
#	- bindgen
#	- esp-idf-sys headers, libs; ensured by running 'cargo build' once.
#

include .BUILD_ENV
	# global:
	#		DEP_ESP_IDF_EMBUILD_ENV_PATH=[ ... ]:/home/ubuntu/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20251107/riscv32-esp-elf/bin:[ ... ]
	# 	DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH=/home/ubuntu/.espressif/esp-idf/v5.5.3
	# 	DEP_ESP_IDF_ROOT=/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-185221e30d16ee04/out
	# 	ESP_IDF_VERSION = "v5.5.3"
  # 	MCU=esp32c6
  #		OUT_DIR=/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-zb-raw-e55700f8a301711e/out
  #
  # out:
  #		...

_ESP_IDF_PATH:=${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}
#_SDKCONFIG:=${DEP_ESP_IDF_ROOT}/sdkconfig
	# ^-- good for checking what config got generated [DEBUG]
_SDKCONFIG_H:=${DEP_ESP_IDF_ROOT}/build/config/sdkconfig.h

# Only the 'DEP_ESP_IDF_EMBUILD_ENV_PATH' env.var. has the "/home/ubuntu/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20251107/riscv32-esp-elf/[...]"
# section that we need.
#
# Pick the particular piece
# 	GNU make note: Cannot use a pattern ('%') twice, but it's very good to match with the tail.
_RISCV_BIN_PATH := $(word 1,$(filter %/riscv32-esp-elf/bin,$(subst :, ,${DEP_ESP_IDF_EMBUILD_ENV_PATH})))
_SYSROOT:=$(realpath ${_RISCV_BIN_PATH}/..)
	# /home/ubuntu/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20251107/riscv32-esp-elf

# 'bindings_0.rs' *could* be created into the 'OUT_DIR' but in Multipass VM use, it's nice to see it in the IDE.
#_BIND_RS:=${OUT_DIR}/bindings.rs
_BIND_RS:=tmp/bindings_0.rs

ifeq (${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH},)
	$(error Missing 'DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH' from '.BUILD_ENV')
endif
ifeq (${DEP_ESP_IDF_ROOT},)
	$(error Missing 'DEP_ESP_IDF_ROOT' from '.BUILD_ENV')
endif
ifeq (${DEP_ESP_IDF_EMBUILD_ENV_PATH},)
	$(error Missing 'DEP_ESP_IDF_EMBUILD_ENV_PATH' from '.BUILD_ENV')
endif
ifeq (${OUT_DIR},)
	$(error Missing 'OUT_DIR' from '.BUILD_ENV')
endif

_TARGET:=${MCU}

_ESP_ZIGBEE_SDK=../.sub/esp-zigbee-sdk
ESP_ZIGBEE_LIB=${_ESP_ZIGBEE_SDK}/components/esp-zigbee-lib

_INCLUDE:=${ESP_ZIGBEE_LIB}/include
_TMP:=./tmp

all:
	@false

# The targets that 'build.rs' normally builds (for debugging, in case make fails)
manual: ${_BIND_RS}
	@echo "\nManual build succeeded."
	#cloc --quiet ${_BIND_RS}

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
#|${_TMP}/zb_vendor.h:
#|	@echo "#pragma once" > $@
#|	@echo "#define ZB_MANUFACTURER_CODE 0x1234" >> $@
#|	@echo "#define ZB_VENDOR_CLUSTER_ID 0xFC00" >> $@
#|	@echo "#define ZB_VENDOR_ENDPOINT   1" >> $@

#---
# Generate Rust
#
# Generate ’{output path}/raw.rs’. MANUAL CHANGES TO IT WILL BE LOST; modify this instead.
#
#esp_zigbee_attribute.h
#esp_zigbee_cluster.h
#esp_zigbee_endpoint.h
#nwk/esp_zigbee_nwk.h
#
${_BIND_RS}: wrap.h ${_INCLUDE}/*.h $(_OTHER_INCS) Makefile ${_SDKCONFIG_H} ./zb_vendor.h \
	| bindgen # _check_sdkconfig	todo: enabled '_check_sdkconfig' to see that the config is actually acted upon
	bindgen $< \
	    --allowlist-file wrap.h \
	    --allowlist-type 'esp_zb_.+' \
	    --allowlist-function 'esp_zb_.+' \
	    --allowlist-var 'ESP_ZB_.+' \
	    \
	    --allowlist-item 'ESP_ZB_VER' \
			--allowlist-type 'esp_zb_zcl_.*' \
			--allowlist-function 'esp_zb_zcl_.*' \
			\
			--allowlist-type 'esp_zb_zdo_.*' \
			--allowlist-function 'esp_zb_zdo_.*' \
			\
			--allowlist-type 'esp_zb_cluster_.*' \
			--allowlist-function 'esp_zb_cluster_.*' \
			\
			--allowlist-type 'esp_zb_attribute_.*' \
	    \
	    --use-core \
	    --generate-inline-functions \
	    --default-enum-style rust \
		-- \
		-D __riscv -D __riscv_xlen=32 --sysroot ${_SYSROOT} \
		\
	  -I. -I${_INCLUDE} -I${_TMP} -Ifake \
		-I $(dir ${_SDKCONFIG_H}) \
		-I ${_SYSROOT}/riscv32-esp-elf/include \
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
		-I ${_ESP_IDF_PATH}/components/log/include \
		-I ${_ESP_IDF_PATH}/components/newlib/platform_include \
		\
		-Wno-int-to-void-pointer-cast -Wno-void-pointer-to-int-cast \
	  > $@

#	    --allowlist-function 'abc' \
#	    --allowlist-function 'abc_(?:(def)|(ghi))_klm' \
#
# --bitfield-enum "ESP_ZB_BLAH_.*"  // jos on enum-maskeja
#	    --raw-line '#![allow(non_camel_case_types)]' \
#	    --raw-line '#![allow(non_snake_case)]' \
#			--raw-line 'use strum::FromRepr;' \
#			--no-copy 'VL_(?:(Configuration)|(Platform))' \


# Test the above (for manual development, only)
#
#		'grep -q' for strings that should be in.
#		'! grep -q' for those that should be omitted.
#
manual_test: ${_BIND_RS}
	#
	grep -q ESP_ZB_VER_MAJOR $<
  #
	! grep -q '_NEWLIB_VERSION_H__' $<
	@#
	@echo ""
	@echo "Yay!"

# Check that required tools are installed
bindgen:
	@which bindgen >/dev/null || ( \
	  echo >&2 "ERROR: 'bindgen' CLI not detected. Please install via 'cargo install --locked bindgen-cli'."; false \
	)

#---
_clean:
	-rm ${_BIND_RS}

echo:
	@echo ${_SDKCONFIG_H}
	@echo ${_ESP_IDF_PATH}

# Remove any targets created if a build fails.
.DELETE_ON_ERROR:

.PHONY: all manual manual_test bindgen _clean echo
