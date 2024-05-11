![WASI Update](logo.png)
========================

A simple tool to convert a WebAssembly module (`wasm32-wasi`, `wasm32-freestanding`) to a WASI component (component model, WASI-preview2).

# Installation

[Precompiled binaries](https://github.com/jedisct1/wasi-update/releases) are available for:

- Linux/x86_64 (.tar.gz and .deb)
- Linux/aarch64
- MacOS/aarch64
- Windows/x86_64
- Windows/aarch64
- WebAssembly (WASI)


# Usage

Give the tool the WebAssembly module to process, and output file name, and that's it!

Example:

```sh
wasi-update -i module.wasm -o component.wasm
```

# Compilation from source

Or if you really want to compile from source, install Rust and type:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasi-update
```
