#!/usr/bin/env just --justfile

#
# Optional tools for development
#
cloc:
	cloc --exclude-dir tmp,DEVS \
	  --not-match-f="^~" \
	  api apps config raw README.md Cargo.toml

	@# --not-match-f: leaves out files that start with a tilde ('~')

# Hint: add '--by-file' to see what caused the stats.

