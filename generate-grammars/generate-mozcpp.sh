#!/bin/bash

# This script updates the mozcpp grammar automatically.
#
# Usage: ./generate-grammars/generate-mozcpp.sh

# Set tree-sitter-cpp version
TS_CPP_VERSION="a35a275df92e7583df38f2de2562361f2b69987e"

# Enter the mozcpp directory
pushd tree-sitter-mozcpp

# Create tree-sitter-cpp directory
mkdir -p tree-sitter-cpp

# Enter tree-sitter-cpp directory
pushd tree-sitter-cpp

# Shallow clone tree-sitter-cpp to a specific revision
git init
git remote add origin https://github.com/tree-sitter/tree-sitter-cpp.git
git fetch --depth 1 origin $TS_CPP_VERSION
git checkout FETCH_HEAD

# Install tree-sitter-cpp dependencies
npm install -y

# Exit tree-sitter-cpp directory
popd

# Copy tree-sitter-cpp `scanner.cc` functions into the `src` directory
cp --verbose tree-sitter-cpp/src/scanner.cc ./src/scanner.cc

# Exit tree-sitter-mozcpp directory
popd

# Generate tree-sitter-mozcpp grammar
./generate-grammars/generate-grammar.sh tree-sitter-mozcpp

# Delete tree-sitter-mozcpp/tree-sitter-cpp directory
rm -rf ./tree-sitter-mozcpp/tree-sitter-cpp
