#!/bin/sh

echo "Installing rustup"
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

echo "Installing wasm-pack"
npm install -g wasm-pack

node ./tools/build-core.js
