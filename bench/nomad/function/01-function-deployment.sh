#!/bin/bash

ITERATIONS=100
NOMAD_URL="nomadi.toramolampi.com"
QUERY="/"

NOMAD_ADDR=http://10.223.6.50:4646 nomad monitor -log-level=trace -json > nomad0.log &
log1_pid=$!
NOMAD_ADDR=http://10.223.6.51:4646 nomad monitor -log-level=trace -json > nomad1.log &
log2_pid=$!
NOMAD_ADDR=http://10.223.6.52:4646 nomad monitor -log-level=trace -json > nomad2.log &
log3_pid=$!

export NOMAD_ADDR=http://10.223.6.50:4646

for test_type in rust-wasmedge-fuzzysearch-http rust-spin-whatlang rust-spin-user-manager rust-spin-n-body rust-spin-aes rust-spin-prime-numbers rust-spin-fuzzysearch-http rust-spin-audio-sine-wave rust-wasmedge-whatlang rust-wasmedge-user-manager rust-wasmedge-n-body-socket rust-wasmedge-fuzzysearch-http-socket rust-wasmedge-whatlang-socket rust-wasmedge-prime-numbers-socket rust-wasmedge-n-body rust-wasmedge-aes rust-wasmedge-user-manager-socket rust-wasmedge-prime-numbers rust-wasmedge-audio-sine-wave
do
  echo "Testing $test_type..."
  BASE_URL="http://$test_type.$NOMAD_URL"
  case $test_type in
     *"aes"*)
       QUERY="/?length=1000&iterations=100";;
     *"fuzzysearch"*)
       QUERY="/?search=Hamlet";;
     *"n-body"*)
       QUERY="/";;
     *"prime-numbers"*)
       QUERY="/?n=100";;
     *"whatlang"*)
       QUERY="/?text=The%20quick%20brown%20fox%20jumps%20over%20the%20lazy%20dog";;
     *"user-manager"*)
       QUERY="/?entries=1";;
     *"audio-sine-wave"*)
       QUERY="/";;
  esac
     for ((i=1; i<=$ITERATIONS; i++)); do
             echo "Run: $i for $test_type"
             echo -e "\tRestarting $test_type"
             nomad job restart -all-tasks -reschedule $test_type
             echo -e "\tSleeping for 5 seconds..."
             sleep 5
             echo -e "\tInvoking function $test_type"
             hey -disable-keepalive -o csv -n 1 -c 1 -t 0 -m GET "$BASE_URL$QUERY" >> $test_type-function-deployment.csv
    done
done
kill $log1_pid $log2_pid $log3_pid
