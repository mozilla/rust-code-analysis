# Update grammars

Each programming language needs to be parsed in order to extract its syntax and semantic: the so-called grammar of a language.
In `rust-code-analysis`, we use [tree-sitter](https://github.com/tree-sitter) as parsing library since it provides a set of distinct grammars for each of our
supported programming languages. But a grammar is not a static monolith, it changes over time, and it can also be affected by bugs,
hence it is necessary to update it every now and then.

As now, since we have used `bash` scripts to automate the operations, grammars can be updated natively **only** on `Linux` and `MacOS` systems, but these scripts can also run on `Windows` using `WSL`.

In `rust-code-analysis` we use both **third-party** and **internal** grammars.
The first ones are published on `crates.io` and maintained by external developers,
while the second ones have been thought and defined inside the project to manage variant of some languages
used in `Firefox`.
We are going to explain how to update both of them in the following sections.

## Third-party grammars

Update the grammar version in `Cargo.toml` and `enums/Cargo.toml`. Below an example for the `tree-sitter-java` grammar

```toml
tree-sitter-java = "x.xx.x"
```
where `x` represents a digit.

Run `./recreate-grammars.sh` to recreate and refresh all grammars structures and data

```bash
./recreate-grammars.sh
```

Once the script above has finished its execution, you need to fix, if there are any, all failed tests and problems
introduced by changes in the grammars.

Commit your changes and create a new pull request

## Internal grammars

Update dependency `version` field in `Cargo.toml` and `enums/Cargo.toml`. Below an example for the `tree-sitter-ccomment` grammar

```toml
tree-sitter-ccomment = { path = "./tree-sitter-ccomment", version = "=x.xx.x" }
```
where `x` represents a digit.

Open the `Cargo.toml` file of the chosen grammar and:
 - Set its version to the **same** value present in the main `Cargo.toml` file
 - Increase the `tree-sitter` version to the most recent one

Run the appropriate script to update the grammar by recreating and refreshing every file and script.

For `tree-sitter-ccomment` and `tree-sitter-preproc` run `./generate-grammars/generate-grammar.sh` followed by the name of the grammar.
Below an example always using the `tree-sitter-ccomment` grammar

```bash
./generate-grammars/generate-grammar.sh tree-sitter-ccomment
```

Instead, for `tree-sitter-mozcpp` and `tree-sitter-mozjs`, use their specific scripts.

For `tree-sitter-mozcpp`, run

```bash
./generate-grammars/generate-mozcpp.sh
```

For `tree-sitter-mozjs`, run

```bash
./generate-grammars/generate-mozjs.sh
```

Once the script above has finished its execution, you need to fix, if there are any, all failed tests and problems
introduced by changes in the grammars.

Commit your changes and create a new pull request
