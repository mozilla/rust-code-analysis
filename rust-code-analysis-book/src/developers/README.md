# Developers Guide

If you want to contribute to the development of `rust-code-analysis` we have
summarized here a series of guidelines that are supposed to help you in your
building process.

As prerequisite, you need to install the last available version of `Rust`.
You can learn how to do that
<a href="https://www.rust-lang.org/tools/install" target="_blank">here</a>.

## Clone Repository

First of all, you need to clone the repository.
You can do that:

through **HTTPS**

```
git clone -j8 https://github.com/mozilla/rust-code-analysis.git
```

or through **SSH**

```
git clone -j8 git@github.com:mozilla/rust-code-analysis.git
```

## Building

To build the `rust-code-analysis` library, you need to run the following
command:

```console
cargo build
```

If you want to build the `cli`:

```console
cargo build -p rust-code-analysis-cli
```

If you want to build the `web` server:

```console
cargo build -p rust-code-analysis-web
```

If you want to build everything in one fell swoop:

```console
cargo build --workspace
```

## Testing

After you have finished changing the code, you should **always** verify whether
all tests pass with the `cargo test` command.

```console
cargo test --workspace --all-features --verbose
```

## Code Formatting

If all previous steps went well, and you want to make a pull request
to integrate your invaluable help in the codebase, the last step left is
code formatting.

### Rustfmt

This tool formats your code according to Rust style guidelines.

To install:

```console
rustup component add rustfmt
```

To format the code:

```console
cargo fmt
```

### Clippy

This tool helps developers to write better code catching automatically lots of
common mistakes for them. It detects in your code a series of errors and
warnings that **must** be fixed before making a pull request.

To install:

```console
rustup component add clippy
```

To detect errors and warnings:

```console
cargo clippy --workspace --all-targets --
```

## Code Documentation

If you have documented your code, to generate the final documentation,
run this command:

```console
cargo doc --open --no-deps
```

Remove the `--no-deps` option if you also want to build the documentation of
each dependency used by **rust-code-analysis**.

## Run your code

You can run **rust-code-analysis-cli** using:

```console
cargo run -p rust-code-analysis-cli -- [rust-code-analysis-cli-parameters]
```

To know the list of **rust-code-analysis-cli** parameters, run:

```console
cargo run -p rust-code-analysis-cli -- --help
```

You can run **rust-code-analysis-web** using:

```console
cargo run -p rust-code-analysis-web -- [rust-code-analysis-web-parameters]
```

To know the list of **rust-code-analysis-web** parameters, run:

```console
cargo run -p rust-code-analysis-web -- --help
```

## Practical advice

- When you add a new feature, add at least one unit or integration test to
  verify that everything works correctly
- Document public API
- Do not add dead code
- Comment intricate code such that others can comprehend what you have
  accomplished
