# Commands Overview

The **rust-code-analysis-cli** offers a range of **commands** to analyze source code and extract valuable insights. Each command **may** include parameters specific to the task it performs. Below, we describe the core types of commands available in **rust-code-analysis-cli**.

## Metrics

Metrics provide quantitative measures about source code, which can help in:

- Compare different programming languages
- Provide information on the quality of a code
- Tell developers where their code is more tough to handle
- Discovering potential issues early in the development process

**rust-code-analysis** calculates the metrics starting from the
source code of a program. These kind of metrics are called *static metrics*.

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

The **rust-code-analysis-cli** can function as a server using a REST API. This allows users to send source code via HTTP and receive corresponding metrics in `JSON` format, enabling easy integration into web applications or other services.
