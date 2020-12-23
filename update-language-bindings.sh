#!/bin/bash

# Recreate the language
pushd enums
cargo clean
cargo run -- -lrust -o ../src/languages
popd

# Format the code
cargo fmt
