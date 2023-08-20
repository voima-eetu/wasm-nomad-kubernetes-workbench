#!/bin/bash

spin build
cp target/wasm32-wasi/release/fuzzysearch_http.wasm build/main.wasm
cd build
docker build -t artifacts.apps.mrezhi.net/wasm/spin/rust/fuzzysearch-http:v1 .
sudo docker push artifacts.apps.mrezhi.net/wasm/spin/rust/fuzzysearch-http:v1
