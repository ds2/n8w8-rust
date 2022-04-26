#!/usr/bin/env bash

cargo build --package nachtwacht-rust --bin nachtwacht-rust
cargo test
#cargo install --path .
