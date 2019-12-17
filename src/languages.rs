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
    // 6) emacs modes
    (
        Mozjs,
        MozjsCode,
        MozjsParser,
        tree_sitter_mozjs,
        [js, jsm],
        ["js"]
    ),
    (
        Javascript,
        JavascriptCode,
        JavascriptParser,
        tree_sitter_javascript,
        [],
        []
    ),
    (
        Java,
        JavaCode,
        JavaParser,
        tree_sitter_java,
        [java],
        ["java"]
    ),
    (Go, GoCode, GoParser, tree_sitter_go, [go], ["go"]),
    (
        Html,
        HtmlCode,
        HtmlParser,
        tree_sitter_html,
        [html],
        ["html"]
    ),
    (
        CSharp,
        CSharpCode,
        CSharpParser,
        tree_sitter_c_sharp,
        [cs],
        ["csharp", "c#"]
    ),
    (Rust, RustCode, RustParser, tree_sitter_rust, [rs], ["rust"]),
    (Css, CssCode, CssParser, tree_sitter_css, [css], ["css"]),
    (
        Cpp,
        CppCode,
        CppParser,
        tree_sitter_cpp,
        [cpp, cxx, cc, hxx, hpp, c, h, hh, inc],
        ["c++", "c"]
    ),
    (
        Python,
        PythonCode,
        PythonParser,
        tree_sitter_python,
        [py],
        ["python"]
    ),
    (Tsx, TsxCode, TsxParser, tree_sitter_tsx, [tsx], []),
    (
        Typescript,
        TypescriptCode,
        TypescriptParser,
        tree_sitter_typescript,
        [ts, jsw, jsmw],
        ["typescript"]
    ),
    (
        Ccomment,
        CcommentCode,
        CcommentParser,
        tree_sitter_ccomment,
        [],
        []
    ),
    (
        Preproc,
        PreprocCode,
        PreprocParser,
        tree_sitter_preproc,
        [],
        []
    )
);
