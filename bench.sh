#!/usr/bin/env bash

export GOMAXPROCS=1

test_cases=("fib" "matrix" "nqueens" "prime" "quicksort")
wasmtime_flags="--wasm-features all --allow-precompiled"
wasmedge_flags="--enable-all"

for test_case in "${test_cases[@]}"; do
    echo -e "Testing ${test_case}:\n"

    echo -e "Testing go native:\n"
    ./output/go/"${test_case}"

    echo -e "\nTesting wasmtime:\n"
    wasmtime run ${wasmtime_flags} output/wasmtime/"${test_case}"

    echo -e "\nTesting wasmedge:\n"
    wasmedge ${wasmedge_flags} output/wasmedge/"${test_case}"

    echo -e "\n${test_case} testing done.\n"
    echo -e "——————————————————————————————————————\n"
done
