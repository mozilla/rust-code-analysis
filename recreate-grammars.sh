#!/bin/bash

# Enter enums directory
pushd enums

# Recreate all grammars
cargo clean && cargo run -- -lrust -o ../src/languages

# Exit enums directory
popd

# Format the code of the recreated grammars
cargo fmt
