#!/bin/bash

spin build
cp target/wasm32-wasi/release/user_manager.wasm build/main.wasm
cd build
docker build -t artifacts.apps.mrezhi.net/wasm/spin/rust/user-manager:v1 .
sudo docker push artifacts.apps.mrezhi.net/wasm/spin/rust/user-manager:v1
