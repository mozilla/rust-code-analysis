use std::io::{self, Write};
use std::path::PathBuf;

use crate::checker::Checker;
use crate::tools::*;
use crate::traits::*;

const CR: [u8; 8192] = [b'\n'; 8192];

pub fn rm_comments<T: TSParserTrait>(parser: &T) -> Option<Vec<u8>> {
    let node = parser.get_root();
    let mut stack = Vec::new();
    let mut cursor = node.walk();
    let mut spans = Vec::new();

    stack.push(node);

    while let Some(node) = stack.pop() {
        if T::Checker::is_comment(&node) && !T::Checker::is_useful_comment(&node, parser.get_code())
        {
            let lines = node.end_position().row - node.start_position().row;
            spans.push((node.start_byte(), node.end_byte(), lines));
        } else {
            cursor.reset(node);
            if cursor.goto_first_child() {
                loop {
                    stack.push(cursor.node());
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
        }
    }
    if !spans.is_empty() {
        Some(remove_from_code(parser.get_code(), spans))
    } else {
        None
    }
}

fn remove_from_code(code: &[u8], mut spans: Vec<(usize, usize, usize)>) -> Vec<u8> {
    let mut new_code = Vec::with_capacity(code.len());
    let mut code_start = 0;
    for (start, end, lines) in spans.drain(..).rev() {
        new_code.extend(&code[code_start..start]);
        if lines != 0 {
            if lines <= CR.len() {
                new_code.extend(&CR[..lines]);
            } else {
                new_code.resize_with(new_code.len() + lines, || b'\n');
            }
        }
        code_start = end;
    }
    if code_start < code.len() {
        new_code.extend(&code[code_start..]);
    }
    new_code
}

pub struct CommentRmCfg {
    pub in_place: bool,
    pub path: PathBuf,
}

pub struct CommentRm {}

impl Callback for CommentRm {
    type Res = std::io::Result<()>;
    type Cfg = CommentRmCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        if let Some(new_source) = rm_comments(parser) {
            if cfg.in_place {
                write_file(&cfg.path, &new_source)?;
            } else if let Ok(new_source) = std::str::from_utf8(&new_source) {
                println!("{}", new_source);
            } else {
                io::stdout().write_all(&new_source)?;
            }
        }
        Ok(())
    }
}
