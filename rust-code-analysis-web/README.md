# rust-code-analysis-web

`rust-code-analysis-web` is a web server that facilitates the analysis of source code through a RESTful API. It is part of the rust-code-analysis suite and allows developers to remotely interact with the code analysis features provided by the library.

## Features

- **Comment Removal**: Removes comments from source code to produce a clean version.
- **Function Spans**: Retrieves the start and end lines of functions in the provided source code.
- **Metrics Calculation**: Computes various static analysis metrics for the source code.

## Project Structure

```sh
rust-code-analysis-web
├── Cargo.toml      # Project metadata, dependencies, and build configuration
└── src
    ├── bin/
    │   └── rust-code-analysis-web.rs  # Entry point for running the web server with CLI options
    ├── web/
    │   ├── comment.rs    # Logic for handling requests related to removing comments from source code
    │   ├── function.rs   # Logic for extracting and returning function spans from the source code
    │   ├── metrics.rs    # Logic for computing and returning code metrics based on the source code
    │   ├── mod.rs        # Module declarations and routing between web modules
    │   └── server.rs     # Actix web server setup, route definitions, and main server handler logic
    └── lib.rs            # Main library file, sets up the module imports and starts the server
```

## Installation

To use `rust-code-analysis-web`, you need to have Rust installed on your system.

Clone the repository and run the following command:

```sh
cd rust-code-analysis-web/
cargo build --debug # or cargo build --release
```

## Usage

Run the server by specifying the host and port:

```sh
cargo run -- [OPTIONS]
```

### Available Options

- `-j, --num-jobs <NUM_JOBS>`: Number of parallel jobs to run (optional).
- `--host <HOST>`: IP address where the server should run (default is 127.0.0.1).
- `--port <PORT>`: Port to be used by the server (default is 8080).
- `-h, --help`: Show help information.
- `-v, --version`: Show version information.

# Examples

To start the server on a specific host and port:

```sh
cargo run -- --host <HOST> --port <PORT> -j <NUM_JOBS>
```
