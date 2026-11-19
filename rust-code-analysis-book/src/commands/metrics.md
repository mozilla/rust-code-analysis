# Metrics

Metrics can be displayed or exported in various formats using **rust-code-analysis-cli**.

## Display Metrics

To compute and display metrics for a given file or directory, run:

```bash
rust-code-analysis-cli -m -p /path/to/your/file/or/directory
```

- `-p`: Path to the file or directory to analyze. If a directory is provided, metrics will be computed for all supported files it contains.

## Exporting Metrics

**rust-code-analysis-cli** supports multiple output formats for exporting metrics, including:

- CBOR
- JSON
- TOML
- YAML

Both `JSON` and `TOML` can be exported as pretty-printed.

### Export Command

To export metrics as a JSON file:

```bash
rust-code-analysis-cli -m -p /path/to/your/file/or/directory -O json -o /path/to/output/directory
```

- `-O`: Specifies the output format (e.g., json, toml, yaml, cbor).
- `-o`: Path to save the output file. The filename of the output file is the same as the input file plus the extension associated to the format. If not specified, the result will be printed in the shell. 

### Pretty Print

To output pretty-printed JSON metrics:

```bash
rust-code-analysis-cli -m -p /path/to/your/file/or/directory --pr -O json
```

This command prints the formatted metrics to the console or the specified output path.

### Progress Reporting

Analyzing a large directory can take a while. To report the progress of the analysis on stderr:

```bash
rust-code-analysis-cli -m -p /path/to/your/directory -O json -o /path/to/output/directory --progress
```

When stderr is a terminal, a progress bar is shown. Otherwise, for instance in CI logs, a status line is printed every 30 seconds.
