#!/usr/bin/env bash

set -euox pipefail

cmake -Wno-dev
make clean
bear -- make lighttpd
./amalgamate.mjs | "${SHELL}" -euox pipefail
