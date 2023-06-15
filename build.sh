#!/usr/bin/env bash

# Create necessary directories
output_dir=output
targets=(tinygo go wasmtime wasmedge)
dirs=(fib matrix nqueens prime quicksort)

rm -rf ${output_dir}

for target in "${targets[@]}"; do
    mkdir -p "${output_dir}/${target}"
done

# Build TinyGo and Go binaries
files=(calc/fib.go calc/matrix.go calc/nqueens.go calc/prime.go calc/quicksort.go)

for i in "${!files[@]}"; do
    tinygo build -o "${output_dir}/tinygo/${dirs[i]}.wasm" -target=wasi -opt=2 "${files[i]}"
    go build -o "${output_dir}/go/${dirs[i]}" "${files[i]}"
done

# Compile with Wasmtime and WasmEdge
for dir in "${dirs[@]}"; do
    wasmtime compile --wasm-features all -o "${output_dir}/wasmtime/${dir}" "${output_dir}/tinygo/${dir}.wasm"
    wasmedgec --enable-all --optimize 3 "${output_dir}/tinygo/${dir}.wasm" "${output_dir}/wasmedge/${dir}"
done
