#!/bin/bash
set -e

export RUSTFLAGS='-C link-arg=-s'

cargo fmt --all
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/authz_demo.wasm authz_demo.wasm
