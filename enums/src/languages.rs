use tree_sitter::Language;

mk_langs!(
    // 1) Name for enum
    // 2) tree-sitter function to call to get a Language
    (Java, tree_sitter_java),
    (Rust, tree_sitter_rust),
    (Cpp, tree_sitter_cpp),
    (Python, tree_sitter_python),
    (Tsx, tree_sitter_tsx),
    (Typescript, tree_sitter_typescript),
    (Ccomment, tree_sitter_ccomment),
    (Preproc, tree_sitter_preproc),
    (Mozjs, tree_sitter_mozjs),
    (Javascript, tree_sitter_javascript)
);
