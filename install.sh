#!/bin/bash
rustup target add wasm32-unknown-unknown && \
cargo install -f wasm-bindgen-cli