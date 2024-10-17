# Nodes

The `rust-code-analysis-cli` provides commands to analyze and extract information from the nodes in the **Abstract Syntax Tree (AST)** of a source file.

## Error Detection

To detect syntactic errors in your code, run:

```console
rust-code-analysis-cli -p /path/to/your/file/or/directory -I "*.ext" -f error
```

- `-p`: Path to a file or directory (analyzes all files in the directory).
- `-I`: Glob filter for selecting files by extension (e.g., `*.js`, `*.rs`).
- `-f`: Flag to search for nodes of a specific type (e.g., errors).


## Counting Nodes

You can count the number of specific node types in your code by using the `--count` flag:

```console
rust-code-analysis-cli -p /path/to/your/file/or/directory -I "*.ext" --count <NODE_TYPE>
```
This counts how many nodes of the specified type exist in the analyzed files.

## Printing the AST

To visualize the AST of a source file, use the `-d` flag:

```console
rust-code-analysis-cli -p /path/to/your/file/or/directory -d
```
The `-d` flag prints the entire AST, allowing you to inspect the code's syntactic structure.

## Analyzing Code Portions

To analyze only a specific part of the code, use the `--ls` (line start) and `--le` (line end) options.
For example, if we want to print the AST of a single function which starts at line 5 and ends at line 10:

```console
rust-code-analysis-cli -p /path/to/your/file/or/directory -d --ls 5 --le 10
```
