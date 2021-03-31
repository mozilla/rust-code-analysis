# rust-code-analysis

[![Task Status](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/badge.svg)](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/latest)
[![codecov](https://codecov.io/gh/mozilla/rust-code-analysis/branch/master/graph/badge.svg)](https://codecov.io/gh/mozilla/rust-code-analysis)
<a href="https://chat.mozilla.org/#/room/#rust-code-analysis:mozilla.org" target="_blank">
   <img src="https://img.shields.io/badge/chat%20on%20[m]-%23rust--code--analysis%3Amozilla.org-blue">
</a>

**rust-code-analysis** is a Rust library to analyze and extract information
from source code written in many different programming languages.
It is based on a parser generator tool and an incremental parsing library
called
<a href="https://tree-sitter.github.io/tree-sitter/" target="_blank">Tree Sitter</a>.


A command line tool called **rust-code-analysis-cli** is provided to interact with the API of the library in an easy way.

This tool can be used to:

- Call **rust-code-analysis** API
- Print nodes and metrics information
- Export metrics in different formats

In addition, we provide a **rust-code-analysis-web** tool to use the library through a REST API.


# Usage

**rust-code-analysis** supports many types of programming languages and
computes a great variety of metrics. You can find up to date documentation at
<a href="https://mozilla.github.io/rust-code-analysis/index.html" target="_blank">Documentation</a>.

On the
<a href="https://mozilla.github.io/rust-code-analysis/commands/index.html" target="_blank">
    Commands
</a> page, there is a list of commands that can be run to get information
about metrics, nodes, and other general data provided by this software.

## Building

To build the `rust-code-analysis` library, you need to run the following
command:

```console
cargo build
```

If you want to build the `cli`:

```console
cargo build -p rust-code-analysis-cli
```

If you want to build the `web` server:

```console
cargo build -p rust-code-analysis-web
```

If you want to build everything in one fell swoop:

```console
cargo build --workspace
```

## Testing

To verify whether all tests pass, run the `cargo test` command.

```console
cargo test --workspace --all-features --verbose
```

# Contributing

If you want to contribute to the development of this software, have a look at the
guidelines contained in our
<a href="https://mozilla.github.io/rust-code-analysis/developers/index.html" target="_blank">Developers Guide</a>.


# How to cite rust-code-analysis

```
@article{ARDITO2020100635,
    title = {rust-code-analysis: A Rust library to analyze and extract maintainability information from source codes},
    journal = {SoftwareX},
    volume = {12},
    pages = {100635},
    year = {2020},
    issn = {2352-7110},
    doi = {https://doi.org/10.1016/j.softx.2020.100635},
    url = {https://www.sciencedirect.com/science/article/pii/S2352711020303484},
    author = {Luca Ardito and Luca Barbato and Marco Castelluccio and Riccardo Coppola and Calixte Denizet and Sylvestre Ledru and Michele Valsesia},
    keywords = {Algorithm, Software metrics, Software maintainability, Software quality},
    abstract = {The literature proposes many software metrics for evaluating the source code non-functional properties, such as its complexity and maintainability. The literature also proposes several tools to compute those properties on source codes developed with many different software languages. However, the Rust language emergence has not been paired by the community’s effort in developing parsers and tools able to compute metrics for the Rust source code. Also, metrics tools often fall short in providing immediate means of comparing maintainability metrics between different algorithms or coding languages. We hence introduce rust-code-analysis, a Rust library that allows the extraction of a set of eleven maintainability metrics for ten different languages, including Rust. rust-code-analysis, through the Abstract Syntax Tree (AST) of a source file, allows the inspection of the code structure, analyzing source code metrics at different levels of granularity, and finding code syntax errors before compiling time. The tool also offers a command-line interface that allows exporting the results in different formats. The possibility of analyzing source codes written in different programming languages enables simple and systematic comparisons between the metrics produced from different empirical and large-scale analysis sources.}
}
```


# Licenses

- Mozilla-defined grammars are released under the MIT license.

- **rust-code-analysis**, **rust-code-analysis-cli** and **rust-code-analysis-web**
are released under the
<a href="https://www.mozilla.org/MPL/2.0/" target="_blank">Mozilla Public License v2.0</a>.
