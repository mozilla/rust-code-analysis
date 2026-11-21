use std::collections::HashMap;
use std::fmt;

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
        Node(self.0.root_node(), None)
    }
}

/// The parent of each node of a tree.
///
/// tree-sitter doesn't store parent pointers: `tree_sitter::Node::parent`
/// walks down from the root of the tree, so it costs O(depth). Walking up
/// through all the ancestors of a node then costs O(depth²), which takes
/// hours on very deep trees (e.g. generated files). Nodes created from a
/// root with [`Node::with_parents`] look their parent up here instead.
pub(crate) struct Parents<'a>(HashMap<usize, OtherNode<'a>>);

impl<'a> Parents<'a> {
    /// Computes the parent of each node of the tree rooted at `root`.
    pub(crate) fn new(root: &Node<'a>) -> Self {
        let mut parents = HashMap::with_capacity(root.0.descendant_count());
        let mut ancestors: Vec<OtherNode<'a>> = Vec::new();
        let mut cursor = root.0.walk();
        'walk: loop {
            let node = cursor.node();
            if let Some(&parent) = ancestors.last() {
                parents.insert(node.id(), parent);
            }
            if cursor.goto_first_child() {
                ancestors.push(node);
                continue;
            }
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    break 'walk;
                }
                ancestors.pop();
            }
        }
        Self(parents)
    }
}

impl fmt::Debug for Parents<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Parents")
            .field("len", &self.0.len())
            .finish_non_exhaustive()
    }
}

/// An `AST` node.
///
/// The inner `tree_sitter::Node` is exposed for advanced use cases
/// where direct access to the underlying tree-sitter API is needed.
#[derive(Clone, Copy)]
pub struct Node<'a>(pub OtherNode<'a>, Option<&'a Parents<'a>>);

impl fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Node").field(&self.0).finish()
    }
}

impl<'a> From<OtherNode<'a>> for Node<'a> {
    fn from(node: OtherNode<'a>) -> Self {
        Self(node, None)
    }
}

impl<'a> Node<'a> {
    /// Returns this node, using `parents` to look up the parent of this
    /// node and of all the nodes reached from it.
    pub(crate) fn with_parents<'b>(&self, parents: &'b Parents<'a>) -> Node<'b>
    where
        'a: 'b,
    {
        Node(self.0, Some(parents))
    }

    /// Wraps another node of the same tree.
    fn wrap(&self, node: OtherNode<'a>) -> Node<'a> {
        Node(node, self.1)
    }

    /// Checks if a node represents a syntax error or contains any syntax errors
    /// anywhere within it.
    pub fn has_error(&self) -> bool {
        self.0.has_error()
    }

    pub(crate) fn id(&self) -> usize {
        self.0.id()
    }

    pub(crate) fn kind(&self) -> &'static str {
        self.0.kind()
    }

    pub(crate) fn kind_id(&self) -> u16 {
        self.0.kind_id()
    }

    pub(crate) fn utf8_text(&self, data: &'a [u8]) -> Option<&'a str> {
        self.0.utf8_text(data).ok()
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

    pub(crate) fn parent(&self) -> Option<Node<'a>> {
        // The map only contains the nodes below the root it was computed
        // from, which may be a subtree: fall back to tree-sitter for the
        // others. For the root of the whole tree, this returns immediately.
        self.1
            .and_then(|parents| parents.0.get(&self.0.id()).copied())
            .or_else(|| self.0.parent())
            .map(|parent| self.wrap(parent))
    }

    #[inline(always)]
    pub(crate) fn has_sibling(&self, id: u16) -> bool {
        self.parent().is_some_and(|parent| {
            self.0
                .children(&mut parent.0.walk())
                .any(|child| child.kind_id() == id)
        })
    }

    pub(crate) fn previous_sibling(&self) -> Option<Node<'a>> {
        self.0.prev_sibling().map(|node| self.wrap(node))
    }

    pub(crate) fn next_sibling(&self) -> Option<Node<'a>> {
        self.0.next_sibling().map(|node| self.wrap(node))
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

    pub(crate) fn child_by_field_name(&self, name: &str) -> Option<Node<'a>> {
        self.0.child_by_field_name(name).map(|node| self.wrap(node))
    }

    pub(crate) fn child(&self, pos: usize) -> Option<Node<'a>> {
        self.0.child(pos as u32).map(|node| self.wrap(node))
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
        Cursor(self.0.walk(), self.1)
    }

    #[allow(dead_code)]
    pub(crate) fn get_parent(&self, level: usize) -> Option<Node<'a>> {
        let mut level = level;
        let mut node = *self;
        while level != 0 {
            node = node.parent()?;
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
}

/// An `AST` cursor.
#[derive(Clone)]
pub struct Cursor<'a>(TreeCursor<'a>, Option<&'a Parents<'a>>);

impl<'a> Cursor<'a> {
    pub(crate) fn reset(&mut self, node: &Node<'a>) {
        self.0.reset(node.0);
        self.1 = node.1;
    }

    pub(crate) fn goto_next_sibling(&mut self) -> bool {
        self.0.goto_next_sibling()
    }

    pub(crate) fn goto_first_child(&mut self) -> bool {
        self.0.goto_first_child()
    }

    pub(crate) fn node(&self) -> Node<'a> {
        Node(self.0.node(), self.1)
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;
    use crate::{
        Checker, CppParser, Javascript, JavascriptCode, JavascriptParser, ParserTrait, Search,
        metrics,
    };

    #[test]
    fn parents_match_tree_sitter() {
        let code = "namespace ns {
            int f(int a) {
                if (a > 0) {
                    return [](int b) { return b * 2; }(a);
                }
                for (int i = 0; i < a; i++) { a += i; }
                return a;
            }
            struct S { int x; void g(); };
        }";
        let parser = CppParser::new(code.as_bytes().to_vec(), Path::new("foo.cpp"), None);
        let root = parser.get_root();
        let parents = Parents::new(&root);

        let mut stack = vec![root.with_parents(&parents)];
        let mut count = 0;
        while let Some(node) = stack.pop() {
            assert_eq!(
                node.parent().map(|parent| parent.id()),
                node.0.parent().map(|parent| parent.id()),
                "Wrong parent for {node:?}"
            );
            stack.extend(node.children());
            count += 1;
        }
        assert_eq!(count, root.0.descendant_count());
    }

    // A parser can return a subtree as its root: the ancestors above it must
    // still be found, e.g. to tell a function from a closure.
    #[test]
    fn parents_above_subtree_root() {
        let code = "const f = () => 1;";
        let parser = JavascriptParser::new(code.as_bytes().to_vec(), Path::new("foo.js"), None);
        let arrow = parser
            .get_root()
            .first_occurrence(|id| id == Javascript::ArrowFunction)
            .unwrap();
        let parents = Parents::new(&arrow);
        let arrow = arrow.with_parents(&parents);

        assert_eq!(
            arrow.parent().map(|parent| parent.kind_id()),
            Some(Javascript::VariableDeclarator as u16)
        );
        assert!(JavascriptCode::is_func(&arrow));
        assert!(!JavascriptCode::is_closure(&arrow));
    }

    // Every declaration below is nested in all the namespaces before it, and
    // the LOC metric walks up through all its ancestors. With O(depth) parent
    // lookups, this test takes about ten times longer (a deeper tree makes the
    // difference much larger, see #1258).
    #[test]
    fn metrics_on_deep_tree() {
        let depth = 500;
        let code = "namespace a { int x; ".repeat(depth) + &"}".repeat(depth);
        let parser = CppParser::new(code.into_bytes(), Path::new("foo.cpp"), None);
        let space = metrics(&parser, Path::new("foo.cpp")).unwrap();
        assert_eq!(space.metrics.loc.lloc(), depth as f64);
    }
}
