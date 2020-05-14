# Nodes

**rust-code-analysis-cli** allows to extract some information from the nodes
which compose the *Abstract Syntax Tree (AST)* of a source code.

## Find Errors

To know if there are some syntactic errors in your code, run:

```console
rust-code-analysis-cli -p /path/to/your/file/or/directory -I "*.ext" -f -error
```

The `-p` option represents the path to a file or a directory. If a directory is
passed as input, **rust-code-analysis-cli** computes the metrics for each file
contained in it.
The `-I` option is a glob filter used to consider only the files written in
the language defined by the extension of the file.
The `-f` option instead searches all nodes of a certain type.
In the case above, we are looking for all the erroneous nodes present in the
code.

It is also possible to count the number of nodes of a certain type using the
`--count` option:

```console
rust-code-analysis-cli -p /path/to/your/file/or/directory -I "*.ext" --count -error
```

## Print AST

If you want to print the AST of a source code, run the following command:

```console
rust-code-analysis-cli -p /path/to/your/file/or/directory -d
```

The `-d` option prints the entire AST on the shell.

## Code Splitting

Commands can be run on a single portion of the code using the `--ls`
and `--le` options. The former represents the starting line of the code to be
considered, while the latter its ending line.
For example, if we want to print the AST of a single function which starts at
line 5 and ends at line 10, we need to launch this command:

```console
rust-code-analysis-cli -p /path/to/your/file/or/directory -d --ls 5 --le 10
```
