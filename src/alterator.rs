use crate::*;

/// A trait to create a richer `AST` node for a programming language, mainly
/// thought to be sent on the network.
pub trait Alterator
where
    Self: Checker + LanguageInfo,
{
    /// Gets the kind of a node as a `'static` string.
    ///
    /// Since tree-sitter 0.27, `Node::kind` returns a string bound to the
    /// lifetime of the tree, so the kind is resolved through the generated
    /// language enum instead.
    fn get_kind(node: &Node) -> &'static str {
        Self::BaseLang::from(node.kind_id()).into()
    }

    /// Creates a new `AST` node containing the code associated to the node,
    /// its span, and its children.
    ///
    /// This function can be overloaded according to the needs of each
    /// programming language.
    fn alterate(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        Self::get_default(node, code, span, children)
    }

    /// Gets the code as text and the span associated to a node.
    fn get_text_span(node: &Node, code: &[u8], span: bool, text: bool) -> (String, Span) {
        let text = if text {
            String::from_utf8(code[node.start_byte()..node.end_byte()].to_vec()).unwrap()
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

    /// Gets a default `AST` node containing the code associated to the node,
    /// its span, and its children.
    fn get_default(node: &Node, code: &[u8], span: bool, children: Vec<AstNode>) -> AstNode {
        let (text, span) = Self::get_text_span(node, code, span, node.child_count() == 0);
        AstNode::new(Self::get_kind(node), text, span, children)
    }

    /// Gets a new `AST` node if and only if the code is not a comment,
    /// otherwise [`None`] is returned.
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
                AstNode::new(Self::get_kind(node), text, span, Vec::new())
            }
            Cpp::PreprocDef | Cpp::PreprocFunctionDef | Cpp::PreprocCall => {
                if let Some(last) = children.last()
                    && last.r#type == "\n"
                {
                    children.pop();
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
            Mozjs::String | Mozjs::String2 => {
                // TODO: have a thought about template_strings:
                // they may have children for replacement...
                let (text, span) = Self::get_text_span(node, code, span, true);
                AstNode::new(Self::get_kind(node), text, span, Vec::new())
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
                AstNode::new(Self::get_kind(node), text, span, Vec::new())
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
                AstNode::new(Self::get_kind(node), text, span, Vec::new())
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
                AstNode::new(Self::get_kind(node), text, span, Vec::new())
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
                AstNode::new(Self::get_kind(node), text, span, Vec::new())
            }
            _ => Self::get_default(node, code, span, children),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `Alterator::get_kind` relies on the generated language enums mapping
    // every kind id to the same name tree-sitter reports.
    fn check_kind_names<T: LanguageInfo>() {
        let lang = T::get_lang().get_ts_language();
        for id in (0..lang.node_kind_count() as u16).chain([u16::MAX]) {
            let Some(expected) = lang.node_kind_for_id(id) else {
                continue;
            };
            let got: &'static str = T::BaseLang::from(id).into();
            assert_eq!(got, expected, "{} id={id}", T::get_lang_name());
        }
    }

    #[test]
    fn kind_names_match_tree_sitter() {
        check_kind_names::<MozjsCode>();
        check_kind_names::<JavascriptCode>();
        check_kind_names::<JavaCode>();
        check_kind_names::<KotlinCode>();
        check_kind_names::<RustCode>();
        check_kind_names::<CppCode>();
        check_kind_names::<PythonCode>();
        check_kind_names::<TsxCode>();
        check_kind_names::<TypescriptCode>();
        check_kind_names::<PreprocCode>();
        check_kind_names::<CcommentCode>();
    }
}
