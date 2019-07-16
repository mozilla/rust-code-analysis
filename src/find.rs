use std::collections::VecDeque;
use std::path::PathBuf;
use tree_sitter::Node;

use crate::dump::*;
use crate::traits::*;

pub fn find<'a, T: TSParserTrait>(parser: &'a T, filters: &[String]) -> Option<Vec<Node<'a>>> {
    let filters = parser.get_filters(filters);
    let node = parser.get_root();
    let mut cursor = node.walk();
    let mut stack = VecDeque::new();
    let mut good = Vec::new();

    stack.push_back(node);

    while let Some(node) = stack.pop_front() {
        if filters.any(&node) {
            good.push(node);
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
