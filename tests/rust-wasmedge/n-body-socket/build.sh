#!/bin/bash

PROJECT=$(echo $(pwd) | awk -F/ '{print $NF}')
cargo build --target wasm32-wasi --release
cp target/wasm32-wasi/release/$PROJECT.wasm build/main.wasm
cd build
docker build -t 10.223.6.99:5000/wasmedge/rust/$PROJECT:v1 .
sudo docker push 10.223.6.99:5000/wasmedge/rust/$PROJECT:v1