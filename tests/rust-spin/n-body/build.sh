#!/bin/bash

spin build
cp target/wasm32-wasi/release/n_body.wasm build/main.wasm
cd build
docker build -t artifacts.apps.mrezhi.net/wasm/spin/rust/n-body:v1 .
sudo docker push artifacts.apps.mrezhi.net/wasm/spin/rust/n-body:v1
