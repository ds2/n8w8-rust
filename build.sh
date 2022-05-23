#!/usr/bin/env bash

cargo fmt
cargo build --workspace
cargo test
#cargo install --path .
