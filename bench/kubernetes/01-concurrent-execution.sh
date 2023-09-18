#!/bin/bash

NOMAD_URL="kubi.toramolampi.com"

for test_type in rust-spin-whatlang rust-spin-user-manager rust-spin-n-body rust-spin-aes rust-spin-prime-numbers rust-spin-fuzzysearch-http rust-spin-audio-sine-wave rust-wasmedge-whatlang rust-wasmedge-user-manager rust-wasmedge-n-body-socket rust-wasmedge-fuzzysearch-http-socket rust-wasmedge-whatlang-socket rust-wasmedge-prime-numbers-socket rust-wasmedge-n-body rust-wasmedge-aes rust-wasmedge-user-manager-socket rust-wasmedge-prime-numbers rust-wasmedge-audio-sine-wave
#for test_type in rust-spin-fuzzysearch-http rust-spin-user-manager rust-spin-audio-sine-wave rust-wasmedge-fuzzysearch-http-socket rust-wasmedge-user-manager rust-wasmedge-user-manager-socket rust-wasmedge-audio-sine-wave
do
  echo "Testing $test_type..."
  BASE_URL="http://$test_type.$NOMAD_URL"
  case $test_type in
     *"aes"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/?length=1000&iterations=100" > $test_type-concurrent-execution.csv;;
     *"float-operation"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL?n=10000000" > $test_type-concurrent-execution.csv;;
     *"linear-equations"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/?unknowns=128" > $test_type-concurrent-execution.csv;;
     *"matmul"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/?dimensions=50" > $test_type-concurrent-execution.csv;;
     *"fuzzysearch"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/?search=Hamlet" > $test_type-concurrent-execution.csv;;
     *"n-body"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/" > $test_type-concurrent-execution.csv;;
     *"prime-numbers"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/?n=100" > $test_type-concurrent-execution.csv;;
     *"whatlang"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/?text=The%20quick%20brown%20fox%20jumps%20over%20the%20lazy%20dog" > $test_type-concurrent-execution.csv;;
     *"audio-sine-wave"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/" > $test_type-concurrent-execution.csv;;
     *"user-manager"*)
       hey -c 5 -q 5 -z 1m -m GET -t 0 -disable-keepalive -o csv "$BASE_URL/?entries=1" > $test_type-concurrent-execution.csv;;
  esac
done
