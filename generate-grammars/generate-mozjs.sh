#!/bin/bash

# This script updates the mozjs grammar automatically.
#
# Usage: ./generate-grammars/generate-mozjs.sh

# FIXME we need to remove this line once we are going to use
# the tree-sitter-javascript bindings
# Get the tree-sitter-javascript submodule version
TS_JS_VERSION=`git submodule status tree-sitter-javascript | awk '{ print $1 }'`

# Enter the mozjs directory
pushd tree-sitter-mozjs

# Create tree-sitter-javascript directory
mkdir -p tree-sitter-javascript

# Enter tree-sitter-javascript directory
pushd tree-sitter-javascript

# Shallow clone tree-sitter-javascript to a specific revision
git init
git remote add origin https://github.com/tree-sitter/tree-sitter-javascript.git
git fetch --depth 1 origin $TS_JS_VERSION
git checkout FETCH_HEAD

# Exit tree-sitter-javascript directory
popd

# Since the tree-sitter-mozjs `scanner.cc` file contains the very same functions
# present in the tree-sitter-javascript `scanner.cc` file, to avoid having a
# multiple symbols definitions error during the linking phase,
# to those functions will be assigned a new prefix and the relative
# output file will be saved into the `src` directory.
SED_PATTERN="s/tree_sitter_javascript_external_scanner_/tree_sitter_javascript_external_scanner_mozjs_/g"
sed $SED_PATTERN tree-sitter-javascript/src/scanner.c > ./src/tree_sitter_javascript_scanner.c

# Init npm
npm init -y

# Install a small module that lets the parser be used from Node
npm install --save nan

# Install the Tree-sitter CLI
npm install --save-dev tree-sitter-cli

# Generate moz-cpp grammar
./node_modules/.bin/tree-sitter generate

# Delete node_modules
rm -rf node_modules

# Delete package files
rm -rf package-lock.json package.json

# Delete tree-sitter-javascript directory
rm -rf tree-sitter-javascript

# Exit tree-sitter-mozjs directory
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
