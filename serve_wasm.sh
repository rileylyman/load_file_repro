#!/usr/bin/env bash

set -e

cargo b --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/debug/load_file_repro.wasm .
python3 -m http.server 8000
