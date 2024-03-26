#!/usr/bin/env bash

cargo update --workspace 
cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features --workspace  -- -D warnings
cargo fix --allow-dirty --allow-staged --all-targets --all-features --workspace 
cargo fmt --all 
cargo check --all-targets --all-features --workspace 
cargo test --all-targets --all-features --workspace 
cargo build --all-targets --all-features --workspace
cargo run --release