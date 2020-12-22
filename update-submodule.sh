#!/bin/bash

# Update tree-sitter submodule
#
# Usage: ./update-submodule.sh $tree-sitter-language

# Update submodule
git submodule update --remote $1

# Generate the updated grammar for the submodule
./update-language-bindings.sh
