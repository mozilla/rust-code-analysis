# rust-code-analysis

[![Task Status](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/badge.svg)](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/latest)
[![codecov](https://codecov.io/gh/mozilla/rust-code-analysis/branch/master/graph/badge.svg)](https://codecov.io/gh/mozilla/rust-code-analysis)
<a href="https://chat.mozilla.org/#/room/#rust-code-analysis:mozilla.org" target="_blank">
   <img src="https://img.shields.io/badge/chat%20on%20[m]-%23rust--code--analysis%3Amozilla.org-blue">
</a>

**rust-code-analysis** is a Rust library to analyze and extract information
from source codes written in many different programming languages.
It is based on a parser generator tool and an incremental parsing library
called
<a href="https://tree-sitter.github.io/tree-sitter/" target="_blank">
    Tree Sitter
</a>.


In addition, we provide a command line tool called **rust-code-analysis-cli**
to interact with the API of the library in an easy way.

This tool can be used to:

- Call **rust-code-analysis** API
- Print nodes and metrics information
- Export metrics in different formats

# Software Usability

**rust-code-analysis** supports many types of programming languages and
computes a great variety of metrics. If you want to discover more on this
software, read our
<a href="https://mozilla.github.io/rust-code-analysis/index.html" target="_blank">
    Documentation
</a>.

On the
<a href="https://mozilla.github.io/rust-code-analysis/commands/index.html" target="_blank">
    Commands
</a> page, you will learn which commands need to be run to get information
about metrics, nodes, and other general data provided by this software.

If you want to contribute to the development of this software
or you are just interested in building **rust-code-analysis**, have a look at the
<a href="https://mozilla.github.io/rust-code-analysis/developers/index.html" target="_blank">
    Developers Guide
</a>.

# License

All the source code of **rust-code-analysis** is released under the
<a href="https://www.mozilla.org/MPL/2.0/" target="_blank">
    Mozilla Public License v2.0
</a>.
