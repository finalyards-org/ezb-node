#
# Makefile
#
# Launched by 'build.rs' but also usable stand-alone.
#
# Handles:
#	- compiling the 'esp-zigbee-sdk' C code
#	- creating Rust interfaces for it
#
# Requires:
#	- bindgen
#	- (patch)	# not needed
#	- clang
#
# Note: 'bindgen' docs show using that as a Rust library. We use 'bindgen' as a CLI.
#
# Note: 'clang' only recognizes the 'riscv32-unknown-elf' target, not the chip specific ones seen e.g. in
#		'../.cargo/config.toml'. This seems normal - and works, so..
#
# Note: Unconditional (manual) make with 'make -B' MAY BE CURRENTLY BROKEN. Just run 'make _clean', instead. #tbd.
#
#R ESP_ZIGBEE_SDK=.sub/esp-zigbee-sdk
ESP_ZIGBEE_LIB=../.sub/esp-zigbee-sdk/components/esp-zigbee-lib

_INCLUDE:=${ESP_ZIGBEE_LIB}/include
_TMP:=./tmp

_TARGET:=esp32c6

#R_OBJ_PATH:=$(_TMP)/$(TARGET)

_OTHER_INCS:=fake/*.h

all:
	@false

# The targets that 'build.rs' normally builds (for debugging, in case make fails)
manual: ${_TMP}/bindings.rs
	@echo "\nManual build succeeded."

#R#---
#R# Patch sources from './VL53L{58}CX_ULD_API/**' to 'tmp/c_src/'
#R#
#R# 	- places e.g. headers and C files in the same folder, for convenience
#R#	- removes a nasty! reference to a field in a structure that's supposed to be customer-made and opaque
#R#
#R$(_C_SRC)/%.h: $(VL53L$(X)CX_ULD_API)/inc/%.h \
#R	| tmp_c_exists
#R	@cp $< $@

#---
# Generate Rust
#
# Generate ’tmp/bindings.rs’. MANUAL CHANGES TO IT WILL BE LOST; modify this instead.
#
# About 'bindgen' (v.0.69.4):		<-- tbd. might have changed?  Now in 0.72.1
#	- prints errors to stdout, which is .... a bit unkind.
#	- does not process lines like:
#		<<
#			define SOME_ANY_THING ((uint8_t) 16U)
#		<<
#		As a countermeasure, we declare 'consts' in 'master.h'.
#
${_TMP}/bindings.rs: master.h ${_INCLUDE}/*.h $(_OTHER_INCS) Makefile \
	| bindgen
	bindgen $< \
	  -- -I. -Ifake -I${_INCLUDE} \
		-I ${IDF_PATH}/components/esp_common/include \
		-I ${IDF_PATH}/components/esp_system/include \
		-I ${IDF_PATH}/components/esp_hw_support/include \
		-I ${IDF_PATH}/components/hal/include \
		-I ${IDF_PATH}/components/esp_rom/include \
		-I ${IDF_PATH}/components/esp_rom/include \
		-I ${IDF_PATH}/components/esp_rom/include \
		-I ${IDF_PATH}/components/soc/${_TARGET}/include \
		-I ${IDF_PATH}/components/soc/${_TARGET}/include/soc \
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

#?clang:
#?	@which clang >/dev/null || ( \
#?	  echo >&2 "ERROR: 'clang' CLI not detected. Please install via 'sudo apt install llvm-dev libclang-dev clang'."; false \
#?	)

#patch:
#	@which $@ >/dev/null || ( \
#	  echo >&2 "ERROR: '$@' is not detected. Please install via 'sudo apt install $@'."; false \
#	)

#---
_clean:
	-rm ${_TMP}/bindings.rs

echo:
	@echo AA

# Remove any targets created if a build fails.
.DELETE_ON_ERROR:

.PHONY: all bindgen clang _clean echo
