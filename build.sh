#!/bin/bash
cargo build -p wasm --target wasm32-unknown-unknown --release && \
cargo build -p webserver --release && \
wasm-bindgen target/wasm32-unknown-unknown/release/wasm.wasm --target web --out-dir www/assets