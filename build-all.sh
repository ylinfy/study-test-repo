#!/usr/bin/env bash

set -eu
cargo +nightly contract build --manifest-path contracts/lending/Cargo.toml
cargo +nightly contract build --manifest-path contracts/loan/Cargo.toml
cargo +nightly contract build --manifest-path contracts/shares/Cargo.toml
cargo +nightly contract build --manifest-path contracts/stable_coin/Cargo.toml
# cargo +nightly contract build --manifest-path contracts/periphery/NonfungiblePositionManager/Cargo.toml
