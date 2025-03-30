#!/bin/bash

# This script updates the mozjs grammar automatically.
#
# Usage: ./generate-grammars/generate-mozjs.sh

# Name of the tree-sitter-javascript crate
TS_JS_CRATE="tree-sitter-javascript"

# Filename of the JSON file containing the sha1 of the commit associated to
# the current tree-sitter-javascript crate version
JSON_CRATE_FILENAME=".cargo_vcs_info.json"

# Get the current tree-sitter-javascript crate version
TS_JS_VERSION=`grep -m 1 $TS_JS_CRATE tree-sitter-mozjs/Cargo.toml | cut -f2 -d "," | cut -f2 -d "=" | tr -d ' ' | tr -d } | tr -d \"`

# Name assigned to the compressed binary crate downloaded from crates.io
CRATE_OUTPUT="$TS_JS_CRATE-download.gz"

# Link of the current tree-sitter-javascript crate on crates.io
CRATES_IO_LINK="https://crates.io/api/v1/crates/$TS_JS_CRATE/$TS_JS_VERSION/download"

# Download the crate from crates.io and uncompress it
wget -O $CRATE_OUTPUT $CRATES_IO_LINK && tar -xf $CRATE_OUTPUT

# Uncompressed directory name
CRATE_DIR="$TS_JS_CRATE-$TS_JS_VERSION"

# Get the sha1 of the commit associated to the current tree-sitter-javascript crate version
TS_JS_SHA1=`grep "sha1" $CRATE_DIR/$JSON_CRATE_FILENAME | cut -f2 -d ":" | tr -d ' ' | tr -d \"`

# Remove compressed binary file and the relative uncompressed directory
rm -rf $CRATE_OUTPUT $CRATE_DIR

# Enter the mozjs directory
pushd tree-sitter-mozjs

# Create tree-sitter-javascript directory
mkdir -p $TS_JS_CRATE

# Enter tree-sitter-javascript directory
pushd $TS_JS_CRATE

# Shallow clone tree-sitter-javascript to a specific revision
git init
git remote add origin https://github.com/tree-sitter/tree-sitter-javascript.git
git fetch --depth 1 origin $TS_JS_SHA1
git checkout FETCH_HEAD

# Exit tree-sitter-javascript directory
popd

# Copy tree-sitter-cpp `scanner.c` functions into the `src` directory
cp --verbose $TS_JS_CRATE/src/scanner.c ./src/scanner.c

# Since the tree-sitter-mozjs `scanner.c` file contains the very same functions
# present in the tree-sitter-javascript `scanner.c` file, to avoid having a
# multiple symbol definition error during the linking phase,
# those functions will be assigned a new prefix.
sed -i 's/tree_sitter_javascript/tree_sitter_mozjs/g' ./src/scanner.c

# Exit tree-sitter-mozjs directory
popd

# Generate tree-sitter-mozjs grammar
./generate-grammars/generate-grammar.sh tree-sitter-mozjs

# Delete tree-sitter-mozjs/tree-sitter-javascript directory
rm -rf ./tree-sitter-mozjs/$TS_JS_CRATE
