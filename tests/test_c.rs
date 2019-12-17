extern crate rust_code_analysis;

use std::path::PathBuf;

use crate::rust_code_analysis::*;

fn parse(samples: &[&str], debug: bool) {
    let path = PathBuf::from("foo.c");
    for (n, sample) in samples.iter().enumerate() {
        let v_sample = sample.as_bytes().to_vec();
        let parser = CppParser::new(v_sample.clone(), &path, None);
        let root = parser.get_root();
        if debug || root.has_error() {
            eprintln!("Sample (CPP) {}: {}", n, sample);
            dump_node(&v_sample, &root, -1, None, None).unwrap();
        }
        assert!(!root.has_error());
    }
}

#[test]
fn test_fn_macros() {
    let samples = vec![
        "MOZ_ALWAYS_INLINE void f() { }",
        "MOZ_NEVER_INLINE void f() { }",
    ];
    parse(&samples, false);
}

#[test]
fn test_fn_macros_cpp() {
    let samples = vec!["class MOZ_NONHEAP_CLASS Factory : public IClassFactory {};"];
    parse(&samples, false);
}

#[test]
fn test_fn_id_strings() {
    let samples = vec!["nsPrintfCString(\"%\" PRIi32, lifetime.mTag);"];
    parse(&samples, false);
}
