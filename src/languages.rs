use enum_iterator::IntoEnumIterator;
use std::path::PathBuf;
use std::sync::Arc;
use tree_sitter::Language;

use crate::preproc::PreprocResults;
use crate::*;

mk_langs!(
    // 1) Name for enum
    // 2) Display name
    // 3) Empty struct name to implement
    // 4) Parser name
    // 5) tree-sitter function to call to get a Language
    // 6) file extensions
    // 7) emacs modes
    (
        Mozjs,
        "javascript",
        MozjsCode,
        MozjsParser,
        tree_sitter_mozjs,
        [js, jsm],
        ["js", "js2"]
    ),
    (
        Javascript,
        "javascript",
        JavascriptCode,
        JavascriptParser,
        tree_sitter_javascript,
        [],
        []
    ),
    (
        Java,
        "java",
        JavaCode,
        JavaParser,
        tree_sitter_java,
        [java],
        ["java"]
    ),
    (Go, "go", GoCode, GoParser, tree_sitter_go, [go], ["go"]),
    (
        Html,
        "html",
        HtmlCode,
        HtmlParser,
        tree_sitter_html,
        [html],
        ["html"]
    ),
    (
        CSharp,
        "c#",
        CSharpCode,
        CSharpParser,
        tree_sitter_c_sharp,
        [cs],
        ["csharp", "c#"]
    ),
    (
        Rust,
        "rust",
        RustCode,
        RustParser,
        tree_sitter_rust,
        [rs],
        ["rust"]
    ),
    (
        Css,
        "css",
        CssCode,
        CssParser,
        tree_sitter_css,
        [css],
        ["css"]
    ),
    (
        Cpp,
        "c/c++",
        CppCode,
        CppParser,
        tree_sitter_cpp,
        [cpp, cxx, cc, hxx, hpp, c, h, hh, inc, mm],
        ["c++", "c", "objc", "objc++", "objective-c++", "objective-c"]
    ),
    (
        Python,
        "python",
        PythonCode,
        PythonParser,
        tree_sitter_python,
        [py],
        ["python"]
    ),
    (
        Tsx,
        "typescript",
        TsxCode,
        TsxParser,
        tree_sitter_tsx,
        [tsx],
        []
    ),
    (
        Typescript,
        "typescript",
        TypescriptCode,
        TypescriptParser,
        tree_sitter_typescript,
        [ts, jsw, jsmw],
        ["typescript"]
    ),
    (
        Ccomment,
        "ccomment",
        CcommentCode,
        CcommentParser,
        tree_sitter_ccomment,
        [],
        []
    ),
    (
        Preproc,
        "preproc",
        PreprocCode,
        PreprocParser,
        tree_sitter_preproc,
        [],
        []
    )
);

pub(crate) mod fake {
    pub fn get_true(ext: &str, mode: &str) -> Option<String> {
        if ext == "m"
            || ext == "mm"
            || mode == "objc"
            || mode == "objc++"
            || mode == "objective-c++"
            || mode == "objective-c"
        {
            Some("obj-c/c++".to_string())
        } else {
            None
        }
    }
}
