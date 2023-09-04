#!/bin/bash

NOMAD_URL="nomadi.toramolampi.com"

for test_type in rust-spin-whatlang rust-spin-user-manager rust-spin-n-body rust-spin-aes rust-spin-prime-numbers rust-spin-fuzzysearch-http rust-spin-audio-sine-wave rust-wasmedge-whatlang rust-wasmedge-user-manager rust-wasmedge-n-body-socket rust-wasmedge-fuzzysearch-http-socket rust-wasmedge-whatlang-socket rust-wasmedge-prime-numbers-socket rust-wasmedge-n-body rust-wasmedge-aes rust-wasmedge-user-manager-socket rust-wasmedge-prime-numbers rust-wasmedge-audio-sine-wave
do
  echo "Testing $test_type..."
  BASE_URL="http://$test_type.$NOMAD_URL"
  case $test_type in
    *"aes"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/?length=1000&iterations=100" > $test_type-serial.csv;;
    *"float-operation"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL?n=10000000" > $test_type-serial.csv;;
    *"linear-equations"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/?unknowns=128" > $test_type-serial.csv;;
    *"matmul"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/?dimensions=50" > $test_type-serial.csv;;
    *"fuzzysearch"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/?search=Hamlet" > $test_type-serial.csv;;
    *"n-body"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/" > $test_type-serial.csv;;
    *"prime-numbers"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/?n=100" > $test_type-serial.csv;;
    *"whatlang"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/?text=The%20quick%20brown%20fox%20jumps%20over%20the%20lazy%20dog" > $test_type-serial.csv;;
    *"audio-sine-wave"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/" > $test_type-serial.csv;;
    *"user-manager"*)
      hey -disable-keepalive -z 5m -c 1 -t 0 -o csv -m GET "$BASE_URL/?entries=1" > $test_type-serial.csv;;
  esac
done
