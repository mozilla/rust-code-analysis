use aho_corasick::AhoCorasick;
use regex::bytes::Regex;
use tree_sitter::Node;

use crate::*;

pub trait Checker {
    // TODO: at some point use a type BaseLang = language to avoid to pass
    // lang as first arg of mk_checker
    // right now: error: enum variants on type aliases are experimental
    // ideally add the type in TSLanguage and then <Self as TSLanguage>::BaseLang

    fn is_comment(node: &Node) -> bool;

    #[inline(always)]
    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool;
    fn is_call(node: &Node) -> bool;

    fn is_error(node: &Node) -> bool {
        node.is_error()
    }
}

impl Checker for PreprocCode {
    mk_checker!(Preproc, is_comment, Comment);
    mk_checker!(Preproc, is_string, StringLiteral, RawStringLiteral);
    mk_checker!(Preproc, is_call,);
}

impl Checker for CcommentCode {
    mk_checker!(Ccomment, is_comment, Comment);
    mk_checker!(Ccomment, is_string, StringLiteral, RawStringLiteral);
    mk_checker!(Ccomment, is_call,);

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            static ref AC: AhoCorasick = AhoCorasick::new(vec![b"<div rustbindgen"]);
        }
        let code = &code[node.start_byte()..node.end_byte()];
        AC.is_match(code)
    }
}

impl Checker for CCode {
    mk_checker!(C, is_comment, Comment);
    mk_checker!(C, is_string, StringLiteral, ConcatenatedString);
    mk_checker!(C, is_call, CallExpression);

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            static ref AC: AhoCorasick = AhoCorasick::new(vec![b"<div rustbindgen"]);
        }
        let code = &code[node.start_byte()..node.end_byte()];
        AC.is_match(code)
    }
}

impl Checker for CppCode {
    mk_checker!(Cpp, is_comment, Comment);
    mk_checker!(
        Cpp,
        is_string,
        StringLiteral,
        ConcatenatedString,
        RawStringLiteral
    );
    mk_checker!(Cpp, is_call, CallExpression);

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            static ref AC: AhoCorasick = AhoCorasick::new(vec![b"<div rustbindgen"]);
        }
        let code = &code[node.start_byte()..node.end_byte()];
        AC.is_match(code)
    }
}

impl Checker for CSharpCode {
    mk_checker!(CSharp, is_comment, Comment);
    mk_checker!(CSharp, is_string, StringLiteral);
    mk_checker!(CSharp, is_call,); // TODO not implemented in ts
}

impl Checker for PythonCode {
    mk_checker!(Python, is_comment, Comment);

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            // comment containing coding info are useful
            static ref RE: Regex = Regex::new(r"^[ \t\f]*#.*?coding[:=][ \t]*([-_.a-zA-Z0-9]+)").unwrap();
        }
        node.start_position().row <= 1 && RE.is_match(&code[node.start_byte()..node.end_byte()])
    }

    mk_checker!(Python, is_string, String, ConcatenatedString);
    mk_checker!(Python, is_call, Call);
}

impl Checker for JavaCode {
    mk_checker!(Java, is_comment, Comment);
    mk_checker!(Java, is_string, StringLiteral);
    mk_checker!(Java, is_call, MethodInvocation);
}

impl Checker for MozjsCode {
    mk_checker!(Mozjs, is_comment, Comment);
    mk_checker!(Mozjs, is_string, String, TemplateString);
    mk_checker!(Mozjs, is_call, CallExpression);
}

impl Checker for JavascriptCode {
    mk_checker!(Javascript, is_comment, Comment);
    mk_checker!(Javascript, is_string, String, TemplateString);
    mk_checker!(Javascript, is_call, CallExpression);
}

impl Checker for TypescriptCode {
    mk_checker!(Typescript, is_comment, Comment);
    mk_checker!(Typescript, is_string, String, TemplateString);
    mk_checker!(Typescript, is_call, CallExpression);
}

impl Checker for TsxCode {
    mk_checker!(Tsx, is_comment, Comment);
    mk_checker!(Tsx, is_string, String, TemplateString);
    mk_checker!(Tsx, is_call, CallExpression);
}

impl Checker for GoCode {
    mk_checker!(Go, is_comment, Comment);
    mk_checker!(Go, is_string, StringLiteral);
    mk_checker!(Go, is_call, CallExpression);
}

impl Checker for CssCode {
    mk_checker!(Css, is_comment, Comment);
    mk_checker!(Css, is_string, StringValue);
    mk_checker!(Css, is_call, CallExpression);
}

impl Checker for HtmlCode {
    mk_checker!(Html, is_comment, Comment);
    mk_checker!(Html, is_string,);
    mk_checker!(Html, is_call,);
}

impl Checker for RustCode {
    mk_checker!(Rust, is_comment, LineComment, BlockComment);

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        if let Some(parent) = node.parent() {
            if parent.kind_id() == Rust::TokenTree {
                // A comment could be a macro token
                return true;
            }
        }
        let code = &code[node.start_byte()..node.end_byte()];
        code.starts_with(b"/// cbindgen:")
    }

    mk_checker!(Rust, is_string, StringLiteral, RawStringLiteral);
    mk_checker!(Rust, is_call, CallExpression);
}
