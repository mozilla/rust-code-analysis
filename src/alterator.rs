use crate::*;

pub trait Alterator
where
    Self: Checker,
{
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        Self::get_default(node, code, span, children)
    }

    fn get_text_span(node: &Node, code: &[u8], span: bool, text: bool) -> (String, Span) {
        let text = if text {
            String::from_utf8(code[node.object().start_byte()..node.object().end_byte()].to_vec())
                .unwrap()
        } else {
            "".to_string()
        };
        if span {
            let (spos_row, spos_column) = node.start_position();
            let (epos_row, epos_column) = node.end_position();
            (
                text,
                Some((spos_row + 1, spos_column + 1, epos_row + 1, epos_column + 1)),
            )
        } else {
            (text, None)
        }
    }

    fn get_default(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        let (text, span) = Self::get_text_span(node, code, span, node.child_count() == 0);
        AstNode::new(node.kind(), text, span, children)
    }

    fn get_ast_node(
        node: &Node,
        code: &[u8],
        children: Vec<AstNode>,
        span: bool,
        comment: bool,
    ) -> Option<AstNode> {
        if comment && Self::is_comment(node) {
            None
        } else {
            Some(Self::alterate(node, code, span, children))
        }
    }
}

impl Alterator for PreprocCode {}

impl Alterator for CcommentCode {}

impl Alterator for CppCode {
    fn alterate(node: &Node, code: &[u8], span: bool, mut children: Vec<AstNode>) -> AstNode {
        match Cpp::from(node.kind_id()) {
            Cpp::StringLiteral | Cpp::CharLiteral => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            Cpp::PreprocDef | Cpp::PreprocFunctionDef | Cpp::PreprocCall => {
                if let Some(last) = children.last() {
                    if last.r#type == "\n" {
                        children.pop();
                    }
                }
                Self::get_default(node, code, span, children)
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for PythonCode {}

impl Alterator for JavaCode {}
impl Alterator for KotlinCode {}

impl Alterator for MozjsCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Mozjs::from(node.kind_id()) {
            Mozjs::String => {
                // TODO: have a thought about template_strings:
                // they may have children for replacement...
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for JavascriptCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Javascript::from(node.kind_id()) {
            Javascript::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for TypescriptCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Typescript::from(node.kind_id()) {
            Typescript::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for TsxCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Tsx::from(node.kind_id()) {
            Tsx::String => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

impl Alterator for RustCode {
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        match Rust::from(node.kind_id()) {
            Rust::StringLiteral | Rust::CharLiteral => {
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(node.kind(), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}
