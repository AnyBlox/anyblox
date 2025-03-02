#!/bin/bash

./target/release/bundler --metadata-path ./ignition-dataset-defs/CommonGovernment_45_rle.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle.wasm  --output-path ./dataset/CommonGovernment_45_rle.ignition
./target/release/bundler --metadata-path ./ignition-dataset-defs/CommonGovernment_45_rle_simd.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle_simd.wasm  --output-path ./dataset/CommonGovernment_45_rle_simd.ignition


for x in 100 1000 10000 100000 1000000 10000000; do
  for y in $(seq 1 9); do
    b=$((x * y))
    ./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 $b wasm -t 1 -d ./dataset/CommonGovernment.rle -q
    ./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 $b wasm -t 1 -d ./dataset/CommonGovernment.rle -q
  done;
done;

./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 100000000 wasm -t 1 -d ./dataset/CommonGovernment.rle -q
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 100000000 wasm -t 1 -d ./dataset/CommonGovernment.rle -q
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 141123827 wasm -t 1 -d ./dataset/CommonGovernment.rle -q
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 141123827 wasm -t 1 -d ./dataset/CommonGovernment.rle -q


