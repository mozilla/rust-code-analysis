#!/bin/bash

# Update tree-sitter submodules
#
# Usage: ./update-sumbodules.sh $tree-sitter-language

# Update submodule
git submodule update --remote $1

# Recreate the language
pushd enums
cargo clean
cargo run -- -lrust -o ../src/languages
popd

# Format the code
cargo fmt
