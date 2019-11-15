use tree_sitter::Node;

use crate::enums::NodeKind;
use crate::*;

pub trait Getter {
    fn get_func_name<'a>(_node: &Node, _code: &'a [u8]) -> Option<&'a str> {
        None
    }

    fn get_func_space_name<'a>(_node: &Node, _code: &'a [u8]) -> Option<&'a str> {
        None
    }

    fn get_kind(_node: &Node) -> NodeKind {
        NodeKind::Unknown
    }
}

impl Getter for PythonCode {
    fn get_func_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        Self::get_func_space_name(node, code)
    }

    fn get_func_space_name<'a>(node: &Node, code: &'a [u8]) -> Option<&'a str> {
        // we're in a function or in a class
        if let Some(name) = node.child_by_field_name("name") {
            let code = &code[name.start_byte()..name.end_byte()];
            std::str::from_utf8(code).ok()
        } else {
            None
        }
    }

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

impl Getter for PreprocCode {}
impl Getter for CcommentCode {}
impl Getter for CCode {}
impl Getter for CppCode {}
impl Getter for CSharpCode {}
impl Getter for JavaCode {}
impl Getter for MozjsCode {}
impl Getter for JavascriptCode {}
impl Getter for TypescriptCode {}
impl Getter for TsxCode {}
impl Getter for GoCode {}
impl Getter for CssCode {}
impl Getter for HtmlCode {}
impl Getter for RustCode {}
