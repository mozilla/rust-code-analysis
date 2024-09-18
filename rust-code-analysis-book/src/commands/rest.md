# Rest API

**rust-code-analysis-web** is a web server that allows users to analyze source code through a REST API. It provides several endpoints to extract information such as removing comments, retrieving function spans, and computing various metrics. This service is useful for anyone looking to perform code analysis over HTTP.

The server can be run on any host and port, and supports the following main functionalities:

- Remove Comments from source code.
- Retrieve Function Spans for given code.
- Compute Metrics for the provided source code.


## Running the Server

To run the server, you can use the following command:

```sh
rust-code-analysis-web --host 127.0.0.1 --port 9090
```

- `--host` specifies the IP address where the server should run (default is 127.0.0.1).
- `--port` specifies the port to be used (default is 8080).
- `-j` specifies the number of parallel jobs (optional).

## Endpoints

### 1. Ping the Server

Use this endpoint to check if the server is running.

**Request:**

```http
GET http://127.0.0.1:8080/ping
```

**Response:**

- Status Code: `200 OK`
- Body:

```json
{
  "message": "pong"
}
```

### 2. Remove Comments

This endpoint removes comments from the provided source code.

**Request:**

```http
POST http://127.0.0.1:8080/comments
```

**Payload:**

```json
{
  "id": "unique-id",
  "file_name": "filename.ext",
  "code": "source code with comments"
}
```

- `id`: A unique identifier for the request.
- `file_name`: The name of the file being analyzed.
- `code`: The source code with comments.

**Response:**

```json
{
  "id": "unique-id",
  "code": "source code without comments"
}
```

### 3. Retrieve Function Spans

This endpoint retrieves the spans of functions in the provided source code.

**Request:**

```http
POST http://127.0.0.1:8080/functions
```

**Payload:**

```json
{
  "id": "unique-id",
  "file_name": "filename.ext",
  "code": "source code with functions"
}
```

- `id`: A unique identifier for the request.
- `file_name`: The name of the file being analyzed.
- `code`: The source code with functions.

**Response:**

```json
{
  "id": "unique-id",
  "spans": [
    {
      "name": "function_name",
      "start_line": 1,
      "end_line": 10
    }
  ]
}
```

### 4. Compute Metrics

This endpoint computes various metrics for the provided source code.

**Request:**

```http
POST http://127.0.0.1:8080/metrics
```

**Payload:**

```json
{
  "id": "unique-id",
  "file_name": "filename.ext",
  "code": "source code for metrics"
  "unit": false
}
```

- `id`: Unique identifier for the request.
- `file_name`: The filename of the source code file.
- `code`: The source code to analyze.
- `unit`: A boolean value. true to compute only top-level metrics, false for detailed metrics across all units (functions, classes, etc.).

**Response:**

```json
{
  "id": "unique-id",
  "language": "Rust",
  "spaces": {
    "metrics": {
      "cyclomatic_complexity": 5,
      "lines_of_code": 100,
      "function_count": 10
    }
  }
}
```
