# Metrics

Metrics can be printed on screen or exported as different output formats
through **rust-code-analysis-cli**.

## Print metrics

For each function space, **rust-code-analysis** computes the list of metrics
described above. At the end of this process, **rust-code-analysis-cli**
dumps the result formatted in a certain way on the screen.
The command used to print the metrics is the following one:

```console
rust-code-analysis-cli -m -p /path/to/your/file/or/directory
```

The `-p` option represents the path to a file or a directory. If a directory is
passed as input, **rust-code-analysis-cli** computes the metrics for each file
contained in it.

## Export formats

Different output formats can be used to export metrics:

- Cbor
- Json
- Toml
- Yaml

`Json` and `Toml` can also be exported pretty-printed.

### Export command

For example, if you want to export metrics as a `json` file, run:

```console
rust-code-analysis-cli -m -O json -o /output/path -p /path/to/your/file/or/directory
```

The `-O` option allows you to choose the output format. It supports
**only** these values: *cbor*, *json*, *toml*, *yaml*.

The `-o` option is used to specify the path where your file will be saved.
It accepts **only** paths. The filename of your output file is the same as
your input file plus the extension associated to the format. When this option
is not given, the output is printed on shell.

As we said before, `Json` and `Toml` can be exported as pretty-printed. To do
so, the `--pr` option is used.
In the case below, the pretty-printed `json` output will be printed on shell:

```console
rust-code-analysis-cli -m -O json --pr -p /path/to/your/file/or/directory
```
