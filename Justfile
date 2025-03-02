[private]
default: build-all

init: gen-vmlinux
  cargo libbpf build
  cargo libbpf gen

build-all: build-anyblox build-bench

build-anyblox: build-wasm-libs
  cargo build -p anyblox
  cargo build -p anyblox --release

build-bench:
  cargo build -p anyblox-bench
  cargo build -p anyblox-bench --release

build-wasm-libs $RUSTFLAGS="-C target-feature=+simd128": \
  (build-wasm-lib "clickbench-vortex") \
  (build-wasm-lib "empty-decoder") \
  (build-wasm-lib "fsst") \
  (build-wasm-lib "taxpayer-fsst") \
  (build-wasm-lib "taxpayer-fsst-stateless") \
  (build-wasm-lib "taxpayer_libfsst") \
  (build-wasm-lib "rle") \
  (build-wasm-lib "rle-linestatus") \
  (build-wasm-lib "rle-linestatus-paged") \
  (build-wasm-lib "rle-simd") \
  (build-wasm-lib "rle-simd-stateless") \
  (build-wasm-lib "tpch-vortex") \
  (build-wasm-lib "cern-root")

build-wasm-lib name $RUSTFLAGS="-C target-feature=+simd128":
  cargo build --target wasm32-unknown-unknown --manifest-path ./decompress/Cargo.toml -p {{name}} --features log
  cargo build --target wasm32-unknown-unknown --manifest-path ./decompress/Cargo.toml -p {{name}} --release

build-ebpf: (ebpf-make "packing") (ebpf-make "rle")
  cargo libbpf gen

build-tools:
  cargo build -p anyblox-tools
  cargo build -p anyblox-tools --release
  cargo build -p dataset-utils
  cargo build -p dataset-utils --release

ebpf-make name:
  llvm-mc -triple bpf -filetype=obj -o ./target/bpf/{{name}}.bpf.o ./ebpf-decompress/src/bpf/{{name}}.bpf.s

ebpf-emit name:
  clang -D__TARGET_ARCH_x86 -target bpf -g -O2 -S ./ebpf-decompress/src/bpf/{{name}}.bpf.c -o ./ebpf-decompress/src/bpf/{{name}}.bpf.s

gen-vmlinux:
  bpftool btf dump file /sys/kernel/btf/vmlinux format c > ebpf-decompress/src/bpf/vmlinux.h

bundle-all: build-wasm-libs build-tools
  ./target/release/bundler --metadata-path ./anyblox-dataset-defs/Bimbo-rle-ext.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle.wasm --output-path ./dataset/Bimbo-rle-ext.anyblox
  ./target/release/bundler --metadata-path ./anyblox-dataset-defs/Bimbo-rle-simd-ext.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle_simd.wasm --output-path ./dataset/Bimbo-rle-simd-ext.anyblox
  ./target/release/bundler --metadata-path ./anyblox-dataset-defs/taxpayer.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/taxpayer_fsst.wasm --output-path ./dataset/taxpayer.anyblox --data-path ./dataset/taxpayer.bin
  ./target/release/bundler --metadata-path ./anyblox-dataset-defs/taxpayer-libfsst.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/taxpayer_libfsst.wasm --output-path ./dataset/taxpayer-libfsst.anyblox --data-path ./dataset/taxpayer-libfsst.bin
  ./target/release/bundler --metadata-path ./anyblox-dataset-defs/tpch-linestatus-rlebitpack.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/rle_linestatus.wasm --output-path ./dataset/tpch-linestatus-rlebitpack.anyblox --data-path ./dataset/tpch-linestatus-rlebitpack.bin
  ./target/release/bundler --metadata-path ./anyblox-dataset-defs/tpch-vortex.toml --wasm-path ./decompress/target/wasm32-unknown-unknown/release/tpch_vortex.wasm --output-path ./dataset/tpch-vortex.anyblox --data-path ./dataset/tpch-vortex.bin

# Test scripts

test: test-rle test-fsst test-rle-lineitem test-tpch-vortex

test-rle: bundle-all
  ./target/release/anyblox2csv -i ./dataset/Bimbo-rle-ext.anyblox -d ./dataset/1_Agencia_ID.rle -o /tmp/1_Agencia_ID_rle_anyblox.csv
  ./target/release/anyblox2csv -i ./dataset/Bimbo-rle-ext.anyblox -d ./dataset/1_Agencia_ID.rle -o /tmp/1_Agencia_ID_rle_native.csv --native
  ./target/release/anyblox2csv -i ./dataset/Bimbo-rle-simd-ext.anyblox -d ./dataset/1_Agencia_ID.rle -o /tmp/1_Agencia_ID_rle_simd_anyblox.csv
  ./target/release/anyblox2csv -i ./dataset/Bimbo-rle-simd-ext.anyblox -d ./dataset/1_Agencia_ID.rle -o /tmp/1_Agencia_ID_rle_simd_native.csv --native
  diff /tmp/1_Agencia_ID_rle_anyblox.csv /tmp/1_Agencia_ID_rle_native.csv -a -q
  diff /tmp/1_Agencia_ID_rle_anyblox.csv /tmp/1_Agencia_ID_rle_simd_anyblox.csv -a -q
  diff /tmp/1_Agencia_ID_rle_anyblox.csv /tmp/1_Agencia_ID_rle_simd_native.csv -a -q
  diff /tmp/1_Agencia_ID_rle_anyblox.csv ~/PublicBiBenchmark/1_Agencia_ID.csv -a -q 

test-fsst: bundle-all
  ./target/release/anyblox2csv -i ./dataset/taxpayer.anyblox -o /tmp/Taxpayer_1_anyblox.csv
  ./target/release/anyblox2csv -i ./dataset/taxpayer.anyblox -o /tmp/Taxpayer_1_anyblox_native.csv --native
  ./target/release/anyblox2csv -i ./dataset/taxpayer-libfsst.anyblox -o /tmp/Taxpayer_1_libfsst_anyblox.csv
  ./target/release/anyblox2csv -i ./dataset/taxpayer-libfsst.anyblox -o /tmp/Taxpayer_1_libfsst_anyblox_native.csv --native
  diff /tmp/Taxpayer_1_anyblox.csv /tmp/Taxpayer_1_anyblox_native.csv -a -q
  diff /tmp/Taxpayer_1_anyblox.csv ./dataset/taxpayer.csv -a -q 
  diff /tmp/Taxpayer_1_libfsst_anyblox.csv /tmp/Taxpayer_1_libfsst_anyblox_native.csv -a -q
  diff /tmp/Taxpayer_1_libfsst_anyblox.csv ./dataset/taxpayer.csv -a -q 

test-rle-lineitem: bundle-all
  ./target/release/anyblox2csv -i ./dataset/tpch-linestatus-rlebitpack.anyblox -o /tmp/tpch-linestatus-rlebitpack_anyblox.csv
  ./target/release/anyblox2csv -i ./dataset/tpch-linestatus-rlebitpack.anyblox -o /tmp/tpch-linestatus-rlebitpack_native.csv --native
  diff /tmp/tpch-linestatus-rlebitpack_native.csv ./dataset/tpch-linestatus-rlebitpack.csv -a -q
  diff /tmp/tpch-linestatus-rlebitpack_native.csv /tmp/tpch-linestatus-rlebitpack_anyblox.csv -a -q

test-tpch-vortex: bundle-all
  ./target/release/anyblox2csv -i ./dataset/tpch-vortex.anyblox -o /tmp/tpch-vortex_anyblox.csv --byte-is-char --tpch-decimals
  ./target/release/anyblox2csv -i ./dataset/tpch-vortex.anyblox -o /tmp/tpch-vortex_anyblox_native.csv --byte-is-char --tpch-decimals --native
  diff /tmp/tpch-vortex_anyblox.csv /tmp/tpch-vortex_anyblox_native.csv -a -q
# diff /tmp/tpch-vortex_anyblox_native.csv ./dataset/lineitem.tbl -a -q