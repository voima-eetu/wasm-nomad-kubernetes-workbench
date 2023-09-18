#!/bin/bash

ITERATIONS=100
NAMESPACE=default
NOMAD_URL="kubi.toramolampi.com"

export KUBECONFIG=/home/voima/k3s.yaml

for test_type in rust-wasmedge-fuzzysearch-http rust-spin-whatlang rust-spin-user-manager rust-spin-n-body rust-spin-aes rust-spin-prime-numbers rust-spin-fuzzysearch-http rust-wasmedge-whatlang rust-wasmedge-user-manager rust-wasmedge-n-body-socket rust-wasmedge-fuzzysearch-http-socket rust-wasmedge-whatlang-socket rust-wasmedge-prime-numbers-socket rust-wasmedge-n-body rust-wasmedge-aes
#for test_type in rust-wasmedge-user-manager-socket rust-wasmedge-prime-numbers rust-wasmedge-audio-sine-wave rust-spin-audio-sine-wave
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
      pod_name=$(kubectl get pod -n $NAMESPACE | grep "$test_type-[a-z0-9]\{7,10\}" | awk '{print $1}')
      echo -e "\tDeleting $pod_name"
      line=$(kubectl delete pod -n $NAMESPACE $pod_name)
      echo -e "\t$line"
      echo -e "\tSleeping for 5 seconds..."
      sleep 5
      echo -e "\tInvoking function $test_type"
      hey -disable-keepalive -o csv -n 1 -c 1 -t 0 -m GET "$BASE_URL$QUERY" >> $test_type-function-deployment.csv
  done
done
