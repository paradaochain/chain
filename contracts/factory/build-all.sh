#!/usr/bin/env bash

set -eu

cargo +nightly contract build --manifest-path ../dao/Cargo.toml
cargo +nightly contract build
