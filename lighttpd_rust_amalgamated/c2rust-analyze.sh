#!/bin/bash

set -euo pipefail

# This is for convenience in running things, but it may miss small things,
# and can be replicated by copying the exact arguments and environment
# variables cargo normally passes. It should be run from the `c2rust` directory.

# Validate arguments
if [[ $# -ne 0 ]]; then
  echo "Usage: $0"
  exit 1
fi

# Get the directory of this script
MODULE_DIR="$(dirname "$0")"

# Find the sysroot directory of rustc
SYSROOT="$(rustc --print sysroot)"

# Find the necessary rlibs
extern() {
  local name=$1
  local rlib=$(find "$MODULE_DIR/target/debug/deps" -name "lib${name}*.rlib" -print -quit)
  echo >&2 "found rlib for $name: $rlib"
  echo --extern $name=$rlib
}

# Run the program with the necessary dependencies
cargo run --bin c2rust-analyze -- "$MODULE_DIR/src/main.rs" \
  --crate-name "$(basename "$MODULE_DIR")" \
  --edition 2021 \
  --crate-type rlib \
  --sysroot "$SYSROOT" \
  -L "dependency=$MODULE_DIR/target/debug/deps" \
  $(extern c2rust_bitfields) \
  $(extern libc) \
  -A warnings
