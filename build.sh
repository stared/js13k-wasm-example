#!/bin/bash

echo "Building WASM..."
cargo build --release --target wasm32-unknown-unknown

echo "Copying WASM..."
cp target/wasm32-unknown-unknown/release/js13k_invaders.wasm g.wasm

if command -v wasm-opt &> /dev/null; then
    echo "Optimizing WASM with wasm-opt..."
    wasm-opt -Oz g.wasm -o g_opt.wasm
    mv g_opt.wasm g.wasm
else
    echo "wasm-opt not found, skipping optimization"
fi

echo "Build complete!"