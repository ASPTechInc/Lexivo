#!/usr/bin/env bash
# This script runs various CI-like checks in a convenient way.
set -eux

cargo check --quiet --workspace --all-targets
cargo check --quiet --workspace --all-features --lib --target wasm32-unknown-unknown
cargo check --quiet --workspace --all-features --lib --target aarch64-linux-android
cargo fmt --all -- --check
cargo clippy --quiet --workspace --all-targets --all-features --  -D warnings -W clippy::all
cargo test --quiet --workspace --all-targets --all-features
cargo test --quiet --workspace --doc
trunk build
