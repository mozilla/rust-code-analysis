extern crate rust_code_analysis;

use std::path::PathBuf;
use std::sync::Arc;

use crate::rust_code_analysis::*;

fn parse(samples: &[&str], debug: bool, c: bool) {
    let macros = vec!["AA", "BB", "CC"];
    let path = PathBuf::from("foo.c");
    let mut pr = preproc::PreprocResults::default();
    pr.files
        .insert(path.clone(), preproc::PreprocFile::new_macros(&macros));
    let pr = Arc::new(pr);

    for (n, sample) in samples.iter().enumerate() {
        let v_sample = sample.as_bytes().to_vec();
        if c {
            let parser = CParser::new(v_sample.clone(), &path, Some(pr.clone()));
            let root = parser.get_root();
            if debug || root.has_error() {
                eprintln!("Sample (C) {}: {}", n, sample);
                dump_node(&v_sample, &root, -1, None, None).unwrap();
            }
            assert!(!root.has_error());
        } else {
            let parser = CppParser::new(v_sample.clone(), &path, Some(pr.clone()));
            let root = parser.get_root();
            if debug || root.has_error() {
                eprintln!("Sample (CPP) {}: {}", n, sample);
                dump_node(&v_sample, &root, -1, None, None).unwrap();
            }
            assert!(!root.has_error());
        }
    }
}

#[test]
fn test_fn_def() {
    let samples = vec![
        "void f() { }",
        "void AA f() { }",
        "void AA BB() f() { }",
        "AA BB() f() { }",
        "AA BB() f() CC AA() { }",
        "void f(void * AA a, int * AA(b) b) { }",
    ];
    parse(&samples, false, true);
    parse(&samples, false, false);
}

#[test]
fn test_fn_decl() {
    let samples = vec![
        "void f();",
        "void AA f();",
        "void AA BB() f();",
        "AA BB() f();",
        "AA BB() f() CC AA();",
    ];
    parse(&samples, false, true);
    parse(&samples, false, false);
}

#[test]
fn test_decl() {
    let samples = vec![
        "static AA x;",
        "static AA(x) y;",
        "static AA;",
        "static AA(x);",
        "AA x;",
        "int x;",
        "AA(int, x);",
    ];
    parse(&samples, false, true);
    parse(&samples, false, false);
}

#[test]
fn test_strings() {
    let samples = vec![
        "const char * s = \"hello\";",
        "const char * s = AA \"hello\";",
        "const char * s = AA \"hello\" BB;",
        "const char * s = AA CC \"hello\" BB;",
    ];
    parse(&samples, false, true);
    parse(&samples, false, false);
}

#[test]
fn test_assign() {
    let samples = vec![
        "AA x = y;",
        "AA x = (int)y;",
        "AA x = (AA)y;",
        "AA * x = BB;",
        "AA = BB;",
        "AA() = BB;",
        "int AA = 1;",
    ];
    parse(&samples, false, true);
    parse(&samples, false, false);
}

#[test]
fn test_preproc() {
    let samples = vec![
        "#define AA x\nint x = 1;",
        "#define AA(x) ((x)+1)\n",
        "#define AA(x)  ((sizeof(x)-1)\n",
        "#if AA\nint x = 1;\n#endif\n",
        "#if AA\n#define BB\n#endif\n",
        "#if AA&&defined(BB)\n#define CC\n#endif\n",
        "#ifdef AA\n#endif\n",
        "#if !(AA)\n0;\n#else\n1;\n#endif\n",
        "#ifndef AA  /* comment */\n#endif\n",
    ];
    parse(&samples, false, true);
    parse(&samples, false, false);
}

#[test]
fn test_only_macros() {
    let samples = vec![
        "AA\nint x = 1;",
        "AA(ab)\nint x = 1;",
        "AA(ab)\n +2;",
        "switch(x) {\ncase 1:\nAA\nint x = 1;}",
    ];
    parse(&samples, false, true);
    parse(&samples, false, false);
}

#[test]
fn test_fn_cpp() {
    let samples = vec![
        "AA(x)\nstatic T<U> f() { }",
        "class C { public:\nAA(x)\nstatic T<U> f() { }};",
        "AA(x)\nBB(y)\nx->y(CC);\nBB\n",
        "class C { public:\nAA\nBB\n};",
        "class C { void f() override;};",
        "class C { AA f() override;};",
        "AA()\n<< \"aaa\";",
        //"AA(a, b, c) { }",
    ];
    parse(&samples, false, false);
}
