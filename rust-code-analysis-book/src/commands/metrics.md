# Metrics Overview

Metrics can be displayed or exported in various formats using the **rust-code-analysis-cli**.

## Display Metrics

To compute and display metrics for a given file or directory, run:

```bash
rust-code-analysis-cli -m -p /path/to/your/file/or/directory
```

- `-p`: Path to the file or directory to analyze. If a directory is passed, metrics will be computed for all supported files within.

## Exporting Metrics

The **rust-code-analysis-cli** supports multiple output formats for exporting metrics, including:

- CBOR
- JSON
- TOML
- YAML

Both `Json` and `Toml` can be exported as pretty-printed.

### Export Command

To export metrics as a JSON file:

```bash
rust-code-analysis-cli -m -p /path/to/your/file/or/directory -O json -o /path/to/output/directory
```

- `-O`: Specifies the output format (e.g., json, toml, yaml, cbor).
- `-o`: Path to save the output file. If not specified, the result will be printed in the shell.

## Pretty Print

To output pretty-printed JSON metrics:

```bash
rust-code-analysis-cli -m -p /path/to/your/file/or/directory --pr -O json
```

This command prints the formatted metrics to the console or the specified output path.
