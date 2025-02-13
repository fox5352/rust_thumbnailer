#!/bin/bash

set -e  # Exit immediately if a command fails

# Ensure Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Add WebAssembly target
echo "Adding WebAssembly target for Rust..."
rustup target add wasm32-unknown-unknown

# Ensure wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "Installing wasm-pack..."
    cargo install wasm-pack
fi

# Build Rust project for WebAssembly
echo "Building Rust project for WebAssembly..."
cargo build --target wasm32-unknown-unknown

# Use wasm-pack to package the library
echo "Packaging the library with wasm-pack..."
wasm-pack build --target bundler

echo "✅ Build complete. Check the 'pkg/' directory for the package."
