use aho_corasick::AhoCorasick;
use lazy_static::lazy_static;
use regex::bytes::Regex;

use crate::*;

#[inline(always)]
fn is_child(node: &Node, id: u16) -> bool {
    node.object()
        .children(&mut node.object().walk())
        .any(|child| child.kind_id() == id)
}

#[inline(always)]
fn has_sibling(node: &Node, id: u16) -> bool {
    node.object().parent().map_or(false, |parent| {
        node.object()
            .children(&mut parent.walk())
            .any(|child| child.kind_id() == id)
    })
}

macro_rules! check_if_func {
    ($node: ident) => {
        count_specific_ancestors!(
            $node,
            VariableDeclarator | AssignmentExpression | LabeledStatement | Pair,
            StatementBlock | ReturnStatement | NewExpression | Arguments
        ) > 0
            || is_child($node, Identifier as u16)
    };
}

macro_rules! check_if_arrow_func {
    ($node: ident) => {
        count_specific_ancestors!(
            $node,
            VariableDeclarator | AssignmentExpression | LabeledStatement,
            StatementBlock | ReturnStatement | NewExpression | CallExpression
        ) > 0
            || has_sibling($node, PropertyIdentifier as u16)
    };
}

macro_rules! is_js_func {
    ($node: ident) => {
        match $node.object().kind_id().into() {
            FunctionDeclaration | MethodDefinition => true,
            Function => check_if_func!($node),
            ArrowFunction => check_if_arrow_func!($node),
            _ => false,
        }
    };
}

macro_rules! is_js_closure {
    ($node: ident) => {
        match $node.object().kind_id().into() {
            GeneratorFunction | GeneratorFunctionDeclaration => true,
            Function => !check_if_func!($node),
            ArrowFunction => !check_if_arrow_func!($node),
            _ => false,
        }
    };
}

macro_rules! is_js_func_and_closure_checker {
    ($grammar: ident) => {
        #[inline(always)]
        fn is_func(node: &Node) -> bool {
            use $grammar::*;
            is_js_func!(node)
        }

        #[inline(always)]
        fn is_closure(node: &Node) -> bool {
            use $grammar::*;
            is_js_closure!(node)
        }
    };
}

pub trait Checker {
    fn is_comment(_: &Node) -> bool;
    fn is_useful_comment(_: &Node, _: &[u8]) -> bool;
    fn is_func_space(_: &Node) -> bool;
    fn is_func(_: &Node) -> bool;
    fn is_closure(_: &Node) -> bool;
    fn is_call(_: &Node) -> bool;
    fn is_non_arg(_: &Node) -> bool;
    fn is_string(_: &Node) -> bool;
    fn is_else_if(_: &Node) -> bool;
    fn is_primitive(_id: u16) -> bool;

    fn is_error(node: &Node) -> bool {
        node.object().is_error()
    }
}

    #[inline(always)]
    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    #[inline(always)]
    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool;
    fn is_call(node: &Node) -> bool;
    fn is_func(node: &Node) -> bool;
    fn is_closure(node: &Node) -> bool;
    fn is_func_space(node: &Node) -> bool;
    fn is_non_arg(node: &Node) -> bool;

    #[inline(always)]
    fn is_primitive(_id: u16) -> bool {
        false
    }

    fn is_error(node: &Node) -> bool {
        node.object().is_error()
    }
}

impl Checker for PreprocCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, StringLiteral, RawStringLiteral);
    mk_checker!(is_call,);
    mk_checker!(is_func,);
    mk_checker!(is_closure,);
    mk_checker!(is_func_space,);
    mk_checker!(is_non_arg,);
}

impl Checker for CcommentCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, StringLiteral, RawStringLiteral);
    mk_checker!(is_call,);
    mk_checker!(is_func,);
    mk_checker!(is_closure,);
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
        FunctionDefinition3,
        FunctionDefinition4
    );
    mk_checker!(is_closure, LambdaExpression);
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

    fn is_else_if(node: &Node) -> bool {
        if node.object().kind_id() != Cpp::IfStatement {
            return false;
        }
        if let Some(parent) = node.object().parent() {
            return node.object().kind_id() == Cpp::IfStatement
                && parent.kind_id() == Cpp::IfStatement;
        }
        false
    }

    mk_checker!(is_non_arg, LPAREN, LPAREN2, COMMA, RPAREN);

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Cpp::PrimitiveType
    }
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
    mk_checker!(is_closure, Lambda);
    mk_checker!(is_func_space, Module, FunctionDefinition, ClassDefinition);
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for JavaCode {
    mk_checker!(is_comment, LineComment, BlockComment);
    mk_checker!(is_string, StringLiteral);
    mk_checker!(is_call, MethodInvocation);
    mk_checker!(is_func, MethodDeclaration, ConstructorDeclaration);
    mk_checker!(is_closure, LambdaExpression);
    mk_checker!(
        is_func_space,
        Program,
        ClassDeclaration,
        InterfaceDeclaration
    );
    mk_checker!(is_non_arg,);
}

impl Checker for KotlinCode {
    mk_checker!(is_comment,);
    mk_checker!(is_string,);
    mk_checker!(is_call,);
    mk_checker!(is_func,);
    mk_checker!(is_closure,);
    mk_checker!(is_func_space,);
    mk_checker!(is_non_arg,);
}

impl Checker for MozjsCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, String, TemplateString);
    mk_checker!(is_call, CallExpression);

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

    is_js_func_and_closure_checker!(Mozjs);

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.object().kind_id() != Mozjs::IfStatement {
            return false;
        }
        if let Some(parent) = node.object().parent() {
            return parent.kind_id() == Mozjs::ElseClause;
        }
        false
    }
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for JavascriptCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, String, TemplateString);
    mk_checker!(is_call, CallExpression);
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

    is_js_func_and_closure_checker!(Javascript);

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.object().kind_id() != Javascript::IfStatement {
            return false;
        }
        if let Some(parent) = node.object().parent() {
            return node.object().kind_id() == Javascript::IfStatement
                && parent.kind_id() == Javascript::IfStatement;
        }
        false
    }
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for TypescriptCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, String, TemplateString);
    mk_checker!(is_call, CallExpression);
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
        InterfaceDeclaration,
        ArrowFunction
    );

    is_js_func_and_closure_checker!(Typescript);

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.object().kind_id() != Typescript::IfStatement {
            return false;
        }
        if let Some(parent) = node.object().parent() {
            return parent.kind_id() == Typescript::ElseClause;
        }
        false
    }
    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Typescript::PredefinedType
    }
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);
}

impl Checker for TsxCode {
    mk_checker!(is_comment, Comment);
    mk_checker!(is_string, String, TemplateString);
    mk_checker!(is_call, CallExpression);
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
        InterfaceDeclaration,
        ArrowFunction
    );

    is_js_func_and_closure_checker!(Tsx);

    fn is_else_if(node: &Node) -> bool {
        if node.object().kind_id() != Tsx::IfStatement {
            return false;
        }
        if let Some(parent) = node.object().parent() {
            return node.object().kind_id() == Tsx::IfStatement
                && parent.kind_id() == Tsx::IfStatement;
        }
        false
    }
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN);

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Tsx::PredefinedType
    }
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

    #[inline(always)]
    fn is_else_if(node: &Node) -> bool {
        if node.object().kind_id() != Rust::IfExpression {
            return false;
        }
        if let Some(parent) = node.object().parent() {
            return parent.kind_id() == Rust::ElseClause;
        }
        false
    }

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Rust::PrimitiveType
    }
    mk_checker!(is_string, StringLiteral, RawStringLiteral);
    mk_checker!(is_call, CallExpression);
    mk_checker!(is_func, FunctionItem);
    mk_checker!(is_closure, ClosureExpression);
    mk_checker!(
        is_func_space,
        SourceFile,
        FunctionItem,
        ImplItem,
        TraitItem,
        ClosureExpression
    );
    mk_checker!(is_non_arg, LPAREN, COMMA, RPAREN, PIPE, AttributeItem);
}
