# Commands

With the term **command**, we define any procedure used by
**rust-code-analysis-cli** to extract information from source codes.
At each command **may** be associated parameters depending on the task
it needs to carry out.
In this page we have grouped the principal **types** of commands implemented in
**rust-code-analysis-cli**.

## Metrics

Metrics are a series of measures that can be used to:

- Compare different programming languages
- Provide information on the quality of a code
- Tell developers where their code is more tough to handle
- Discover errors earlier

**rust-code-analysis** calculates the metrics starting from the
source code of a program. These kind of metrics are called *static metrics*.

The list of metrics supported right now are the following ones:

- **CC**: it calculates the code complexity examining the
  control flow of a program.
- **SLOC**: it counts the number of lines in a source file.
- **PLOC**: it counts the number of physical lines (instructions) contained in
  a source file.
- **LLOC**: it counts the number of logical lines (statements) contained in
  a source file.
- **CLOC**: it counts the number of comments in a source file.
- **BLANK**: it counts the number of blank lines in a source file.
- **HALSTEAD**: it is a suite that provides a series of information, such as the
  effort required to maintain the analyzed code, the size in bits to store the
  program, the difficulty to understand the code, an estimate of the number of
  bugs present in the codebase, and an estimate of the time needed to
  implement the software.
- **MI**: it is a suite that allows to evaluate the maintainability of a software.
- **NOM**: it counts the number of functions and closures in a file/trait/class.
- **NEXITS**: it counts the number of possible exit points from a method/function.
- **NARGS**: it counts the number of arguments of a function/method.

## Nodes

To represent the structure of program code, **rust-code-analysis-cli** builds
an
<a href="https://en.wikipedia.org/wiki/Abstract_syntax_tree" target="_blank">Abstract Syntax Tree (AST)</a>.
A **node** is an element of this tree and denotes any syntactic construct
present in a language.

Nodes can be used to:

- Create the syntactic structure of a source file
- Discover if a construct of a language is present in the analyzed
  code
- Count the number of constructs of a certain kind
- Detect errors i the source code

## REST API

**rust-code-analysis-cli** can be run as a server which accepts requests sent
through `REST API`.
The server receives in input the filename of a source code file and returns the
relative metrics formatted as a `json` file.
