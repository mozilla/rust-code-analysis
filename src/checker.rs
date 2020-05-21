use aho_corasick::AhoCorasick;
use regex::bytes::Regex;

use crate::*;

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
    mk_checker!(is_non_arg, LPAREN, LPAREN2, COMMA, RPAREN);
}

impl Checker for CSharpCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, StringLiteral);
    mk_checker!(is_call, InvocationExpression);
    mk_checker!(
        is_func,
        MethodDeclaration,
        ConstructorDeclaration,
        ConversionOperatorDeclaration,
        DestructorDeclaration,
        OperatorDeclaration,
        AccessorDeclaration,
        LocalFunctionStatement
    );
    mk_checker!(
        is_func_space,
        CompilationUnit,
        ClassDeclaration,
        StructDeclaration,
        NamespaceDeclaration
    );
    mk_checker!(is_non_arg,);
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
}

impl Checker for JavaCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, StringLiteral);
    mk_checker!(is_call, MethodInvocation);
    mk_checker!(is_func, MethodDeclaration);
    mk_checker!(is_func_space, Program, ClassDeclaration);
    mk_checker!(is_non_arg,);
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
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for GoCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, RawStringLiteral, InterpretedStringLiteral);
    mk_checker!(is_call, CallExpression);
    mk_checker!(is_func, FunctionDeclaration, MethodDeclaration, FuncLiteral);
    mk_checker!(
        is_func_space,
        SourceFile,
        FunctionDeclaration,
        MethodDeclaration,
        FuncLiteral
    );
    mk_checker!(is_non_arg,);
}

impl Checker for CssCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, StringValue);
    mk_checker!(is_call, CallExpression);
    mk_checker!(is_func,);
    mk_checker!(is_func_space,);
    mk_checker!(is_non_arg,);
}

impl Checker for HtmlCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string,);
    mk_checker!(is_call,);
    mk_checker!(is_func,);
    mk_checker!(is_func_space,);
    mk_checker!(is_non_arg,);
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
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN, AttributeItem);
}
