use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};

use crate::*;

/// Start and end positions of a node in a code in terms of rows and columns.
///
/// The first and second fields represent the row and column associated to
/// the start position of a node.
///
/// The third and fourth fields represent the row and column associated to
/// the end position of a node.
pub type Span = Option<(usize, usize, usize, usize)>;

/// The payload of an `Ast` request.
#[derive(Debug, Deserialize, Serialize)]
pub struct AstPayload {
    /// The id associated to a request for an `AST`
    pub id: String,
    /// The filename associated to a source code file
    pub file_name: String,
    /// The code to be represented as an `AST`
    pub code: String,
    /// If `true`, nodes representing comments are ignored
    pub comment: bool,
    /// If `true`, the start and end positions of a node in a code
    /// are considered
    pub span: bool,
}

/// The response of an `AST` request.
#[derive(Debug, Serialize)]
pub struct AstResponse {
    /// The id associated to a request for an `AST`
    id: String,
    /// The root node of an `AST`
    ///
    /// If `None`, an error has occurred
    root: Option<AstNode>,
}

/// Information on an `AST` node.
#[derive(Debug)]
pub struct AstNode {
    /// The type of node
    pub r#type: &'static str,
    /// The code associated to a node
    pub value: String,
    /// The start and end positions of a node in a code
    pub span: Span,
    /// The children of a node
    pub children: Vec<AstNode>,
}

impl Serialize for AstNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("Node", 4)?;
        st.serialize_field("Type", &self.r#type)?;
        st.serialize_field("TextValue", &self.value)?;
        st.serialize_field("Span", &self.span)?;
        st.serialize_field("Children", &self.children)?;
        st.end()
    }
}

impl AstNode {
    pub fn new(r#type: &'static str, value: String, span: Span, children: Vec<AstNode>) -> Self {
        Self {
            r#type,
            value,
            span,
            children,
        }
    }
}

fn build<T: ParserTrait>(parser: &T, span: bool, comment: bool) -> Option<AstNode> {
    let code = parser.get_code();
    let root = parser.get_root();
    let mut cursor = root.object().walk();
    let mut node_stack = Vec::new();
    let mut child_stack = Vec::new();

    node_stack.push(root);
    child_stack.push(Vec::new());

    /* To avoid Rc, RefCell and stuff like that (or use of unsafe)
    the idea here is to build AstNode from bottom-to-top and from left-to-right.
    So once we have built the array of children we can build the node itself until the root. */
    loop {
        let ts_node = node_stack.last().unwrap();
        cursor.reset(ts_node.object());
        if cursor.goto_first_child() {
            let node = cursor.node();
            child_stack.push(Vec::with_capacity(node.child_count()));
            node_stack.push(Node::new(node));
        } else {
            loop {
                let ts_node = node_stack.pop().unwrap();
                if let Some(node) = T::Checker::get_ast_node(
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
                if let Some(next_node) = ts_node.object().next_sibling() {
                    child_stack.push(Vec::with_capacity(next_node.child_count()));
                    node_stack.push(Node::new(next_node));
                    break;
                }
            }
        }
    }
}

pub struct AstCallback {
    _guard: (),
}

/// Configuration options for retrieving the nodes of an `AST`.
pub struct AstCfg {
    /// The id associated to a request for an `AST`
    pub id: String,
    /// If `true`, nodes representing comments are ignored
    pub comment: bool,
    /// If `true`, the start and end positions of a node in a code
    /// are considered
    pub span: bool,
}

impl Callback for AstCallback {
    type Res = AstResponse;
    type Cfg = AstCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        AstResponse {
            id: cfg.id,
            root: build(parser, cfg.span, cfg.comment),
        }
    }
}
/* where T::Checker: Alterator*/
