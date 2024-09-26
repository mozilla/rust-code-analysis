# rust-code-analysis-web

`rust-code-analysis-web` is a web server that provides source code analysis capabilities via a RESTful API. It enables developers to interact with the code analysis functionality from the rust-code-analysis suite through HTTP requests.

## Features

- **Comment Removal**: Removes comments from source code to provide a cleaner version of the code.
- **Function Spans**: Retrieves the start and end lines of functions in the given source code.
- **Metrics Calculation**: Computes static analysis metrics for the source code.

Refer to the REST API documentation for detailed information about the available endpoints and parameters.

## Installation

Clone the repository and build the project:

```sh
cd rust-code-analysis-web/
cargo build
```

## Usage

Run the server by specifying the host and port:

```sh
rust-code-analysis-web [OPTIONS]
```

### Available Options

- `-j, --num-jobs <NUM_JOBS>`: Number of parallel jobs to run (optional).
- `--host <HOST>`: IP address where the server should run (default is 127.0.0.1).
- `--port <PORT>`: Port to be used by the server (default is 8080).
- `-h, --help`: Show help information.
- `-v, --version`: Show version information.

## Examples

To start the server on a specific host and port:

```sh
rust-code-analysis-web --host <HOST> --port <PORT> -j <NUM_JOBS>
```
