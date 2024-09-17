# rust-code-analysis-cli

`rust-code-analysis-cli` is a tool designed to compute and export code metrics, analyze source code, and perform various operations such as removing comments, counting nodes, retrieving functions, and computing code metrics in different formats.

## Features

- Analyze source code for different programming languages.
- Export results in different formats (CBOR, JSON, TOML, YAML).
- Perform various operations on source code (e.g., dumping abstract syntax tree to stdout, count nodes, computing code metrics).

## Project Structure

```sh
rust-code-analysis-cli
├── Cargo.toml      # Project metadata and dependencies
└── src
    ├── formats.rs  # Handles format-specific output (CBOR, JSON, TOML, YAML)
    └── main.rs     # Main logic for command-line operations and code analysis
```

## Installation

To use this tool, you need to have Rust installed on your system.

Clone the repository and run the following command:

```sh
cd rust-code-analysis-cli/
cargo build --debug # or cargo build --release
```

## Usage

Run the tool by specifying the input file and the desired operation:

```sh
cargo run -- [OPTIONS]
```

## Available Options

- `-p, --paths <FILE>...`: Input files to analyze.
- `-d, --dump`: Dump the abstract syntax tree to stdout.
- `-c, --comments`: Remove comments from specified files.
- `-f, --find <NODE_TYPE>`: Find nodes of the given type.
- `-F, --function`: Get functions and their spans.
- `-C, --count <NODE_TYPE>`: Count nodes of the given type.
- `-m, --metrics`: Compute code metrics.
- `--ops`: Retrieve all operands and operators in the code.
- `-i, --in-place`: Do actions in place.
- `-I, --include [<INCLUDE>...]`: Include files matching the given pattern.
- `-X, --exclude [<EXCLUDE>...]`: Exclude files matching the given pattern.
- `-j, --num-jobs <NUM_JOBS>`: Number of threads to use.
- `-l, --language-type <LANGUAGE>`: Language of the input files.
- `-O, --output-format <FORMAT>`: Output format for the results (CBOR, JSON, TOML, YAML).
- `--pr`: Dump a pretty json output file.
- `-o, --output <OUTPUT>`: Output directory for the results.
- `--preproc <PREPROCESSOR>`: Get preprocessor directives for C/C++ files.
- `--ls <LINE_START>`: Start line for the analysis.
- `--le <LINE_END>`: End line for the analysis.
- `-w, --warning`: Show warnings.
- `-v, --version`: Show version information.
- `-h, --help`: Show help information.

## Examples

To analyze the code in a file and export the metrics in JSON format:

```sh
cargo run -- --metrics --output-format json --output . --paths path/to/file.rs
```
