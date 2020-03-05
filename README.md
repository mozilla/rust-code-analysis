# rust-code-analysis

[![Task Status](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/badge.svg)](https://community-tc.services.mozilla.com/api/github/v1/repository/mozilla/rust-code-analysis/master/latest)

This project is a Rust crate (library) to analyse source code. This software is based on [Tree Sitter](https://github.com/tree-sitter/tree-sitter).

It supports several languages:
* C++
* C#
* CSS
* Go
* HTML
* Java
* JavaScript
* The JavaScript used in Firefox internal
* Python
* Rust
* Typescript


Short example of the API:
```
fn parse(samples: &[&str]) {
    let path = PathBuf::from("foo.c");
    for (n, sample) in samples.iter().enumerate() {
        let v_sample = sample.as_bytes().to_vec();
        let parser = CppParser::new(v_sample.clone(), &path, None);
        let root = parser.get_root();
        if root.has_error() {
            eprintln!("Sample (CPP) {}: {}", n, sample);
            dump_node(&v_sample, &root, -1, None, None).unwrap();
        }
        assert!(!root.has_error());
    }
}
```
