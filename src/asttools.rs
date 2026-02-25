//! AST traversal and analysis utilities.
//!
//! This module provides helper functions and macros for traversing
//! and analyzing the AST tree structure.

use crate::node::Node;

/// Gets an ancestor at a specific level above the current node.
///
/// # Arguments
/// * `node` - The starting node
/// * `level` - How many levels up to traverse (0 returns the node itself)
///
/// # Returns
/// The ancestor node at the specified level, or None if the tree isn't deep enough.
///
/// # Example
/// ```ignore
/// // Get the grandparent (2 levels up)
/// if let Some(grandparent) = get_parent(&node, 2) {
///     println!("Grandparent kind: {}", grandparent.kind());
/// }
/// ```
pub fn get_parent<'a>(node: &Node<'a>, level: usize) -> Option<Node<'a>> {
    let mut level = level;
    let mut current = *node;
    while level != 0 {
        current = current.parent()?;
        level -= 1;
    }
    Some(current)
}

/// Traverses a tree passing from children to children in search of a specific
/// token or series of tokens.
///
/// # Arguments
/// * `node` - The starting node
/// * `token_list` - A slice of predicates, each matching a level of descent
///
/// # Returns
/// The final node after following the token path, or None if any token wasn't found.
///
/// # Example
/// ```ignore
/// // Find: node -> child matching pred1 -> grandchild matching pred2
/// let result = traverse_children(&node, &[
///     |id| id == SomeToken::Foo as u16,
///     |id| id == SomeToken::Bar as u16,
/// ]);
/// ```
pub fn traverse_children<'a, F>(node: &Node<'a>, token_list: &[F]) -> Option<Node<'a>>
where
    F: Fn(u16) -> bool,
{
    let mut current = *node;
    'outer: for token in token_list {
        for child in current.children() {
            if token(child.kind_id()) {
                current = child;
                continue 'outer;
            }
        }
        // Token not found at this level
        return None;
    }
    Some(current)
}

/// Checks if a node has specific ancestors in sequence.
///
/// This macro checks if the node's ancestors match a specific pattern,
/// where the first pattern(s) are immediate ancestors and the last pattern
/// is the final ancestor to match.
///
/// # Example
/// ```ignore
/// // Check if node is inside a function inside a class
/// let is_method = has_ancestors!(node, Class | Struct, Function);
/// ```
#[macro_export]
macro_rules! has_ancestors {
    ($node:expr, $( $typs:pat_param )|*, $( $typ:pat_param ),+) => {{
        let mut res = false;
        loop {
            let mut node = *$node;
            $(
                if let Some(parent) = node.parent() {
                    match parent.kind_id().into() {
                        $typ => {
                            node = parent;
                        },
                        _ => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            )*
            if let Some(parent) = node.parent() {
                match parent.kind_id().into() {
                    $( $typs )|+ => {
                        res = true;
                    },
                    _ => {}
                }
            }
            break;
        }
        res
    }};
}

/// Counts specific ancestors matching a pattern until a stop condition.
///
/// This macro traverses up the tree counting ancestors that match the given
/// patterns, stopping when it encounters an ancestor matching the stop pattern.
///
/// # Example
/// ```ignore
/// // Count nested if statements until we hit a function boundary
/// let nesting = count_specific_ancestors!(node, If | ElseIf, Function | Method);
/// ```
#[macro_export]
macro_rules! count_specific_ancestors {
    ($node:expr, $checker:ty, $( $typs:pat_param )|*, $( $stops:pat_param )|*) => {{
        let mut count = 0;
        let mut node = *$node;
        while let Some(parent) = node.parent() {
            match parent.kind_id().into() {
                $( $typs )|* => {
                    if !<$checker>::is_else_if(&parent) {
                        count += 1;
                    }
                },
                $( $stops )|* => break,
                _ => {}
            }
            node = parent;
        }
        count
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_parent_level_zero() {
        // Level 0 should return the same node
        // (actual test would need a real node)
    }
}
