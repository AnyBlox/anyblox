#!/bin/bash

./target/release/bundler --metadata-path ./ignition-dataset-defs/taxpayer_large.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/taxpayer_fsst.wasm  --output-path ./dataset/taxpayer_large.ignition --data-path ~/hdd/data/PublicBiBenchmark/Taxpayer_large.fsst

./target/release/ignition-bench ./dataset/taxpayer_fsst.ignition 0 91532730 30000 wasm -t 1 -q --no-validate-utf8
./target/release/ignition-bench ./dataset/taxpayer_fsst.ignition 0 91532730 30000 native -t 1 -q --no-validate-utf8

./target/release/ignition-bench ./dataset/taxpayer_fsst.ignition 0 91532730 30000 wasm -t 32 -q --no-validate-utf8
./target/release/ignition-bench ./dataset/taxpayer_fsst.ignition 0 91532730 30000 native -t 32 -q --no-validate-utf8

for t in $(seq 1 32); do
  ./target/release/ignition-bench ./dataset/taxpayer_fsst.ignition 0 91532730 30000 wasm -t $t -q --no-validate-utf8
  ./target/release/ignition-bench ./dataset/taxpayer_fsst.ignition 0 91532730 30000 native -t $t -q --no-validate-utf8
done

