# AnyBlox Decoder specification v0.7

This is a draft specification of valid WebAssembly decoders for AnyBlox.

Due to the experimental research nature of this project this is more me noting down important caveats
about the API than a normative document.

## Preliminaries

A Decoder is a program in the `wasm` bytecode that conforms to this specification.
Whether the bytecode is produced by compilation from a different source language or otherwise is of no concern for the Runtime,
however the decoder API library is written in Rust and only Rust programs compiled with cargo to `wasm32-unknown-unknown` were tested.

Some spec violations are obvious and the Runtime will refuse to load a program, e.g. using `memory64` or an invalid
entry function signature, however in general the Runtime is NOT guaranteed to reject invalid programs.

## WebAssembly limitations

- The Decoder MUST NOT use $64$-bit memory. Only $32$-bit addressing is supported.
- WebAssembly proposals are not supported unless explicitly specified in this document.
In particular, but not limited to, the garbage collection and multithreading facilities of WebAssembly.
- The Decoder MAY use WebAssembly SIMD and the relaxed SIMD proposal.

## Memory

