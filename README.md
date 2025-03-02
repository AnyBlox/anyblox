# AnyBlox (Portable Decompression)

This is the main repo of the AnyBlox project, containing the runtime, dataset utilities, benchmark tools, and the C++ integration.

## Organization

The main runtime lives in the `anyblox` crate. It produces the library and the `anyblox-cli` binary.

In `anyblox-bench` there is a simple CLI app that can run a given workload and output a bunch of metrics. Check the CLI documentation for details.

In `anyblox-format` we define the serialization and deserialization of the metadata files for AnyBlox. This is a library only. Most other stuff depends on it.

In `anyblox-tools` there are three very important tools for working with AnyBlox and testing.

1. The `bundler` binary creates a full `.anyblox` bundle from a metadata file, a wasm decoder, and optional data (for self-contained datasets).
2. The `viewer` can open an existing bundle and read its metadata, verifying correctness.
3. The `anyblox2csv` can run AnyBlox on an existing bundle and output the results to a CSV file. This allows testing by comparing the output to the input dataset that
was compressed in the first place.

In `compress` a bunch of compression algorithms are defined.

In `dataset-utils` there are specific scripts that compress a specific dataset with some predefined compression. For example, `single_column_rle` can compress any
single-column integer dataset with RLE, while `taxpayer` is meant specifically for the Taxpayer PublicBI dataset.

In `decompress` all AnyBlox decompression algorithms are defined. They use the common `decoder-lib` utility library for wasm that defines some helpers, the expected return bundle type,
and a memory allocator for wasm.

The C++ and JNI integrations are in `anyblox-cpplib`. The `rust` directory contains the C API that is built as a static library.
In `cpp` the `libanyblox-cpp` library is defined, using the static Rust library as a dependency. In `jni` the magic of `jni` happens so that
the interface can be loaded from JVM-based engines like Spark.

In `vendored` the forked version of Rust's `arrow` and `parquet` live.

In `anyblox-parquet` the entirely experimental prototype of a Parquet writer for AnyBlox is used.

## Building

### Prerequisites

You need Rust to build the Rust parts, and the `mold` linker (on Debian-based OS use `apt install mold`).
Having [`just`](https://github.com/casey/just?tab=readme-ov-file#installation) helps with automation.

For C++ the prerequisite is an installation of the C++ Arrow library so that CMake can detect it with `find_package`.
The recommended way is to build `arrow/cpp` from source separately, follow [their docs](https://arrow.apache.org/docs/developers/cpp/index.html).

*To install `libanyblox`* systemwide go to `anyblox-cpplib` and run `just install`.

For `anyblox-spark` you need sbt, maven, and a JDK. The recommended way is to use [sdkman](https://sdkman.io/) to manage these dependencies.
The recommended SDK is `11.0.25-tem`. The required Scala version is `2.12`.

### Commands

Run `just` in the root directory to build the library, cli, all wasm decoders, and benchmarks.

To build a specific tool use cargo with the package specifier, e.g. `cargo build -p anyblox-tools --bin bundler` builds the bundler.

## Usage

The datasets we use are mostly from [the Public BI benchmark](https://github.com/cwida/public_bi_benchmark).

- `Bimbo` contains integer columns that are great for RLE compression.
- `Taxpayer` contains string columns for FSST compression.

We also use TCP-H `lineitem` for compressing with RLE+Bitpacking

To make a bundle first use `dataset-utils` to extract data from a CSV dataset into a compressed binary format.
Then use the `bundler` tool to create a bundle. Example TOML configurations are in `anyblox-tools/example-bundle`, e.g. there is
`Bimbo-rle-ext.toml` that can produce an extension working for any of the Bimbo columns compressed with RLE.

The best place for datasets is to create a directory `dataset` at the top level (it is gitignored).

To run the bundle you can use `anyblox-cli` or `anyblox-bench`. Type `help` in the CLI to get the list of commands.
