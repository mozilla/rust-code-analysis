use tree_sitter::Node as OtherNode;
use tree_sitter::{Tree, TreeCursor};

use crate::traits::Search;

/// An `AST` node.
#[derive(Clone, Copy)]
pub struct Node<'a>(OtherNode<'a>);

impl<'a> Node<'a> {
    /// Checks if a node represents a syntax error or contains any syntax errors
    /// anywhere within it.
    pub fn has_error(&self) -> bool {
        self.0.has_error()
    }

    pub(crate) fn get_tree_root(tree: &'a Tree) -> Self {
        Self(tree.root_node())
    }

    pub(crate) fn new(node: OtherNode<'a>) -> Self {
        Node(node)
    }

    pub(crate) fn parent(&self) -> Option<Node<'a>> {
        self.0.parent().map(|p| Node::new(p))
    }

    pub(crate) fn id(&self) -> usize {
        self.0.id()
    }

    pub(crate) fn kind(&self) -> &'static str {
        self.0.kind()
    }

    pub(crate) fn utf8_text(&self, data: &'a [u8]) -> Option<&'a str> {
        self.0.utf8_text(data).ok()
    }

    pub(crate) fn kind_id(&self) -> u16 {
        self.0.kind_id()
    }

    pub(crate) fn child_by_field_name(&self, name: &str) -> Option<Node> {
        self.0.child_by_field_name(name).map(|n| Node::new(n))
    }

    pub(crate) fn child(&self, pos: usize) -> Option<Node<'a>> {
        self.0.child(pos).map(|c| Node::new(c))
    }

    pub(crate) fn start_byte(&self) -> usize {
        self.0.start_byte()
    }

    pub(crate) fn end_byte(&self) -> usize {
        self.0.end_byte()
    }

    pub(crate) fn start_position(&self) -> (usize, usize) {
        let temp = self.0.start_position();
        (temp.row, temp.column)
    }

    pub(crate) fn end_position(&self) -> (usize, usize) {
        let temp = self.0.end_position();
        (temp.row, temp.column)
    }

    pub(crate) fn start_row(&self) -> usize {
        self.0.start_position().row
    }

    pub(crate) fn end_row(&self) -> usize {
        self.0.end_position().row
    }

    pub(crate) fn child_count(&self) -> usize {
        self.0.child_count()
    }

    pub(crate) fn previous_sibling(&self) -> Option<Node<'a>> {
        self.0.prev_sibling().map(|s| Node::new(s))
    }

    pub(crate) fn next_sibling(&self) -> Option<Node<'a>> {
        self.0.next_sibling().map(|s| Node::new(s))
    }

    pub(crate) fn cursor(&self) -> Cursor<'a> {
        Cursor(self.0.walk())
    }

    #[inline(always)]
    pub(crate) fn is_child(&self, id: u16) -> bool {
        self.0
            .children(&mut self.0.walk())
            .any(|child| child.kind_id() == id)
    }

    #[inline(always)]
    pub(crate) fn has_sibling(&self, id: u16) -> bool {
        self.0.parent().map_or(false, |parent| {
            self.0
                .children(&mut parent.walk())
                .any(|child| child.kind_id() == id)
        })
    }

    pub(crate) fn children(&self) -> impl ExactSizeIterator<Item = Node<'a>> {
        let mut cursor = self.cursor();
        cursor.goto_first_child();
        (0..self.child_count()).map(move |_| {
            let result = cursor.node();
            cursor.goto_next_sibling();
            result
        })
    }
}

/// An `AST` cursor.
#[derive(Clone)]
pub struct Cursor<'a>(TreeCursor<'a>);

impl<'a> Cursor<'a> {
    pub(crate) fn reset(&mut self, node: &Node<'a>) {
        self.0.reset(node.0);
    }

    pub(crate) fn goto_first_child(&mut self) -> bool {
        self.0.goto_first_child()
    }

    pub(crate) fn goto_next_sibling(&mut self) -> bool {
        self.0.goto_next_sibling()
    }

    pub(crate) fn node(&self) -> Node<'a> {
        Node::new(self.0.node())
    }
}

impl<'a> Search<'a> for Node<'a> {
    fn first_occurence(&self, pred: fn(u16) -> bool) -> Option<Node<'a>> {
        let mut cursor = self.cursor();
        let mut stack = Vec::new();
        let mut children = Vec::new();

        stack.push(*self);

        while let Some(node) = stack.pop() {
            if pred(node.kind_id()) {
                return Some(node);
            }
            cursor.reset(&node);
            if cursor.goto_first_child() {
                loop {
                    children.push(cursor.node());
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                for child in children.drain(..).rev() {
                    stack.push(child);
                }
            }
        }

        None
    }

    fn act_on_node(&self, action: &mut dyn FnMut(&Node<'a>)) {
        let mut cursor = self.cursor();
        let mut stack = Vec::new();
        let mut children = Vec::new();

        stack.push(*self);

        while let Some(node) = stack.pop() {
            action(&node);
            cursor.reset(&node);
            if cursor.goto_first_child() {
                loop {
                    children.push(cursor.node());
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
                for child in children.drain(..).rev() {
                    stack.push(child);
                }
            }
        }
    }

    fn first_child(&self, pred: fn(u16) -> bool) -> Option<Node<'a>> {
        for child in self.children() {
            if pred(child.kind_id()) {
                return Some(child);
            }
        }
        None
    }

    fn act_on_child(&self, action: &mut dyn FnMut(&Node<'a>)) {
        for child in self.children() {
            action(&child);
        }
    }
}
