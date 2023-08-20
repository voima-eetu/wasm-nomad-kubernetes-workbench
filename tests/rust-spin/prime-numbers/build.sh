#!/bin/bash

spin build
cp target/wasm32-wasi/release/prime_numbers.wasm build/main.wasm
cd build
docker build -t artifacts.apps.mrezhi.net/wasm/spin/rust/prime-numbers:v1 .
sudo docker push artifacts.apps.mrezhi.net/wasm/spin/rust/prime-numbers:v1
