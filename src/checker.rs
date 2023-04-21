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

impl Checker for PreprocCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Preproc::Comment
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(_: &Node) -> bool {
        false
    }

    fn is_func(_: &Node) -> bool {
        false
    }

    fn is_closure(_: &Node) -> bool {
        false
    }

    fn is_call(_: &Node) -> bool {
        false
    }

    fn is_non_arg(_: &Node) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Preproc::StringLiteral
            || node.object().kind_id() == Preproc::RawStringLiteral
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for CcommentCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Ccomment::Comment
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            static ref AC: AhoCorasick = AhoCorasick::new(vec![b"<div rustbindgen"]).unwrap();
        }
        let code = &code[node.object().start_byte()..node.object().end_byte()];
        AC.is_match(code)
    }

    fn is_func_space(_: &Node) -> bool {
        false
    }

    fn is_func(_: &Node) -> bool {
        false
    }

    fn is_closure(_: &Node) -> bool {
        false
    }

    fn is_call(_: &Node) -> bool {
        false
    }

    fn is_non_arg(_: &Node) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Ccomment::StringLiteral
            || node.object().kind_id() == Ccomment::RawStringLiteral
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for CppCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Cpp::Comment
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            static ref AC: AhoCorasick = AhoCorasick::new(vec![b"<div rustbindgen"]).unwrap();
        }
        let code = &code[node.object().start_byte()..node.object().end_byte()];
        AC.is_match(code)
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Cpp::TranslationUnit
                | Cpp::FunctionDefinition
                | Cpp::FunctionDefinition2
                | Cpp::FunctionDefinition3
                | Cpp::StructSpecifier
                | Cpp::ClassSpecifier
                | Cpp::NamespaceDefinition
        )
    }

    fn is_func(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Cpp::FunctionDefinition
                | Cpp::FunctionDefinition2
                | Cpp::FunctionDefinition3
                | Cpp::FunctionDefinition4
        )
    }

    fn is_closure(node: &Node) -> bool {
        node.object().kind_id() == Cpp::LambdaExpression
    }

    fn is_call(node: &Node) -> bool {
        node.object().kind_id() == Cpp::CallExpression
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Cpp::LPAREN | Cpp::LPAREN2 | Cpp::COMMA | Cpp::RPAREN
        )
    }

    fn is_string(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Cpp::StringLiteral | Cpp::ConcatenatedString | Cpp::RawStringLiteral
        )
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

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Cpp::PrimitiveType
    }
}

impl Checker for PythonCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Python::Comment
    }

    fn is_useful_comment(node: &Node, code: &[u8]) -> bool {
        lazy_static! {
            // comment containing coding info are useful
            static ref RE: Regex = Regex::new(r"^[ \t\f]*#.*?coding[:=][ \t]*([-_.a-zA-Z0-9]+)").unwrap();
        }
        node.object().start_position().row <= 1
            && RE.is_match(&code[node.object().start_byte()..node.object().end_byte()])
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Python::Module | Python::FunctionDefinition | Python::ClassDefinition
        )
    }

    fn is_func(node: &Node) -> bool {
        node.object().kind_id() == Python::FunctionDefinition
    }

    fn is_closure(node: &Node) -> bool {
        node.object().kind_id() == Python::Lambda
    }

    fn is_call(node: &Node) -> bool {
        node.object().kind_id() == Python::Call
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Python::LPAREN | Python::COMMA | Python::RPAREN
        )
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Python::String
            || node.object().kind_id() == Python::ConcatenatedString
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for JavaCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Java::LineComment
            || node.object().kind_id() == Java::BlockComment
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Java::Program | Java::ClassDeclaration | Java::InterfaceDeclaration
        )
    }

    fn is_func(node: &Node) -> bool {
        node.object().kind_id() == Java::MethodDeclaration
            || node.object().kind_id() == Java::ConstructorDeclaration
    }

    fn is_closure(node: &Node) -> bool {
        node.object().kind_id() == Java::LambdaExpression
    }

    fn is_call(node: &Node) -> bool {
        node.object().kind_id() == Java::MethodInvocation
    }

    fn is_non_arg(_: &Node) -> bool {
        false
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Java::StringLiteral
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for MozjsCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Mozjs::Comment
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Mozjs::Program
                | Mozjs::Function
                | Mozjs::Class
                | Mozjs::GeneratorFunction
                | Mozjs::FunctionDeclaration
                | Mozjs::MethodDefinition
                | Mozjs::GeneratorFunctionDeclaration
                | Mozjs::ClassDeclaration
                | Mozjs::ArrowFunction
        )
    }

    is_js_func_and_closure_checker!(Mozjs);

    fn is_call(node: &Node) -> bool {
        node.object().kind_id() == Mozjs::CallExpression
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Mozjs::LPAREN | Mozjs::COMMA | Mozjs::RPAREN
        )
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Mozjs::String || node.object().kind_id() == Mozjs::TemplateString
    }

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

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for JavascriptCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Javascript::Comment
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Javascript::Program
                | Javascript::Function
                | Javascript::Class
                | Javascript::GeneratorFunction
                | Javascript::FunctionDeclaration
                | Javascript::MethodDefinition
                | Javascript::GeneratorFunctionDeclaration
                | Javascript::ClassDeclaration
                | Javascript::ArrowFunction
        )
    }

    is_js_func_and_closure_checker!(Javascript);

    fn is_call(node: &Node) -> bool {
        node.object().kind_id() == Javascript::CallExpression
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Javascript::LPAREN | Javascript::COMMA | Javascript::RPAREN
        )
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Javascript::String
            || node.object().kind_id() == Javascript::TemplateString
    }

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

    fn is_primitive(_id: u16) -> bool {
        false
    }
}

impl Checker for TypescriptCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Typescript::Comment
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Typescript::Program
                | Typescript::Function
                | Typescript::Class
                | Typescript::GeneratorFunction
                | Typescript::FunctionDeclaration
                | Typescript::MethodDefinition
                | Typescript::GeneratorFunctionDeclaration
                | Typescript::ClassDeclaration
                | Typescript::InterfaceDeclaration
                | Typescript::ArrowFunction
        )
    }

    is_js_func_and_closure_checker!(Typescript);

    fn is_call(node: &Node) -> bool {
        node.object().kind_id() == Typescript::CallExpression
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Typescript::LPAREN | Typescript::COMMA | Typescript::RPAREN
        )
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Typescript::String
            || node.object().kind_id() == Typescript::TemplateString
    }

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
}

impl Checker for TsxCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Tsx::Comment
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Tsx::Program
                | Tsx::Function
                | Tsx::Class
                | Tsx::GeneratorFunction
                | Tsx::FunctionDeclaration
                | Tsx::MethodDefinition
                | Tsx::GeneratorFunctionDeclaration
                | Tsx::ClassDeclaration
                | Tsx::InterfaceDeclaration
                | Tsx::ArrowFunction
        )
    }

    is_js_func_and_closure_checker!(Tsx);

    fn is_call(node: &Node) -> bool {
        node.object().kind_id() == Tsx::CallExpression
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Tsx::LPAREN | Tsx::COMMA | Tsx::RPAREN
        )
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Tsx::String || node.object().kind_id() == Tsx::TemplateString
    }

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

    #[inline(always)]
    fn is_primitive(id: u16) -> bool {
        id == Tsx::PredefinedType
    }
}

impl Checker for RustCode {
    fn is_comment(node: &Node) -> bool {
        node.object().kind_id() == Rust::LineComment
            || node.object().kind_id() == Rust::BlockComment
    }

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

    fn is_func_space(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Rust::SourceFile
                | Rust::FunctionItem
                | Rust::ImplItem
                | Rust::TraitItem
                | Rust::ClosureExpression
        )
    }

    fn is_func(node: &Node) -> bool {
        node.object().kind_id() == Rust::FunctionItem
    }

    fn is_closure(node: &Node) -> bool {
        node.object().kind_id() == Rust::ClosureExpression
    }

    fn is_call(node: &Node) -> bool {
        node.object().kind_id() == Rust::CallExpression
    }

    fn is_non_arg(node: &Node) -> bool {
        matches!(
            node.object().kind_id().into(),
            Rust::LPAREN | Rust::COMMA | Rust::RPAREN | Rust::PIPE | Rust::AttributeItem
        )
    }

    fn is_string(node: &Node) -> bool {
        node.object().kind_id() == Rust::StringLiteral
            || node.object().kind_id() == Rust::RawStringLiteral
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
}

impl Checker for KotlinCode {
    fn is_comment(_: &Node) -> bool {
        false
    }

    fn is_useful_comment(_: &Node, _: &[u8]) -> bool {
        false
    }

    fn is_func_space(_: &Node) -> bool {
        false
    }

    fn is_func(_: &Node) -> bool {
        false
    }

    fn is_closure(_: &Node) -> bool {
        false
    }

    fn is_call(_: &Node) -> bool {
        false
    }

    fn is_non_arg(_: &Node) -> bool {
        false
    }

    fn is_string(_: &Node) -> bool {
        false
    }

    fn is_else_if(_: &Node) -> bool {
        false
    }

    fn is_primitive(_id: u16) -> bool {
        false
    }
}
