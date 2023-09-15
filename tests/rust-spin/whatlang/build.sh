#!/bin/bash

PROJECT=$(echo $(pwd) | awk -F/ '{print $NF}')
cargo build --target wasm32-wasi --release
cp target/wasm32-wasi/release/$PROJECT.wasm build/main.wasm
cd build
buildah build --platform=wasi/wasm --annotation "module.wasm.image/variant=compat-smart" --no-cache -t 10.223.6.99:5000/spin/rust/$PROJECT:v2 .
buildah push 10.223.6.99:5000/spin/rust/$PROJECT:v2
