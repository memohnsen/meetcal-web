#!/usr/bin/env bash

set -euo pipefail

export PATH="$HOME/.cargo/bin:$PATH"
unset NO_COLOR

if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal --default-toolchain stable
fi

rustup target add wasm32-unknown-unknown
cargo install trunk --version 0.21.14 --locked
trunk build --release
