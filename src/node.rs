use tree_sitter::Node as OtherNode;
use tree_sitter::Tree as OtherTree;
use tree_sitter::{Parser, TreeCursor};

use crate::checker::Checker;
use crate::traits::{LanguageInfo, Search};

#[derive(Clone, Debug)]
pub(crate) struct Tree(OtherTree);

impl Tree {
    pub(crate) fn new<T: LanguageInfo>(code: &[u8]) -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&T::get_lang().get_ts_language())
            .unwrap();

        Self(parser.parse(code, None).unwrap())
    }

    pub(crate) fn get_root(&self) -> Node<'_> {
        Node(self.0.root_node())
    }
}

/// An `AST` node.
///
/// The inner `tree_sitter::Node` is exposed for advanced use cases
/// where direct access to the underlying tree-sitter API is needed.
#[derive(Clone, Copy, Debug)]
pub struct Node<'a>(pub OtherNode<'a>);

impl<'a> Node<'a> {
    /// Checks if a node represents a syntax error or contains any syntax errors
    /// anywhere within it.
    pub fn has_error(&self) -> bool {
        self.0.has_error()
    }

    /// Returns a numeric id for this node that is unique within its tree.
    pub fn id(&self) -> usize {
        self.0.id()
    }

    /// Returns the node's type as a string.
    pub fn kind(&self) -> &'static str {
        self.0.kind()
    }

    /// Returns the node's type as a numeric id.
    pub fn kind_id(&self) -> u16 {
        self.0.kind_id()
    }

    /// Returns the node's text as a UTF-8 string, if valid.
    pub fn utf8_text(&self, data: &'a [u8]) -> Option<&'a str> {
        self.0.utf8_text(data).ok()
    }

    /// Returns the byte offset where this node starts.
    pub fn start_byte(&self) -> usize {
        self.0.start_byte()
    }

    /// Returns the byte offset where this node ends.
    pub fn end_byte(&self) -> usize {
        self.0.end_byte()
    }

    /// Returns the (row, column) position where this node starts.
    pub fn start_position(&self) -> (usize, usize) {
        let temp = self.0.start_position();
        (temp.row, temp.column)
    }

    /// Returns the (row, column) position where this node ends.
    pub fn end_position(&self) -> (usize, usize) {
        let temp = self.0.end_position();
        (temp.row, temp.column)
    }

    /// Returns the row number where this node starts.
    pub fn start_row(&self) -> usize {
        self.0.start_position().row
    }

    /// Returns the row number where this node ends.
    pub fn end_row(&self) -> usize {
        self.0.end_position().row
    }

    /// Returns this node's parent, if any.
    pub fn parent(&self) -> Option<Node<'a>> {
        self.0.parent().map(Node)
    }

    #[inline(always)]
    pub(crate) fn has_sibling(&self, id: u16) -> bool {
        self.0.parent().is_some_and(|parent| {
            self.0
                .children(&mut parent.walk())
                .any(|child| child.kind_id() == id)
        })
    }

    pub(crate) fn previous_sibling(&self) -> Option<Node<'a>> {
        self.0.prev_sibling().map(Node)
    }

    pub(crate) fn next_sibling(&self) -> Option<Node<'a>> {
        self.0.next_sibling().map(Node)
    }

    #[inline(always)]
    pub(crate) fn is_child(&self, id: u16) -> bool {
        self.0
            .children(&mut self.0.walk())
            .any(|child| child.kind_id() == id)
    }

    pub(crate) fn child_count(&self) -> usize {
        self.0.child_count()
    }

    pub(crate) fn child_by_field_name(&self, name: &str) -> Option<Node<'_>> {
        self.0.child_by_field_name(name).map(Node)
    }

    pub(crate) fn child(&self, pos: usize) -> Option<Node<'a>> {
        self.0.child(pos as u32).map(Node)
    }

    pub(crate) fn children(&self) -> impl ExactSizeIterator<Item = Node<'a>> + use<'a> {
        let mut cursor = self.cursor();
        cursor.goto_first_child();
        (0..self.child_count()).map(move |_| {
            let result = cursor.node();
            cursor.goto_next_sibling();
            result
        })
    }

    pub(crate) fn cursor(&self) -> Cursor<'a> {
        Cursor(self.0.walk())
    }

    #[allow(dead_code)]
    pub(crate) fn get_parent(&self, level: usize) -> Option<Node<'a>> {
        let mut level = level;
        let mut node = *self;
        while level != 0 {
            if let Some(parent) = node.parent() {
                node = parent;
            } else {
                return None;
            }
            level -= 1;
        }

        Some(node)
    }

    pub(crate) fn count_specific_ancestors<T: crate::ParserTrait>(
        &self,
        check: fn(&Node) -> bool,
        stop: fn(&Node) -> bool,
    ) -> usize {
        let mut count = 0;
        let mut node = *self;
        while let Some(parent) = node.parent() {
            if stop(&parent) {
                break;
            }
            if check(&parent) && !T::Checker::is_else_if(&parent) {
                count += 1;
            }
            node = parent;
        }
        count
    }

    pub(crate) fn has_ancestors(&self, typ: fn(&Node) -> bool, typs: fn(&Node) -> bool) -> bool {
        let mut res = false;
        let mut node = *self;
        if let Some(parent) = node.parent()
            && typ(&parent)
        {
            node = parent;
        }
        if let Some(parent) = node.parent()
            && typs(&parent)
        {
            res = true;
        }
        res
    }

    /// Checks if this node has any ancestor that meets the given predicate.
    ///
    /// Traverses up the tree from this node's parent to the root,
    /// returning true if any ancestor satisfies the predicate.
    pub fn has_ancestor<F: Fn(&Node) -> bool>(&self, pred: F) -> bool {
        let mut node = *self;
        while let Some(parent) = node.parent() {
            if pred(&parent) {
                return true;
            }
            node = parent;
        }
        false
    }

    // Traverse a tree passing from children to children in search of a specific
    // token or series of tokens
    pub(crate) fn traverse_children<F>(&self, token_list: &[F]) -> Option<Node<'a>>
    where
        F: FnOnce(u16) -> bool + Copy,
    {
        let mut node = *self;
        'outer: for token in token_list {
            for temp_node in node.children() {
                if token(temp_node.kind_id()) {
                    node = temp_node;
                    continue 'outer;
                }
            }
            // If a token has not been found, return None
            return None;
        }
        Some(node)
    }
}

/// An `AST` cursor.
#[derive(Clone)]
pub struct Cursor<'a>(TreeCursor<'a>);

impl<'a> Cursor<'a> {
    pub(crate) fn reset(&mut self, node: &Node<'a>) {
        self.0.reset(node.0);
    }

    pub(crate) fn goto_next_sibling(&mut self) -> bool {
        self.0.goto_next_sibling()
    }

    pub(crate) fn goto_first_child(&mut self) -> bool {
        self.0.goto_first_child()
    }

    pub(crate) fn node(&self) -> Node<'a> {
        Node(self.0.node())
    }
}

impl<'a> Search<'a> for Node<'a> {
    fn first_occurrence(&self, pred: fn(u16) -> bool) -> Option<Node<'a>> {
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

    fn all_occurrences(&self, pred: fn(u16) -> bool) -> Vec<Node<'a>> {
        let mut cursor = self.cursor();
        let mut stack = Vec::new();
        let mut children = Vec::new();
        let mut results = Vec::new();

        stack.push(*self);

        while let Some(node) = stack.pop() {
            if pred(node.kind_id()) {
                results.push(node);
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

        results
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
        self.children().find(|&child| pred(child.kind_id()))
    }

    fn act_on_child(&self, action: &mut dyn FnMut(&Node<'a>)) {
        for child in self.children() {
            action(&child);
        }
    }
}
