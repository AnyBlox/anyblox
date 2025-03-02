#!/bin/bash

./target/release/bundler --metadata-path ./ignition-dataset-defs/CommonGovernment_45_rle.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle.wasm  --output-path ./dataset/CommonGovernment_45_rle.ignition
./target/release/bundler --metadata-path ./ignition-dataset-defs/CommonGovernment_45_rle_simd.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle_simd.wasm  --output-path ./dataset/CommonGovernment_45_rle_simd.ignition
./target/release/bundler --metadata-path ./ignition-dataset-defs/CommonGovernment_45_rle_simd_nativeavx2.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle_simd.wasm  --output-path ./dataset/CommonGovernment_45_rle_simd_nativeavx2.ignition
./target/release/bundler --metadata-path ./ignition-dataset-defs/CommonGovernment_45_rle_simd_nativesse2.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle_simd.wasm  --output-path ./dataset/CommonGovernment_45_rle_simd_nativesse2.ignition
./target/release/bundler --metadata-path ./ignition-dataset-defs/CommonGovernment_45_rle_simd_stateless.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle_simd_stateless.wasm  --output-path ./dataset/CommonGovernment_45_rle_simd_stateless.ignition


./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 9000000 wasm -t 1 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 200000 wasm -t 1 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd_stateless.ignition 0 141123827 200000 wasm -t 1 -d ./dataset/CommonGovernment.rle

./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 9000000 native -t 1 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 200000 native -t 1 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd_nativesse2.ignition 0 141123827 200000 native -t 1 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd_nativeavx2.ignition 0 141123827 200000 native -t 1 -d ./dataset/CommonGovernment.rle

./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 9000000 wasm -t 32 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 200000 wasm -t 32 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd_stateless.ignition 0 141123827 200000 wasm -t 32 -d ./dataset/CommonGovernment.rle

./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 9000000 native -t 32 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 200000 native -t 32 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd_nativesse2.ignition 0 141123827 200000 native -t 32 -d ./dataset/CommonGovernment.rle
./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd_nativeavx2.ignition 0 141123827 200000 native -t 32 -d ./dataset/CommonGovernment.rle


for t in $(seq 1 32); do
  ./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 9000000 wasm -t $t -d ./dataset/CommonGovernment.rle
  ./target/release/ignition-bench ./dataset/CommonGovernment_45_rle.ignition 0 141123827 9000000 native -t $t -d ./dataset/CommonGovernment.rle
  ./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 200000 wasm -t $t -d ./dataset/CommonGovernment.rle
  ./target/release/ignition-bench ./dataset/CommonGovernment_45_rle_simd.ignition 0 141123827 200000 native -t $t -d ./dataset/CommonGovernment.rle
done
