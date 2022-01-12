#!/bin/bash

# This script updates the mozcpp grammar automatically.
#
# Usage: ./generate-grammars/generate-mozcpp.sh

# Name of the tree-sitter-cpp crate
TS_CPP_CRATE="tree-sitter-cpp"

# Filename of the JSON file containing the sha1 of the commit associated to
# the current tree-sitter-cpp crate version
JSON_CRATE_FILENAME=".cargo_vcs_info.json"

# Get the current tree-sitter-cpp crate version from the tree-sitter-mozcpp grammar
TS_CPP_VERSION=`grep -m 1 $TS_CPP_CRATE tree-sitter-mozcpp/Cargo.toml | cut -f2 -d "," | cut -f2 -d "=" | tr -d ' ' | tr -d } | tr -d \"`

# Name assigned to the compressed binary crate downloaded from crates.io
CRATE_OUTPUT="$TS_CPP_CRATE-download.gz"

# Link of the current tree-sitter-cpp crate on crates.io
CRATES_IO_LINK="https://crates.io/api/v1/crates/$TS_CPP_CRATE/$TS_CPP_VERSION/download"

# Download the crate from crates.io and uncompress it
wget -O $CRATE_OUTPUT $CRATES_IO_LINK && tar -xf $CRATE_OUTPUT

# Uncompressed directory name
CRATE_DIR="$TS_CPP_CRATE-$TS_CPP_VERSION"

# Get the sha1 of the commit associated to the current tree-sitter-cpp crate version
TS_CPP_SHA1=`grep "sha1" $CRATE_DIR/$JSON_CRATE_FILENAME | cut -f2 -d ":" | tr -d ' ' | tr -d \"`

# Remove compressed binary file and the relative uncompressed directory
rm -rf $CRATE_OUTPUT $CRATE_DIR

# Enter the mozcpp directory
pushd tree-sitter-mozcpp

# Create tree-sitter-cpp directory
mkdir -p $TS_CPP_CRATE

# Enter tree-sitter-cpp directory
pushd $TS_CPP_CRATE

# Shallow clone tree-sitter-cpp to a specific revision
git init
git remote add origin https://github.com/tree-sitter/tree-sitter-cpp.git
git fetch --depth 1 origin $TS_CPP_SHA1
git checkout FETCH_HEAD

# Install tree-sitter-cpp dependencies
npm install -y

# Exit tree-sitter-cpp directory
popd

# Copy tree-sitter-cpp `scanner.cc` functions into the `src` directory
cp --verbose $TS_CPP_CRATE/src/scanner.cc ./src/scanner.cc

# Exit tree-sitter-mozcpp directory
popd

# Generate tree-sitter-mozcpp grammar
./generate-grammars/generate-grammar.sh tree-sitter-mozcpp

# Delete tree-sitter-mozcpp/tree-sitter-cpp directory
rm -rf ./tree-sitter-mozcpp/$TS_CPP_CRATE
