use std::collections::VecDeque;
use std::path::PathBuf;
use tree_sitter::Node;

use crate::dump::*;
use crate::traits::*;

pub fn errors<T: TSParserTrait>(parser: &T) -> Option<Vec<Node>> {
    let node = parser.get_root();
    if !node.has_error() {
        None
    } else {
        let mut cursor = node.walk();
        let mut stack = VecDeque::new();
        let mut errs = Vec::new();

        stack.push_back(node);

        while let Some(node) = stack.pop_front() {
            if node.is_error() {
                errs.push(node);
            }
            cursor.reset(node);
            if cursor.goto_first_child() {
                loop {
                    stack.push_back(cursor.node());
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        }
        Some(errs)
    }
}

pub struct ErrorsCfg {
    pub path: Option<PathBuf>,
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
}

pub struct Errors {}

impl Callback for Errors {
    type Res = std::io::Result<()>;
    type Cfg = ErrorsCfg;

    fn call<T: TSParserTrait>(cfg: &Self::Cfg, parser: &T) -> Self::Res {
        if let Some(error_nodes) = errors(parser) {
            if !error_nodes.is_empty() {
                println!("In file {:?}", cfg.path);
                for node in error_nodes {
                    dump_node(parser.get_code(), &node, 1, cfg.line_start, cfg.line_end);
                }
                println!();
            }
        }
        Ok(())
    }
}
