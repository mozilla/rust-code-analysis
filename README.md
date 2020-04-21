# rust-code-analysis

[![Task Status](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/badge.svg)](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/latest)
[![codecov](https://codecov.io/gh/mozilla/rust-code-analysis/branch/master/graph/badge.svg)](https://codecov.io/gh/mozilla/rust-code-analysis)

**rust-code-analysis** is a Rust library to analyze the source code of many different programming languages. It is based on a parser generator tool and an incremental parsing library called [Tree Sitter](https://github.com/tree-sitter/tree-sitter).

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
- CLOC: it counts the number of comments in a source file.
- BLANK: it counts the number of blank lines in a source file.
- HALSTEAD: it is a suite that provides a series of information, such as the effort required to maintain the analyzed code, the size in bits to store the program, the difficulty to understand the code, an estimate of the number of bugs present in the codebase, and an estimate of the time needed to implement the software.
- MI: it is a suite that allows to evaluate the maintainability of a software.
- NOM: it counts the number of functions and closures in a file/trait/class.
- NEXITS: it counts the number of possible exit points from a method/function.
- NARGS: it counts the number of arguments of a function/method.

# rust-code-analysis-cli

**rust-code-analysis-cli** is a command line tool thought to interact with
the functions available in the library.
It can print a series of information on your shell such as nodes and metrics.
It can also export metrics as a json file.
Below you can find a series of commands to use the software at best.

### How to get metrics

You can get metrics in your shell in this way

```
rust-code-analysis-cli --metrics --paths /your/path/to/a/file
```

or as a `json` file

```
rust-code-analysis-cli --metrics --output your/path/to/the/output/directory --paths /your/path/to/a/file
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

Build and run the software

```console
# Build rust-code-analysis with cargo
cargo build

# Build rust-code-analysis-cli with cargo
cargo build --all

# Run through cargo
cargo run -- -h
cargo run -- --serve --port 8000
```

## How to run tests

After the build step, run the following command to verify if all tests pass

```
cargo test --verbose
```
