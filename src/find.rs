use std::path::PathBuf;
use tree_sitter::Node;

use crate::dump::*;
use crate::traits::*;

pub fn find<'a, T: TSParserTrait>(parser: &'a T, filters: &[String]) -> Option<Vec<Node<'a>>> {
    let filters = parser.get_filters(filters);
    let node = parser.get_root();
    let mut cursor = node.walk();
    let mut stack = Vec::new();
    let mut good = Vec::new();
    let mut children = Vec::new();

    stack.push(node);

    while let Some(node) = stack.pop() {
        if filters.any(&node) {
            good.push(node);
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
    Some(good)
}

pub struct FindCfg {
    pub path: Option<PathBuf>,
    pub filters: Vec<String>,
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
}

pub struct Find {}

impl Callback for Find {
    type Res = std::io::Result<()>;
    type Cfg = FindCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        if let Some(good) = find(parser, &cfg.filters) {
            if !good.is_empty() {
                println!("In file {:?}", cfg.path);
                for node in good {
                    dump_node(parser.get_code(), &node, 1, cfg.line_start, cfg.line_end);
                }
                println!();
            }
        }
        Ok(())
    }
}
