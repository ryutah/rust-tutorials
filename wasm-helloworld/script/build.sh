#!/bin/sh

cd $(dirname $0)/..

set -eux

cargo +nightly build --target wasm32-unknown-unknown --release
wasm-gc target/wasm32-unknown-unknown/release/wasmhelloworld.wasm -o hello_world.gc.wasm
