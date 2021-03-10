#!/bin/bash

# This script generates a Mozilla-defined grammar automatically.
#
# Usage: ./generate-grammars/generate-grammar.sh GRAMMAR_NAME

# Enter grammar directory
pushd $1

# Init npm
npm init -y

# Install a small module that lets the parser be used from Node
npm install --save nan

# Install the Tree-sitter CLI
npm install --save-dev tree-sitter-cli

# Generate grammar
./node_modules/.bin/tree-sitter generate

# Delete node_modules
rm -rf node_modules

# Exit grammar directory
popd

# Enter enums directory
pushd enums

# Recreate the grammar for rust-code-analysis
cargo clean && cargo run -- -lrust -o ../src/languages

# Exit enums directory
popd

# Format the produced grammars
cargo fmt

# Run rust code-analysis to verify if everything works correctly and to
# update the Cargo.lock
cargo test --workspace
