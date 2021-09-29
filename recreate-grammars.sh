#!/bin/bash

# Clean old grammars builds
cargo clean --manifest-path ./enums/Cargo.toml

# Recreate all grammars
cargo run --manifest-path ./enums/Cargo.toml -- -lrust -o ./src/languages

# Recreate C macros
cargo run --manifest-path ./enums/Cargo.toml -- -lc_macros -o ./src/c_langs_macros

# Format the code of the recreated grammars
cargo fmt
