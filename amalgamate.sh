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
(cd "${rust_dir}" && cargo build)
