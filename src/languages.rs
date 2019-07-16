use enum_iterator::IntoEnumIterator;
use std::path::PathBuf;
use std::sync::Arc;
use tree_sitter::Language;

use crate::preproc::PreprocResults;
use crate::*;

mk_langs!(
    // 1) Name for enum
    // 2) Empty struct name to implement
    // 3) Parser name
    // 4) tree-sitter function to call to get a Language
    // 5) file extensions
    (
        Javascript,
        JavascriptCode,
        JavascriptParser,
        tree_sitter_javascript,
        [js]
    ),
    (Java, JavaCode, JavaParser, tree_sitter_java, [java]),
    (Go, GoCode, GoParser, tree_sitter_go, [go]),
    (Html, HtmlCode, HtmlParser, tree_sitter_html, [html]),
    (C, CCode, CParser, tree_sitter_c, [c, h, hh, inc]),
    (CSharp, CSharpCode, CSharpParser, tree_sitter_c_sharp, [cs]),
    (Rust, RustCode, RustParser, tree_sitter_rust, [rs]),
    (Css, CssCode, CssParser, tree_sitter_css, [css]),
    (
        Cpp,
        CppCode,
        CppParser,
        tree_sitter_cpp,
        [cpp, cxx, cc, hxx, hpp]
    ),
    (Python, PythonCode, PythonParser, tree_sitter_python, [py]),
    (Tsx, TsxCode, TsxParser, tree_sitter_tsx, [tsx]),
    (
        Typescript,
        TypescriptCode,
        TypescriptParser,
        tree_sitter_typescript,
        [ts]
    ),
    (
        Ccomment,
        CcommentCode,
        CcommentParser,
        tree_sitter_ccomment,
        []
    ),
    (Preproc, PreprocCode, PreprocParser, tree_sitter_preproc, [])
);
