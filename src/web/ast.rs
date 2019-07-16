use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use tree_sitter::Node;

use crate::checker::Checker;
use crate::traits::{Callback, TSParserTrait};

type Span = Option<(usize, usize, usize, usize)>;

#[derive(Debug, Deserialize)]
pub struct AstPayload {
    pub id: String,
    pub file_name: String,
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
    children: Vec<AstNode>,
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
        st.serialize_field("Children", &self.children);
        st.end()
    }
}

impl AstNode {
    fn new(r#type: &'static str, value: String, span: Span, children: Vec<AstNode>) -> Self {
        Self {
            r#type,
            value,
            span,
            children,
        }
    }
}

fn get_ast_node<T: Checker>(
    node: &Node,
    code: &[u8],
    children: Vec<AstNode>,
    span: bool,
    comment: bool,
) -> Option<AstNode> {
    if comment && T::is_comment(node) {
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
        Some(AstNode::new(node.kind(), text, span, children))
    }
}

fn build<T: TSParserTrait>(parser: &T, span: bool, comment: bool) -> Option<AstNode> {
    let code = parser.get_code();
    let root = parser.get_root();
    let mut cursor = root.walk();
    let mut node_stack = Vec::new();
    let mut child_stack = Vec::new();

    node_stack.push(root);
    child_stack.push(Vec::new());

    /* To avoid Rc, RefCell and stuff like that (or use of unsafe)
    the idea here is to build AstNode from bottom-to-top and from left-to-right.
    So once we have built the array of children we can build the node itself until the root. */
    loop {
        let ts_node = node_stack.last().unwrap();
        cursor.reset(*ts_node);
        if cursor.goto_first_child() {
            let node = cursor.node();
            child_stack.push(Vec::with_capacity(node.child_count()));
            node_stack.push(node);
        } else {
            loop {
                let ts_node = node_stack.pop().unwrap();
                if let Some(node) = get_ast_node::<T::Checker>(
                    &ts_node,
                    code,
                    child_stack.pop().unwrap(),
                    span,
                    comment,
                ) {
                    if !child_stack.is_empty() {
                        child_stack.last_mut().unwrap().push(node);
                    } else {
                        return Some(node);
                    }
                }
                if let Some(next_node) = ts_node.next_sibling() {
                    child_stack.push(Vec::with_capacity(next_node.child_count()));
                    node_stack.push(next_node);
                    break;
                }
            }
        }
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
