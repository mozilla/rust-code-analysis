use tree_sitter::Node;

use crate::enums::NodeKind;
use crate::traits::Search;

use crate::*;

pub trait Getter {
    fn get_func_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        Self::get_func_space_name(node, code)
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        // we're in a function or in a class
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            Some("<anonymous>")
        }
    }

    fn get_kind(_node: &Node) -> NodeKind {
        NodeKind::Unknown
    }
}

impl Getter for PythonCode {
    fn get_kind(node: &Node) -> NodeKind {
        let typ = node.kind_id();
        match typ.into() {
            Python::FunctionDefinition => NodeKind::Function,
            Python::ClassDefinition => NodeKind::Class,
            Python::Module => NodeKind::Unit,
            _ => NodeKind::Unknown,
        }
    }
}

impl Getter for MozjsCode {
    fn get_kind(node: &Node) -> NodeKind {
        use Mozjs::*;

        let typ = node.kind_id();
        match typ.into() {
            Function
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => NodeKind::Function,
            Class | ClassDeclaration => NodeKind::Class,
            Program => NodeKind::Unit,
            _ => NodeKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }
}

impl Getter for JavascriptCode {
    fn get_kind(node: &Node) -> NodeKind {
        use Javascript::*;

        let typ = node.kind_id();
        match typ.into() {
            Function
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => NodeKind::Function,
            Class | ClassDeclaration => NodeKind::Class,
            Program => NodeKind::Unit,
            _ => NodeKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }
}

impl Getter for TypescriptCode {
    fn get_kind(node: &Node) -> NodeKind {
        use Typescript::*;

        let typ = node.kind_id();
        match typ.into() {
            Function
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => NodeKind::Function,
            Class | ClassDeclaration => NodeKind::Class,
            Program => NodeKind::Unit,
            _ => NodeKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }
}

impl Getter for TsxCode {
    fn get_kind(node: &Node) -> NodeKind {
        use Tsx::*;

        let typ = node.kind_id();
        match typ.into() {
            Function
            | MethodDefinition
            | GeneratorFunction
            | FunctionDeclaration
            | GeneratorFunctionDeclaration
            | ArrowFunction => NodeKind::Function,
            Class | ClassDeclaration => NodeKind::Class,
            Program => NodeKind::Unit,
            _ => NodeKind::Unknown,
        }
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            // We can be in a pair: foo: function() {}
            // Or in a variable declaration: var aFun = function() {}
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    Mozjs::Pair => {
                        if let Some(name) = parent.child_by_field_name("key") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    Mozjs::VariableDeclarator => {
                        if let Some(name) = parent.child_by_field_name("name") {
                            let code = &code[name.start_byte()..name.end_byte()];
                            return std::str::from_utf8(code).ok();
                        }
                    }
                    _ => {}
                }
            }
            Some("<anonymous>")
        }
    }
}

impl Getter for RustCode {
    fn get_kind(node: &Node) -> NodeKind {
        use Rust::*;

        let typ = node.kind_id();
        match typ.into() {
            FunctionItem | ClosureExpression => NodeKind::Function,
            TraitItem => NodeKind::Trait,
            ImplItem => NodeKind::Impl,
            SourceFile => NodeKind::Unit,
            _ => NodeKind::Unknown,
        }
    }
}

impl Getter for CppCode {
    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        let typ = node.kind_id();
        match typ.into() {
            Cpp::FunctionDefinition | Cpp::FunctionDefinition2 | Cpp::FunctionDefinition3 => {
                // we're in a function_definition so need to get the declarator
                if let Some(declarator) = node.child_by_field_name("declarator") {
                    if let Some(fd) = declarator.first_occurence(|id| {
                        Cpp::FunctionDeclarator == id
                            || Cpp::FunctionDeclarator2 == id
                            || Cpp::FunctionDeclarator3 == id
                    }) {
                        if let Some(first) = fd.child(0) {
                            match first.kind_id().into() {
                                Cpp::ScopedIdentifier
                                | Cpp::Identifier
                                | Cpp::FieldIdentifier
                                | Cpp::ScopedFieldIdentifier
                                | Cpp::DestructorName
                                | Cpp::OperatorName
                                | Cpp::TemplateFunction
                                | Cpp::TemplateMethod => {
                                    let code = &code[first.start_byte()..first.end_byte()];
                                    return std::str::from_utf8(code).ok();
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {
                if let Some(name) = node.child_by_field_name("name") {
                    let code = &code[name.start_byte()..name.end_byte()];
                    return std::str::from_utf8(code).ok();
                }
            }
        }
        None
    }

    fn get_kind(node: &Node) -> NodeKind {
        use Cpp::*;

        let typ = node.kind_id();
        match typ.into() {
            FunctionDefinition | FunctionDefinition2 | FunctionDefinition3 => NodeKind::Function,
            StructSpecifier => NodeKind::Struct,
            ClassSpecifier => NodeKind::Class,
            NamespaceDefinition => NodeKind::Namespace,
            TranslationUnit => NodeKind::Unit,
            _ => NodeKind::Unknown,
        }
    }
}

impl Getter for PreprocCode {}
impl Getter for CcommentCode {}
impl Getter for CSharpCode {}
impl Getter for JavaCode {}
impl Getter for GoCode {}
impl Getter for CssCode {}
impl Getter for HtmlCode {}
