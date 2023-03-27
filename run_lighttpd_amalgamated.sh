#!/bin/bash

set -euo pipefail

# This is for convenience in running things, but it may miss small things,
# and can be replicated by copying the exact arguments and environment
# variables cargo normally passes. It should be run from the `c2rust` directory.

# Validate arguments
if [[ $# -ne 1 ]]; then
  echo "Usage: $0 MODULE_DIR"
  exit 1
fi

# Get the directory of this script
MODULE_DIR="$1"

# Find the sysroot directory of rustc
SYSROOT="$(rustc --print sysroot)"

# Find the necessary rlibs
C2RUST_BITFIELDS=$(find "$MODULE_DIR/target/debug/deps" -name "libc2rust_bitfields*.rlib" -print -quit)

# Print the found rlibs
echo "Found rlibs:"
echo "  c2rust_bitfields: $C2RUST_BITFIELDS"

# Run the program with the necessary dependencies
cargo run --bin c2rust-analyze -- "$MODULE_DIR/src/main.rs"\
  --edition 2021 \
  --crate-type rlib \
  -L "dependency=$MODULE_DIR/target/debug/deps" \
  -L "$SYSROOT/lib/rustlib/x86_64-unknown-linux-gnu/lib" \
  --extern c2rust_bitfields="$C2RUST_BITFIELDS"
