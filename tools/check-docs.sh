#!/usr/bin/env bash
#
# tools/check-docs.sh
#
# Checks that command blocks in Markdown files match the 'Justfile' recipes they are derived from.
#
# Usage (from anywhere within the repo):
#   <<
#     $ tools/check-docs.sh
#   <<
#
# Mark a command block in a '.md' file like this (path relative to the repo root):
#   <<
#     <!-- just: config/Justfile build -->
#     ```
#     $ cargo build --release ...
#     ```
#   <<
#
# The block must contain the very commands that 'just --dry-run {recipe}' prints, in the same order.
# A leading '$ ' on a line is ignored, as are differences in whitespace.
#
# Markers with '{' in them (e.g. '<!-- just: {path/to/Justfile} {recipe} -->') are placeholders in
# instructions, and skipped.
#
# Exit code: 0 if all marked blocks match; 1 otherwise.
#
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
cd "$ROOT"

normalize() {
  sed -E 's/^\$ //; s/[[:space:]]+/ /g; s/^ //; s/ $//' | grep -v '^$' || true
}

fails=0
checked=0

while IFS= read -r md; do
  # Emit "LINE<TAB>JUSTFILE<TAB>RECIPE" for each marker
  while IFS=$'\t' read -r line justfile recipe; do
    checked=$((checked+1))
    where="$md:$line"

    if [[ ! -f "$justfile" ]]; then
      echo "❌ $where: no such Justfile: $justfile"; fails=$((fails+1)); continue
    fi

    # The fenced block following the marker
    doc=$(awk -v start="$line" '
      NR <= start { next }
      /^```/ { if (inb) exit; inb = 1; next }
      inb { print }
    ' "$md" | normalize)

    expected=$( (cd "$(dirname "$justfile")" && just --justfile "$(basename "$justfile")" --dry-run "$recipe" 2>&1) | normalize) || {
      echo "❌ $where: 'just --dry-run $recipe' failed:"; echo "$expected"; fails=$((fails+1)); continue
    }

    if [[ "$doc" != "$expected" ]]; then
      echo "❌ $where: differs from '$justfile' recipe '$recipe':"
      diff <(echo "$expected") <(echo "$doc") | sed 's/^/    /' || true
      echo "    ('<' = Justfile, '>' = doc)"
      fails=$((fails+1))
    fi
  done < <(grep -nE '<!-- just: [^ {]+ [^ {]+ -->' "$md" \
            | sed -E 's/^([0-9]+):.*<!-- just: ([^ ]+) ([^ ]+) -->.*/\1\t\2\t\3/')
done < <(git ls-files '*.md')

if [[ $fails -gt 0 ]]; then
  echo "$fails of $checked marked command block(s) out of sync."
  exit 1
fi
echo "✅ $checked marked command block(s) in sync."
