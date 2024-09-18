# Nodes

The rust-code-analysis-cli provides commands to analyze and extract information from the nodes in the **AST** of a source file.

## Error Detection

To detect syntactic errors in your code, run:

```bash
rust-code-analysis-cli -p /path/to/file/or/directory -I "*.ext" -f error
```

- `-p`: Path to a file or directory (analyzes all files in the directory).
- `-I`: Glob filter for selecting files by extension (e.g., `*.js`, `*.rs`).
- `-f`: Flag to search for nodes of a specific type (e.g., errors).

This command will find and list all syntax errors in the specified code.

## Counting Nodes

You can count the number of specific node types in your code by using the `--count` flag:

```bash
rust-code-analysis-cli -p /path/to/file/or/directory -I "*.ext" --count <NODE_TYPE>
```
This counts how many nodes of the specified type exist in the analyzed files.

## Printing the AST

To visualize the AST of a source file, use the `--dump` flag:

```bash
rust-code-analysis-cli -p /path/to/file/or/directory -I "*.ext" --dump
```
The `-d` flag prints the entire AST, allowing you to inspect the code's syntactic structure.

## Analyzing Code Portions

To analyze only a specific part of the code, use the `--ls` (line start) and `--le` (line end) options:

```bash
rust-code-analysis-cli -p /path/to/file/or/directory -d --ls 5 --le 10
```

This prints the AST of the code between lines 5 and 10, useful for analyzing specific functions or blocks.
