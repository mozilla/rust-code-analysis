# Rest API

It is possible to run **rust-code-analysis-cli** as a `HTTP` service using
`REST API` to share data between client and server.

**rust-code-analysis-cli** can act as a server running on your `localhost`
at a specific port.

```
rust-code-analysis-cli --serve --port [port]
```

The `--port` option sets the port used by the server. One possible value
could be `9090`.

From now on, we will use the port `9090` to show you the possible ways to
interact with the server.

If you want to ping the server, make a `GET` request at this `URL`:

```
http://127.0.0.1:9090/ping
```

To get metrics formatted as a `json` file, make a `POST` request at this `URL`:

```
http://127.0.0.1:9090/metrics?file_name={filename}&unit={unit}
```

The `filename` parameter represents the path to the source file to be analyzed,
while `unit` is a boolean value that can assume only `0` or `1`. The latter
tells **rust-code-analysis-cli** to consider only top-level metrics, while the
former returns detailed metrics for all classes, functions, nested functions,
ecc...
