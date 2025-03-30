# tree-sitter-ccomment

This crate provides a Ccomment grammar for the [tree-sitter][] parsing library. To
use this crate, add it to the `[dependencies]` section of your `Cargo.toml`
file.  (Note that you will probably also need to depend on the
[`tree-sitter`][tree-sitter crate] crate to use the parsed result in any useful
way.)

``` toml
[dependencies]
tree-sitter = "0.25.3"
tree-sitter-ccomment = "0.20.3"
```

Typically, you will use the [LANGUAGE][] function to add this
grammar to a tree-sitter [Parser][], and then use the parser to parse some code:

``` rust
let code = r#"
    int double(int x) {
        return x * 2;
    }
"#;
let mut parser = Parser::new();
let language = tree_sitter_ccomment::LANGUAGE;
parser
    .set_language(&language.into())
    .expect("Error loading Ccomment parser");
let tree = parser.parse(code, None).unwrap();
assert!(!tree.root_node().has_error());
```

If you have any questions, please reach out to us in the [tree-sitter
discussions] page.

[Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
[Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
[tree-sitter]: https://tree-sitter.github.io/
[tree-sitter crate]: https://crates.io/crates/tree-sitter
[tree-sitter discussions]: https://github.com/tree-sitter/tree-sitter/discussions
