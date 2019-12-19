use crate::traits::Search;
use tree_sitter::Node;

impl<'a> Search<'a> for Node<'a> {
    fn first_occurence(&self, pred: fn(u16) -> bool) -> Option<Node<'a>> {
        let mut cursor = self.walk();
        let mut stack = Vec::new();
        let mut children = Vec::new();

        stack.push(*self);

        while let Some(node) = stack.pop() {
            if pred(node.kind_id()) {
                return Some(node);
            }
            cursor.reset(node);
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
        let mut cursor = self.walk();
        let mut stack = Vec::new();
        let mut children = Vec::new();

        stack.push(*self);

        while let Some(node) = stack.pop() {
            action(&node);
            cursor.reset(node);
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
        let mut cursor = self.walk();
        for child in self.children(&mut cursor) {
            if pred(child.kind_id()) {
                return Some(child);
            }
        }
        None
    }

    fn act_on_child(&self, action: &mut dyn FnMut(&Node<'a>)) {
        let mut cursor = self.walk();
        for child in self.children(&mut cursor) {
            action(&child);
        }
    }
}
