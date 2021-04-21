#!/bin/bash

# Clean old grammars builds
cargo clean --manifest-path ./enums/Cargo.toml

# Recreate all grammars
cargo run --manifest-path ./enums/Cargo.toml -- -lrust -o ./src/languages

# Format the code of the recreated grammars
cargo fmt
