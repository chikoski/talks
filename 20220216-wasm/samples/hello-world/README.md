# hello-world

This folder contains a sample Rust project for WebAssembly package development with [wasm-pack](https://rustwasm.github.io/wasm-pack/).

# How to build

Run `wasm-pack build` to build a Wasm file and relevant JavaScript files. `wasm-pack pack` command will emit a tgz file containg all necessary files to use the library as npm package.

~~~sh
% wasm-pack build
% wasm-pack pack
~~~

# License

All files in src folder and Cargo.toml are distributed under MIT license. 