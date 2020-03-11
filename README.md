# rust-code-analysis

[![Task Status](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/badge.svg)](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/latest)

This project is a Rust crate (library) to analyse source code. This software is based on [Tree Sitter](https://github.com/tree-sitter/tree-sitter).

It supports several languages:
* C++
* C#
* CSS
* Go
* HTML
* Java
* JavaScript
* The JavaScript used in Firefox internal
* Python
* Rust
* Typescript

## How to build it

```console
# Fetch the tree-sitter dependencies
git submodule init
git submodule update --recursive

# Now build with cargo
cargo build

# It should be available through cargo now
cargo run -- -h
cargo run -- --serve --port 8000
```
