#!/usr/bin/env bash

set -euox pipefail

rust_dir=lighttpd_rust_amalgamated

cmake -Wno-dev
make clean
bear -- make lighttpd
make mod_{indexfile,dirlisting,staticfile}
./amalgamate.mjs | "${SHELL}" -euox pipefail
c2rust transpile --overwrite-existing --emit-build-files --binary lighttpd_amalgamated --output-dir ${rust_dir} amalgamated.compile_commands.json
cp {rust,${rust_dir}}/build.rs
mv ${rust_dir} ${rust_dir}.old
cargo new ${rust_dir}
(cd "${rust_dir}"
    cargo add libc
    cargo add c2rust-bitfields
    cp ../rust/build.rs .
    mv ../${rust_dir}.old/rust-toolchain.toml .
    mv ../${rust_dir}.old/src/lighttpd_amalgamated.rs src/main.rs
    sed -i 's/#\[macro_use\]//g' src/main.rs
    sed -i 's/extern crate [^;]*;//g' src/main.rs
    sed -i 's/use ::lighttpd_rust_amalgamated::\*;/use c2rust_bitfields::BitfieldStruct;/' src/main.rs
    cargo fmt
    cargo build
)
rm -rf ${rust_dir}.old
