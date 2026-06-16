#
# Makefile
#
# Launched by 'build.rs' but also usable stand-alone (for debugging, development).
#
# NOTE: C/Rust bindings can ALSO be done fully within 'esp-idf-sys'; this can be something to consider in the future.
#
# Requires:
#	- bindgen CLI
#	- esp-idf-sys headers, libs; ensured by running 'cargo build' once.
#		- these are written to '.BUILD_ENV'
#		- ..which also shows where 'esp-idf-sys' has placed the Zigbee component's headers
#

include .BUILD_ENV
	# contains ("global" mode):
	#		DEP_ESP_IDF_EMBUILD_ENV_PATH=[ ... ]:/home/ubuntu/.espressif/tools/riscv32-esp-elf/esp-14.2.0_20260121/riscv32-esp-elf/bin:[ ... ]
	#		DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH=/home/ubuntu/.espressif/esp-idf/v5.5.4
	#		DEP_ESP_IDF_ROOT=/home/ubuntu/target/riscv32imac-esp-espidf/release/build/esp-idf-sys-661d76f25670d30a/out
	#		ESP_IDF_TOOLS_INSTALL_DIR=global
	#		ESP_IDF_VERSION=5.5.4
	#		MCU=esp32c6

_ESP_IDF_PATH:=${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}
_SDKCONFIG:=${DEP_ESP_IDF_ROOT}/sdkconfig
	# ^-- good for checking what config got generated [DEBUG]
_SDKCONFIG_H:=${DEP_ESP_IDF_ROOT}/build/config/sdkconfig.h
_ESP_ZIGBEE_LIB_INCLUDE:=\
	${DEP_ESP_IDF_ROOT}/managed_components/espressif__esp-zigbee-lib/include
		# Note: for 'esp-zigbee-sdk' 1.x we needed more than one include path. Keep the setup, for now...

_NVS_COMPONENTS_INCLUDE:=\
  ${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}/components/nvs_flash/include \
  ${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH}/components/esp_partition/include

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

# EDITING NOTE:
#		Keep indentations below _with spaces_ (no tabs)!
#
ifeq (${DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH},)
  $(error Missing 'DEP_ESP_IDF_EMBUILD_ESP_IDF_PATH' from '.BUILD_ENV')
endif
ifeq (${DEP_ESP_IDF_ROOT},)
  $(error Missing 'DEP_ESP_IDF_ROOT' from '.BUILD_ENV')
endif
ifeq (${DEP_ESP_IDF_EMBUILD_ENV_PATH},)
  $(error Missing 'DEP_ESP_IDF_EMBUILD_ENV_PATH' from '.BUILD_ENV')
endif
#ifeq (${OUT_DIR},)
#  $(error Missing 'OUT_DIR' from '.BUILD_ENV')
#endif

# This happens if ESP-IDF does not pick up the definitions that we need 'esp-zigbee-lib'
ifeq ($(wildcard $(DEP_ESP_IDF_ROOT)/managed_components),)
  $(error No 'managed_components' pulled by Rust build)
endif

# Check all paths within the var, separately.
$(foreach p,$(_ESP_ZIGBEE_LIB_INCLUDE),$(if $(wildcard $(p)/*),,$(error No files within '$(p)')))

_TARGET:=${MCU}

_TMP:=./tmp

all:
	@false

# The targets that 'build.rs' normally builds (for debugging, in case make fails)
manual: ${_BIND_RS}
	@echo "\nManual build succeeded."

#---
# Generate Rust
#
# Generate the 'binding*.rs’. MANUAL CHANGES TO IT WILL BE LOST; modify this instead.
#
# Note:
#		- Cannot use '--opaque-type' too broadly, since the methods are defined within those (not the '_t' aliases)
#			<<
#				--opaque-type ".*_s" \
#			<<
#
# Note 2:
#		- '--no-copy' for config structs is merely cosmetic: having 'Copy' for said structs would not hurt
#		  as such. The author just rather has them "move only".
#
_BLOCKLIST_VARS:=$(strip EZB_ZCL_VALUE_NaN)
	# '.._VALUE_NaN' avoid warning on its naming (we don't need it, either)

# Typedefs to zero-size structs. Suppresses warnings.
#|_OPAQUE_TYPES:=$(strip \
#|	esp_zb_diagnostics_cluster_cfg_t \
#|	esp_zb_price_cluster_cfg_t \
#|	esp_zb_touchlink_commissioning_cfg_t \
#|)
_OPAQUE_TYPES:=""

# GNU Make note:
#		- '&:' syntax (GNU Make 4.3+) states that _both_ targets are created on the same run.
${_BIND_RS} ${_TMP}/wrap.c &: wrap.h $(addsuffix /*.h, $(_ESP_ZIGBEE_LIB_INCLUDE)) $(_SDKCONFIG_H) Makefile \
	| bindgen
	bindgen $< \
	    --allowlist-file wrap.h \
			\
	    --allowlist-function 'esp_zigbee_.+' \
			--allowlist-function 'ezb_af_(?:create|device|endpoint)_.+' \
			--allowlist-function 'ezb_app_(?:signal|xxx)_.+' \
			--allowlist-function 'ezb_aps_(?:secur|xxx)_.+' \
			--allowlist-function 'ezb_bdb_.+' \
	    --allowlist-function 'ezb_get_current_.+' \
			--allowlist-function 'ezb_nwk_.+' \
			--allowlist-function 'ezb_zcl_(?:basic_cluster|core|ota_upgrade)_.+' \
			--allowlist-function 'ezb_zha_create_(?:color_dimmable_light|color_dimmer_switch)' \
			\
	    --allowlist-type 'ezb_addr_mode_e' \
	    --allowlist-type 'ezb_af_(?:device|endpoint)_.+' \
	    --allowlist-type 'ezb_app_(?:signal|xxx)_.+' \
	    --allowlist-type 'ezb_bdb_.+' \
	    --allowlist-type 'ezb_nwk_.+' \
	    --allowlist-type 'ezb_zcl_(?:attr|attribute|basic|cluster|cmd|core|set|status)_.+' \
	    --allowlist-type 'ezb_zdo_(?:signal|xxx)_.+' \
	    --allowlist-type 'ezb_ext(?:panid|addr)_t' \
	    \
	    --allowlist-var 'ESP_ERR_.+' \
	    --allowlist-var 'EZB_ZCL_(?:CLUSTER|XXX)_.+' \
	    --allowlist-var 'ESP_ZIGBEE_VER' \
	    \
			$(addprefix --blocklist-var ,$(_BLOCKLIST_VARS)) \
			\
			$(addprefix --opaque-type ,$(_OPAQUE_TYPES)) \
			\
			--bitfield-enum 'ezb_bdb_comm_mode_e' \
	    --use-core \
	    --generate-inline-functions \
	    --default-enum-style rust \
	    --with-derive-custom "ezb_nwk_network_status_t=strum::FromRepr,strum::Display" \
	    --with-derive-custom "ezb_app_signal_type_e=strum::FromRepr,strum::Display" \
	    --with-derive-custom "ezb_zcl_attr_type_e=strum::FromRepr,strum::Display" \
	    --with-derive-custom "ezb_zcl_core_action_callback_id_e=strum::FromRepr,strum::Display" \
	    --with-derive-custom "ezb_zcl_status_e=strum::FromRepr,strum::Display" \
	    --with-derive-custom "ezb_zcl_cluster_id_e=strum::FromRepr,strum::Display" \
	    --with-derive-custom "ezb_addr_mode_e=strum::FromRepr,strum::Display" \
	    --no-copy "esp_zigbee(?:|_platform|_device)_config_s" \
			--wrap-static-fns \
	    --wrap-static-fns-path ${_TMP}/wrap.c \
	    --wrap-static-fns-suffix _ext \
		-- \
		-D __riscv -D __riscv_xlen=32 --sysroot ${_SYSROOT} \
		\
	  -I. -I${_TMP} \
	  $(addprefix -I, ${_ESP_ZIGBEE_LIB_INCLUDE}) \
		-I $(dir ${_SDKCONFIG_H}) \
	  $(addprefix -I, ${_NVS_COMPONENTS_INCLUDE}) \
		-I ${_SYSROOT}/riscv32-esp-elf/include \
		-I ${_ESP_IDF_PATH}/components/esp_common/include \
		-I ${_ESP_IDF_PATH}/components/hal/include \
		-I ${_ESP_IDF_PATH}/components/soc/${_TARGET}/include \
		-I ${_ESP_IDF_PATH}/components/soc/${_TARGET}/register \
		-I ${_ESP_IDF_PATH}/components/esp_driver_uart/include \
		-I ${_ESP_IDF_PATH}/components/esp_hw_support/include \
		-I ${_ESP_IDF_PATH}/components/freertos/FreeRTOS-Kernel/include \
		-I ${_ESP_IDF_PATH}/components/freertos/FreeRTOS-Kernel/portable/riscv/include/freertos \
		-I ${_ESP_IDF_PATH}/components/freertos/config/include \
		-I ${_ESP_IDF_PATH}/components/freertos/config/include/freertos \
		-I ${_ESP_IDF_PATH}/components/freertos/config/riscv/include \
		-I ${_ESP_IDF_PATH}/components/riscv/include \
		-I ${_ESP_IDF_PATH}/components/esp_rom/include \
		-I ${_ESP_IDF_PATH}/components/heap/include \
		-I ${_ESP_IDF_PATH}/components/esp_system/include \
		-I ${_ESP_IDF_PATH}/components/ieee802154/include \
		-I ${_ESP_IDF_PATH}/components/log/include \
		-I ${_ESP_IDF_PATH}/components/newlib/platform_include \
		\
		-Wno-int-to-void-pointer-cast -Wno-void-pointer-to-int-cast \
	  > $@

# Entries removed (keep for a while):
#	    --allowlist-function 'nvs_flash_init(?:|_partition)' \

# Hints:
#	    --raw-line '#![allow(non_camel_case_types)]' \
#	    --raw-line '#![allow(non_snake_case)]' \
#			--raw-line 'use strum::FromRepr;' \
#			--no-copy 'VL_(?:(Configuration)|(Platform))' \

${_TMP}/wrap.o: ${_TMP}/wrap.c
	PATH="$(DEP_ESP_IDF_EMBUILD_ENV_PATH)" \
	  riscv32-esp-elf-gcc -c $< -o $@ -O2 \
		-D __riscv -D __riscv_xlen=32 --sysroot ${_SYSROOT} \
	  -I. -I${_TMP} \
	  $(addprefix -I, ${_ESP_ZIGBEE_LIB_INCLUDE}) \
		-I $(dir ${_SDKCONFIG_H}) \
	  $(addprefix -I, ${_NVS_COMPONENTS_INCLUDE}) \
		-I ${_SYSROOT}/riscv32-esp-elf/include \
		-I ${_ESP_IDF_PATH}/components/esp_common/include \
		-I ${_ESP_IDF_PATH}/components/hal/include \
		-I ${_ESP_IDF_PATH}/components/soc/${_TARGET}/include \
		-I ${_ESP_IDF_PATH}/components/soc/${_TARGET}/register \
		-I ${_ESP_IDF_PATH}/components/esp_driver_uart/include \
		-I ${_ESP_IDF_PATH}/components/esp_hw_support/include \
		-I ${_ESP_IDF_PATH}/components/freertos/FreeRTOS-Kernel/include \
		-I ${_ESP_IDF_PATH}/components/freertos/FreeRTOS-Kernel/portable/riscv/include/freertos \
		-I ${_ESP_IDF_PATH}/components/freertos/config/include \
		-I ${_ESP_IDF_PATH}/components/freertos/config/include/freertos \
		-I ${_ESP_IDF_PATH}/components/freertos/config/riscv/include \
		-I ${_ESP_IDF_PATH}/components/riscv/include \
		-I ${_ESP_IDF_PATH}/components/esp_rom/include \
		-I ${_ESP_IDF_PATH}/components/heap/include \
		-I ${_ESP_IDF_PATH}/components/esp_system/include \
		-I ${_ESP_IDF_PATH}/components/ieee802154/include \
		-I ${_ESP_IDF_PATH}/components/log/include \
		-I ${_ESP_IDF_PATH}/components/newlib/platform_include \


${_TMP}/libezb_wrap.a: ${_TMP}/wrap.o
	PATH="$(DEP_ESP_IDF_EMBUILD_ENV_PATH)" \
	  riscv32-esp-elf-ar rcs $@ $<

# Test the above (for manual development, only)
#
#		'grep -q' for strings that should be in.
#		'! grep -q' for those that should be omitted.
#		- that the binding output is not outragously large (filtering failed)
#
SAFE_SIZE:=200000
	# normal is: <5000 lines, ~185k

manual_test: ${_BIND_RS}
	#
	grep -q ESP_ZIGBEE_VER $<
  #
	! grep -q '_NEWLIB_VERSION_H__' $<
	! grep -q 'EZB_ZCL_SHADE_CONFIG_MOTOR_STEP_SIZE_MIN_VALUE' $<
	@#
	@FILE_SIZE=$$(stat -c%s $<); \
	[ "$$FILE_SIZE" -lt $(SAFE_SIZE) ] || ( \
		echo >&2 "ERROR: '${<}' size $$FILE_SIZE > ${SAFE_SIZE}"; false; \
	)
	wc -l $<
	@echo ""
	@echo "Yay!"

# Check that required tools are installed
bindgen:
	@which bindgen >/dev/null || ( \
	  echo >&2 "ERROR: 'bindgen' CLI not detected. Please install via 'cargo install --locked bindgen-cli'."; false \
	)

#---
_clean:
	-rm ${_BIND_RS} ${_TMP}/wrap.[co] ${_TMP}/libezb_wrap.a

echo:
	@echo ${_SDKCONFIG}

# Remove any targets created if a build fails.
.DELETE_ON_ERROR:

.PHONY: all manual manual_test bindgen _clean echo
