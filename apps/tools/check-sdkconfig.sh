#!/usr/bin/env bash
#
# Check 'sdkconfig.defaults' keys against the ESP-IDF version actually used by the latest build of THIS project.
#
# For each 'CONFIG_X' in the defaults file (active lines and commented-out ones):
#   - name:   OK | RENAMED->CONFIG_Y | UNKNOWN      (Kconfig of ESP-IDF + managed components; 'sdkconfig.rename*')
#   - value:  APPLIED | NOT APPLIED (got: ...)      (active lines only; compared with the generated 'sdkconfig')
#
# Exit code: 0 if all active keys are known and applied; 1 otherwise; 2 on setup errors.
#
# Usage:
#   tools/check-sdkconfig.sh [--out-dir] [path/to/sdkconfig.defaults] [target_dir]
#
#   --out-dir   only print the build's 'out' directory (used by the Justfile '*-path' recipes)
#
# Requires: jq
#
set -euo pipefail

ONLY_OUT=false
if [[ "${1:-}" == --out-dir ]]; then ONLY_OUT=true; shift; fi

DEFAULTS=$(realpath "${1:-sdkconfig.defaults}")
TARGET_DIR=${2:-$(cargo metadata --format-version 1 --no-deps | jq -r '.target_directory')}

# Find the newest 'esp-idf-sys' build whose config defaults include OUR defaults file. The target directory may be
# shared with other projects, so "newest overall" is not good enough.
#
OUT=""
while read -r _ pd; do
    if jq -e --arg d "$DEFAULTS" '.config_defaults | split(";") | index($d)' "$pd" >/dev/null; then
        OUT=$(dirname "$(dirname "$pd")")    # .../out/build/project_description.json -> .../out
        break
    fi
done < <(find "$TARGET_DIR" -path '*esp-idf-sys*/out/build/project_description.json' -printf '%T@ %p\n' 2>/dev/null | sort -rn)

if [[ -z "$OUT" ]]; then
    echo >&2 "Error: no 'esp-idf-sys' build using '$DEFAULTS' found under '$TARGET_DIR'. Build first."
    exit 2
fi

if $ONLY_OUT; then echo "$OUT"; exit 0; fi

PD="$OUT/build/project_description.json"
SDKCONFIG=$(jq -r '.config_file' "$PD")
IDF_PATH_=$(jq -r '.idf_path' "$PD")

echo "defaults:  $DEFAULTS"
echo "sdkconfig: $SDKCONFIG"
echo "ESP-IDF:   $IDF_PATH_ ($(sed -n 3p "$SDKCONFIG" | sed -E 's/.*\(ESP-IDF\) ([^ ]+).*/\1/'))"
echo

# Known symbol names: 'config X', 'menuconfig X', 'choice X' in all Kconfig files of ESP-IDF and the components of
# this build (incl. managed components, e.g. 'esp-zigbee-lib').
#
SYMS=$(mktemp); RENAMES=$(mktemp)
trap 'rm -f "$SYMS" "$RENAMES"' EXIT

{
    find "$IDF_PATH_/components" -type f -name 'Kconfig*'
    jq -r '.build_component_info[]?.dir // empty' "$PD" | while read -r d; do
        [[ -d "$d" ]] && find "$d" -maxdepth 2 -type f -name 'Kconfig*'
    done
} | sort -u | xargs -r grep -hsE '^\s*(menuconfig|config|choice)\s+[A-Z0-9_]+' \
  | sed -E 's/^\s*(menuconfig|config|choice)\s+([A-Z0-9_]+).*/\2/' | sort -u > "$SYMS"

find "$IDF_PATH_/components" -name 'sdkconfig.rename*' -print0 | xargs -0 -r cat \
  | grep -E '^CONFIG_' | awk '{print $1, $2}' > "$RENAMES" || true

fail=0
printf '%-1s %-48s %-40s %s\n' "" "KEY" "NAME" "VALUE"

grep -nE '^\s*#?\s*CONFIG_[A-Z0-9_]+' "$DEFAULTS" | while IFS= read -r line; do
    lineno=${line%%:*}
    text=${line#*:}
    key=$(sed -E 's/^\s*#?\s*CONFIG_([A-Z0-9_]+).*/\1/' <<<"$text")
    if [[ "$text" =~ ^[[:space:]]*# ]]; then active=c; else active=A; fi

    if grep -qx "$key" "$SYMS"; then
        name=OK
    elif new=$(awk -v k="CONFIG_$key" '$1==k {print $2; exit}' "$RENAMES") && [[ -n "$new" ]]; then
        name="RENAMED->$new"
    else
        name=UNKNOWN
    fi

    value=""
    if [[ $active == A ]]; then
        want=${text#*=}; want=${want%%#*}; want=$(sed -E 's/\s+$//' <<<"$want")
        if [[ "$want" == n ]]; then
            got=$(grep -E "^CONFIG_$key=" "$SDKCONFIG" | head -1 | cut -d= -f2- || true)
            if grep -qE "^# CONFIG_$key is not set$" "$SDKCONFIG"; then
                value=APPLIED
            elif [[ -z "$got" ]]; then
                # Symbol not in this build at all (unmet 'depends on', or its component is not built).
                value="ABSENT (effectively n)"
            fi
        else
            got=$(grep -E "^CONFIG_$key=" "$SDKCONFIG" | head -1 | cut -d= -f2- || true)
            [[ "$got" == "$want" ]] && value=APPLIED
        fi
        if [[ -z "$value" ]]; then
            grep -qE "^# CONFIG_$key is not set$" "$SDKCONFIG" && got="(not set)"
            value="NOT APPLIED (want: $want, got: ${got:-<absent>})"
        fi
        if [[ $name != OK || ( $value != APPLIED && $value != ABSENT* ) ]]; then echo FAIL >> "$SYMS.fail"; fi
    fi

    printf '%-1s %-48s %-40s %s\n' "$active" "$key (l.$lineno)" "$name" "$value"
done

echo
echo "A = active line, c = commented out"

if [[ -e "$SYMS.fail" ]]; then
    rm -f "$SYMS.fail"
    exit 1
fi
