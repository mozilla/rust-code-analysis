use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use tree_sitter::Node;

use crate::checker::Checker;
use crate::traits::{Callback, TSParserTrait};

type Span = Option<(usize, usize, usize, usize)>;

#[derive(Debug, Deserialize)]
pub struct AstPayload {
    pub id: String,
    pub language: String,
    pub code: String,
    pub comment: bool,
    pub span: bool,
}

#[derive(Debug, Serialize)]
pub struct AstResponse {
    id: String,
    root: Option<AstNode>,
}

#[derive(Debug)]
pub struct AstNode {
    r#type: &'static str,
    value: String,
    span: Span,
    children: Rc<RefCell<Vec<AstNode>>>,
}

impl Serialize for AstNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("Node", 4)?;
        st.serialize_field("Type", &self.r#type);
        st.serialize_field("TextValue", &self.value);
        st.serialize_field("Span", &self.span);
        st.serialize_field("Children", &self.children.replace(Vec::new()));
        st.end()
    }
}

impl AstNode {
    fn new(r#type: &'static str, value: String, span: Span) -> Self {
        Self {
            r#type,
            value,
            span,
            children: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

fn get_ast_node<T: Checker>(
    node: &Node,
    span: bool,
    code: &[u8],
    comment: bool,
) -> Option<AstNode> {
    if T::is_comment(node) {
        None
    } else {
        let span = if span {
            let spos = node.start_position();
            let epos = node.end_position();
            Some((spos.row + 1, spos.column + 1, epos.row + 1, epos.column + 1))
        } else {
            None
        };
        let text = if node.child_count() == 0 {
            String::from_utf8(code[node.start_byte()..node.end_byte()].to_vec()).unwrap()
        } else {
            "".to_string()
        };
        Some(AstNode::new(node.kind(), text, span))
    }
}

fn build<T: TSParserTrait>(parser: &T, span: bool, comment: bool) -> Option<AstNode> {
    let code = parser.get_code();
    let root = parser.get_root();
    let mut cursor = root.walk();
    let mut stack = Vec::new();
    if let Some(node) = get_ast_node::<T::Checker>(&root, span, code, comment) {
        stack.push((root, Rc::clone(&node.children)));

        while let Some((ts_node, children)) = stack.pop() {
            cursor.reset(ts_node);
            if cursor.goto_first_child() {
                let mut children = children.borrow_mut();
                loop {
                    let ts_node = cursor.node();
                    if let Some(node) = get_ast_node::<T::Checker>(&ts_node, span, code, comment) {
                        stack.push((ts_node, Rc::clone(&node.children)));
                        children.push(node);
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        }
        Some(node)
    } else {
        None
    }
}

pub struct AstCallback {}

pub struct AstCfg {
    pub id: String,
    pub comment: bool,
    pub span: bool,
}

impl Callback for AstCallback {
    type Res = AstResponse;
    type Cfg = AstCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        AstResponse {
            id: cfg.id,
            root: build(parser, cfg.span, cfg.comment),
        }
    }
}
