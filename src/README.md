![WASI Update](logo.png)
========================

A simple tool to convert a WebAssembly module (`wasm32-wasi`, `wasm32-freestanding`) to a WASI component (component model, WASI-preview2).

# Installation

Install Rust, and type:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasi-update
```

or clone the repository and type:

```sh
rustup target add wasm32-unknown-unknown
make
```

# Usage

```text
A simple tool to convert a WebAssembly module to a WASI component.

Usage: wasi-update --input <FILE> --output <FILE>

Options:
  -i, --input <FILE>   Input file (regular module)
  -o, --output <FILE>  Output file (component)
  -h, --help           Print help
  -V, --version        Print version
```
