use aho_corasick::AhoCorasick;
use regex::bytes::Regex;

use crate::*;

macro_rules! mk_else_if {
    ($if_type: ident, $block_type: ident) => {
        #[inline(always)]
        fn is_else_if(node: &Node) -> bool {
            if node.object().kind_id() != <Self as TSLanguage>::BaseLang::$if_type {
                return false;
            }
            if let Some(parent) = node.object().parent() {
                if parent.kind_id() == <Self as TSLanguage>::BaseLang::ElseClause {
                    return true;
                }
                if let Some(grandparent) = parent.parent() {
                    return grandparent.kind_id() == <Self as TSLanguage>::BaseLang::ElseClause
                        && parent.kind_id() == <Self as TSLanguage>::BaseLang::$block_type
                        && parent.child_count() == 3; // Grammars count { } as children too
                }
            }
            false
        }
    };
}

pub trait Checker {
    fn is_comment(node: &Node) -> bool;

    #[inline(always)]
    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool;
    fn is_call(node: &Node) -> bool;
    fn is_func(node: &Node) -> bool;
    fn is_func_space(node: &Node) -> bool;
    fn is_non_arg(node: &Node) -> bool;
    fn is_else_if(_: &Node) -> bool;

    fn is_error(node: &Node) -> bool {
        node.object().is_error()
    }
}

impl Checker for PreprocCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, StringLiteral, RawStringLiteral);
    mk_checker!(is_call,);
    mk_checker!(is_func,);
    mk_checker!(is_func_space,);
    mk_checker!(is_non_arg,);

    #[inline(always)]
    fn is_else_if(_: &Node) -> bool {
        unimplemented!()
    }
}

impl Checker for CcommentCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, StringLiteral, RawStringLiteral);
    mk_checker!(is_call,);
    mk_checker!(is_func,);
    mk_checker!(is_func_space,);
    mk_checker!(is_non_arg,);

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            static ref AC: AhoCorasick = AhoCorasick::new(vec![b"<div rustbindgen"]);
        }
        let code = &code[node.object().start_byte()..node.object().end_byte()];
        AC.is_match(code)
    }

    #[inline(always)]
    fn is_else_if(_: &Node) -> bool {
        unimplemented!()
    }
}

impl Checker for CppCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(
        is_string,
        StringLiteral,
        ConcatenatedString,
        RawStringLiteral
    );
    mk_checker!(is_call, CallExpression);
    mk_checker!(
        is_func,
        FunctionDefinition,
        FunctionDefinition2,
        FunctionDefinition3
    );
    mk_checker!(
        is_func_space,
        TranslationUnit,
        FunctionDefinition,
        FunctionDefinition2,
        FunctionDefinition3,
        StructSpecifier,
        ClassSpecifier,
        NamespaceDefinition
    );

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            static ref AC: AhoCorasick = AhoCorasick::new(vec![b"<div rustbindgen"]);
        }
        let code = &code[node.object().start_byte()..node.object().end_byte()];
        AC.is_match(code)
    }

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.object().kind_id() != <Self as TSLanguage>::BaseLang::IfStatement {
            return false;
        }
        if let Some(parent) = node.object().parent() {
            if parent.kind_id() == <Self as TSLanguage>::BaseLang::IfStatement {
                return true;
            }
            if let Some(grandparent) = parent.parent() {
                return grandparent.kind_id() == <Self as TSLanguage>::BaseLang::IfStatement
                    && parent.kind_id() == <Self as TSLanguage>::BaseLang::CompoundStatement
                    && parent.child_count() == 3; // Grammar count { } as children too
            }
        }
        false
    }
    mk_checker!(is_non_arg, LPAREN, LPAREN2, COMMA, RPAREN);
}

impl Checker for PythonCode {
    mk_checker!(is_comment, Comment);

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            // comment containing coding info are useful
            static ref RE: Regex = Regex::new(r"^[ \t\f]*#.*?coding[:=][ \t]*([-_.a-zA-Z0-9]+)").unwrap();
        }
        node.object().start_position().row <= 1
            && RE.is_match(&code[node.object().start_byte()..node.object().end_byte()])
    }

    mk_checker!(is_string, String, ConcatenatedString);
    mk_checker!(is_call, Call);
    mk_checker!(is_func, FunctionDefinition);
    mk_checker!(is_func_space, Module, FunctionDefinition, ClassDefinition);
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.object().kind_id() == <Self as TSLanguage>::BaseLang::ElifClause {
            return true;
        }
        if node.object().kind_id() == <Self as TSLanguage>::BaseLang::IfStatement {
            if let Some(parent) = node.object().parent() {
                if let Some(grandparent) = parent.parent() {
                    return parent.kind_id() == <Self as TSLanguage>::BaseLang::Block
                        && grandparent.kind_id() == <Self as TSLanguage>::BaseLang::ElseClause
                        && parent.child_count() == 1;
                }
            }
        }
        false
    }
}

impl Checker for JavaCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, StringLiteral);
    mk_checker!(is_call, MethodInvocation);
    mk_checker!(is_func, MethodDeclaration);
    mk_checker!(is_func_space, Program, ClassDeclaration);
    mk_checker!(is_non_arg,);

    #[inline(always)]
    fn is_else_if(_: &Node) -> bool {
        unimplemented!()
    }
}

impl Checker for MozjsCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, String, TemplateString);
    mk_checker!(is_call, CallExpression);
    mk_checker!(
        is_func,
        Function,
        GeneratorFunction,
        FunctionDeclaration,
        GeneratorFunctionDeclaration,
        MethodDefinition,
        ArrowFunction
    );
    mk_checker!(
        is_func_space,
        Program,
        Function,
        Class,
        GeneratorFunction,
        FunctionDeclaration,
        MethodDefinition,
        GeneratorFunctionDeclaration,
        ClassDeclaration,
        ArrowFunction
    );

    mk_else_if!(IfStatement, StatementBlock);
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for JavascriptCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, String, TemplateString);
    mk_checker!(is_call, CallExpression);
    mk_checker!(
        is_func,
        Function,
        GeneratorFunction,
        FunctionDeclaration,
        GeneratorFunctionDeclaration,
        MethodDefinition,
        ArrowFunction
    );
    mk_checker!(
        is_func_space,
        Program,
        Function,
        GeneratorFunction,
        Class,
        FunctionDeclaration,
        MethodDefinition,
        GeneratorFunctionDeclaration,
        ClassDeclaration,
        ArrowFunction
    );

    mk_else_if!(IfStatement, StatementBlock);
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for TypescriptCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, String, TemplateString);
    mk_checker!(is_call, CallExpression);
    mk_checker!(
        is_func,
        Function,
        GeneratorFunction,
        FunctionDeclaration,
        GeneratorFunctionDeclaration,
        MethodDefinition,
        ArrowFunction
    );
    mk_checker!(
        is_func_space,
        Program,
        Function,
        Class,
        GeneratorFunction,
        FunctionDeclaration,
        MethodDefinition,
        GeneratorFunctionDeclaration,
        ClassDeclaration,
        ArrowFunction
    );

    mk_else_if!(IfStatement, StatementBlock);
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for TsxCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, String, TemplateString);
    mk_checker!(is_call, CallExpression);
    mk_checker!(
        is_func,
        Function,
        GeneratorFunction,
        FunctionDeclaration,
        GeneratorFunctionDeclaration,
        MethodDefinition,
        ArrowFunction
    );
    mk_checker!(
        is_func_space,
        Program,
        Function,
        GeneratorFunction,
        Class,
        FunctionDeclaration,
        MethodDefinition,
        GeneratorFunction,
        GeneratorFunctionDeclaration,
        ClassDeclaration,
        ArrowFunction
    );
    mk_else_if!(IfStatement, StatementBlock);
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for RustCode {
    mk_checker!(is_comment, LineComment, BlockComment);

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        if let Some(parent) = node.object().parent() {
            if parent.kind_id() == Rust::TokenTree {
                // A comment could be a macro token
                return true;
            }
        }
        let code = &code[node.object().start_byte()..node.object().end_byte()];
        code.starts_with(b"/// cbindgen:")
    }

    mk_checker!(is_string, StringLiteral, RawStringLiteral);
    mk_checker!(is_call, CallExpression);
    mk_checker!(is_func, FunctionItem, ClosureExpression);
    mk_checker!(
        is_func_space,
        SourceFile,
        FunctionItem,
        ImplItem,
        TraitItem,
        ClosureExpression
    );
    mk_else_if!(IfExpression, Block);
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN, AttributeItem);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    macro_rules! check_else_ifs {
        ($source: expr, $file: expr, $parser: ident, $assert: expr) => {
            let path = PathBuf::from($file);
            let mut trimmed_bytes = $source.trim_end().trim_matches('\n').as_bytes().to_vec();
            trimmed_bytes.push(b'\n');
            let parser = $parser::new(trimmed_bytes, &path, None);

            assert_eq!(find_else_if(&parser), $assert);
        };
    }

    #[inline(always)]
    fn find_else_if<T: ParserTrait>(parser: &T) -> bool {
        let node = parser.get_root();
        let mut cursor = node.object().walk();
        let mut stack = Vec::new();
        let mut children = Vec::new();

        stack.push(node);

        while let Some(node) = stack.pop() {
            if T::Checker::is_else_if(&node) {
                return true;
            }
            cursor.reset(node.object());
            if cursor.goto_first_child() {
                loop {
                    children.push(Node::new(cursor.node()));
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                for child in children.drain(..).rev() {
                    stack.push(child);
                }
            }
        }
        false
    }

    #[test]
    fn python_else_if() {
        check_else_ifs!(
            "if 5 > 6:
               a = 5
             elif 6 == 6:
               a = 6
             else:
               a = 7",
            "foo.py",
            PythonParser,
            true
        );
    }

    #[test]
    fn python_no_else_if() {
        check_else_ifs!(
            "if 5 > 6:
               a = 5
             else:
               a = 7",
            "foo.py",
            PythonParser,
            false
        );
    }

    #[test]
    fn python_if_inside_else() {
        check_else_ifs!(
            "if 5 > 6:
               a = 5
             else:
               a = 6
               if 5 > 6:
                 a = 7",
            "foo.py",
            PythonParser,
            false
        );
    }

    #[test]
    fn python_hidden_else_if() {
        // Conceptual test, an `else` immediately followed by
        // an `if` is an `elif`
        check_else_ifs!(
            "if 5 > 6:
               a = 5
             else:
               if 5 > 6:
                 a = 6",
            "foo.py",
            PythonParser,
            true
        );
    }

    #[test]
    fn python_false_else_if() {
        check_else_ifs!(
            "if 5 > 6:
               a = 5
             else:
               if 5 > 6:
                 a = 6
               a = 7",
            "foo.py",
            PythonParser,
            false
        );
    }

    #[test]
    fn cpp_else_if() {
        check_else_ifs!(
            "int time = 42;
             if (time < 10) {
               time = 5;
             } else if (time < 20) {
               time = 6;
             } else {
               time = 7;
             }",
            "foo.c",
            CppParser,
            true
        );
    }

    #[test]
    fn cpp_no_else_if() {
        check_else_ifs!(
            "int time = 42;
             if (time < 10) {
               time = 5;
             } else {
               time = 6;
             }",
            "foo.c",
            CppParser,
            false
        );
    }

    #[test]
    fn cpp_if_inside_else() {
        check_else_ifs!(
            "int time = 42;
             if (time < 10) {
               time = 5;
             } else {
               time = 7;
               if (time < 20) {
                 time = 8;
               }
             }",
            "foo.c",
            CppParser,
            false
        );
    }

    #[test]
    fn cpp_hidden_else_if() {
        // Conceptual test, an `else` immediately followed by
        // an `if` is an `else if`
        check_else_ifs!(
            "int time = 42;
             if (time < 10) {
               time = 5;
             } else {
               if (time < 20) {
                 time = 6;
               }
             }",
            "foo.c",
            CppParser,
            true
        );
    }

    #[test]
    fn cpp_false_else_if() {
        check_else_ifs!(
            "int time = 42;
             if (time < 10) {
               time = 5;
             } else {
               if (time < 20) {
                 time = 6;
               }
               time = 7;
             }",
            "foo.c",
            CppParser,
            false
        );
    }

    #[test]
    fn javascript_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else if (time < 20) {
               time = 7;
             } else {
               time = 8;
             }",
            "foo.js",
            JavascriptParser,
            true
        );
    }

    #[test]
    fn javascript_no_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               time = 8;
             }",
            "foo.js",
            JavascriptParser,
            false
        );
    }

    #[test]
    fn javascript_if_inside_else() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               time = 7;
               if (time < 10) {
                 time = 8;
               }
             }",
            "foo.js",
            JavascriptParser,
            false
        );
    }

    #[test]
    fn javascript_hidden_else_if() {
        // Conceptual test, an `else` immediately followed by
        // an `if` is an `else if`
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               if (time < 10) {
                 time = 8;
               }
             }",
            "foo.js",
            JavascriptParser,
            true
        );
    }

    #[test]
    fn javascript_false_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               if (time < 10) {
                 time = 8;
               }
               time = 9;
             }",
            "foo.js",
            JavascriptParser,
            false
        );
    }

    #[test]
    fn mozjs_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else if (time < 20) {
               time = 7;
             } else {
               time = 8;
             }",
            "foo.js",
            MozjsParser,
            true
        );
    }

    #[test]
    fn mozjs_no_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               time = 8;
             }",
            "foo.js",
            MozjsParser,
            false
        );
    }

    #[test]
    fn mozjs_if_inside_else() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               time = 7;
               if (time < 10) {
                 time = 8;
               }
             }",
            "foo.js",
            MozjsParser,
            false
        );
    }

    #[test]
    fn mozjs_hidden_else_if() {
        // Conceptual test, an `else` immediately followed by
        // an `if` is an `else if`
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               if (time < 10) {
                 time = 8;
               }
             }",
            "foo.js",
            MozjsParser,
            true
        );
    }

    #[test]
    fn mozjs_false_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               if (time < 10) {
                 time = 8;
               }
               time = 9;
             }",
            "foo.js",
            MozjsParser,
            false
        );
    }

    #[test]
    fn typescript_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else if (time < 20) {
               time = 7;
             } else {
               time = 8;
             }",
            "foo.ts",
            TypescriptParser,
            true
        );
    }

    #[test]
    fn typescript_no_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               time = 8;
             }",
            "foo.ts",
            TypescriptParser,
            false
        );
    }

    #[test]
    fn typescript_if_inside_else() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               time = 7;
               if (time < 10) {
                 time = 8;
               }
             }",
            "foo.ts",
            TypescriptParser,
            false
        );
    }

    #[test]
    fn typescript_hidden_else_if() {
        // Conceptual test, an `else` immediately followed by
        // an `if` is an `else if`
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               if (time < 10) {
                 time = 8;
               }
             }",
            "foo.ts",
            TypescriptParser,
            true
        );
    }

    #[test]
    fn typescript_false_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               if (time < 10) {
                 time = 8;
               }
               time = 9;
             }",
            "foo.ts",
            TypescriptParser,
            false
        );
    }

    #[test]
    fn tsx_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else if (time < 20) {
               time = 7;
             } else {
               time = 8;
             }",
            "foo.ts",
            TsxParser,
            true
        );
    }

    #[test]
    fn tsx_no_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               time = 8;
             }",
            "foo.ts",
            TsxParser,
            false
        );
    }

    #[test]
    fn tsx_if_inside_else() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               time = 7;
               if (time < 10) {
                 time = 8;
               }
             }",
            "foo.ts",
            TsxParser,
            false
        );
    }

    #[test]
    fn tsx_hidden_else_if() {
        // Conceptual test, an `else` immediately followed by
        // an `if` is an `else if`
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               if (time < 10) {
                 time = 8;
               }
             }",
            "foo.ts",
            TsxParser,
            true
        );
    }

    #[test]
    fn tsx_false_else_if() {
        check_else_ifs!(
            "var time = 5;
             if (time < 10) {
               time = 6;
             } else {
               if (time < 10) {
                 time = 8;
               }
               time = 9;
             }",
            "foo.ts",
            TsxParser,
            false
        );
    }

    #[test]
    fn rust_else_if() {
        check_else_ifs!(
            "let mut n = 5;
             if n < 0 {
               n = 6;
             } else if n > 0 {
               n = 7;
             } else {
               n = 8;
             }",
            "foo.rs",
            RustParser,
            true
        );
    }

    #[test]
    fn rust_no_else_if() {
        check_else_ifs!(
            "let mut n = 5;
             if n < 0 {
               n = 6;
             } else {
               n = 8;
             }",
            "foo.rs",
            RustParser,
            false
        );
    }

    #[test]
    fn rust_if_inside_else() {
        check_else_ifs!(
            "let mut n = 5;
             if n < 0 {
               n = 6;
             } else {
               n = 7;
               if n < 0 {
                 n = 8;
               }
             }",
            "foo.rs",
            RustParser,
            false
        );
    }

    #[test]
    fn rust_hidden_else_if() {
        // Conceptual test, an `else` immediately followed by
        // an `if` is an `else if`
        check_else_ifs!(
            "let mut n = 5;
             if n < 0 {
               n = 6;
             } else {
               if n < 0 {
                 n = 8;
               }
             }",
            "foo.rs",
            RustParser,
            true
        );
    }

    #[test]
    fn rust_false_else_if() {
        check_else_ifs!(
            "let mut n = 5;
             if n < 0 {
               n = 6;
             } else {
               if n < 0 {
                 n = 8;
               }
               n = 9;
             }",
            "foo.rs",
            RustParser,
            false
        );
    }
}
