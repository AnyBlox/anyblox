#!/bin/bash

./target/release/bundler --metadata-path ./ignition-dataset-defs/taxpayer_large.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/taxpayer_fsst.wasm  --output-path ./dataset/taxpayer_large.ignition --data-path ~/hdd/data/PublicBiBenchmark/Taxpayer_large.fsst

for x in 100 1000 10000 100000 1000000 10000000; do
  for y in $(seq 1 9); do
    b=$((x * y))
    ./target/release/ignition-bench ./dataset/taxpayer_fsst.ignition 0 91532730 $b wasm -t 1 -q --no-validate-utf8
  done;
done;

./target/release/ignition-bench ./dataset/taxpayer_fsst.ignition 0 91532730 91532730 wasm -t 1 -q --no-validate-utf8
