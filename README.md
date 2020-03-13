# rust-code-analysis

[![Task Status](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/badge.svg)](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/latest)

This project is a Rust crate (library) to analyse source code. This software is based on [Tree Sitter](https://github.com/tree-sitter/tree-sitter).

## Supported Languages

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

## Supported Metrics

- CC: it calculates the code complexity examining the
control flow of a program.
- SLOC: it counts the number of lines in a source file.
- LLOC: it counts the number of logical lines (instructions) contained in a source file.
- HALSTEAD: it is a suite that provides a series of information, such as the effort required to maintain the analyzed code, the size in bits to store the program, the difficulty to understand the code, an estimate of the number of bugs present in the codebase, and an estimate of the time needed to implement the software.
- NEXITS: it counts the number of possible exit points from a method/function.
- NARGS: it counts the number of arguments of a function/method.

## How to get metrics

You can get metrics in your shell in this way

```
rust-code-analysis --metrics --paths /your/path/to/a/file
```

or as a `json` file

```
rust-code-analysis --metrics --output your/path/to/the/output/directory --paths /your/path/to/a/file
```

or you can run it as a `HTTP` service and use its `REST API`


```
rust-code-analysis --serve --port 9090
```


## How to build the software

Clone the repository and its submodules through HTTPS

```
git clone --recurse-submodules -j8 https://github.com/mozilla/rust-code-analysis.git
```

or through SSH

```
git clone --recurse-submodules -j8 git@github.com:mozilla/rust-code-analysis.git
```

Build and run `rust-code-analysis`

```console
# Build with cargo
cargo build

# Run through cargo
cargo run -- -h
cargo run -- --serve --port 8000
```

## How to run tests

After the build step, run the following command in order to verify if all tests pass

```
cargo test --verbose
```
