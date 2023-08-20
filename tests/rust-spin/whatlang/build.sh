#!/bin/bash

spin build
cp target/wasm32-wasi/release/whatlang.wasm build/main.wasm
cd build
docker build -t artifacts.apps.mrezhi.net/wasm/spin/rust/whatlang:v1 .
sudo docker push artifacts.apps.mrezhi.net/wasm/spin/rust/whatlang:v1
