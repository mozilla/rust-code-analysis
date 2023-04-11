use std::io::{self, Write};
use std::path::PathBuf;

use crate::checker::Checker;
use crate::node::Node;

use crate::tools::*;
use crate::traits::*;

const CR: [u8; 8192] = [b'\n'; 8192];

/// Removes comments from a code.
pub fn rm_comments<T: ParserTrait>(parser: &T) -> Option<Vec<u8>> {
    let node = parser.get_root();
    let mut stack = Vec::new();
    let mut cursor = node.object().walk();
    let mut spans = Vec::new();

    stack.push(node);

    while let Some(node) = stack.pop() {
        if T::Checker::is_comment(&node) && !T::Checker::is_useful_comment(&node, parser.get_code())
        {
            let lines = node.object().end_position().row - node.start_row();
            spans.push((node.object().start_byte(), node.object().end_byte(), lines));
        } else {
            cursor.reset(node.object());
            if cursor.goto_first_child() {
                loop {
                    stack.push(Node::new(cursor.node()));
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

/// Configuration options for removing comments from a code.
pub struct CommentRmCfg {
    /// If `true`, the modified code is saved on a file
    pub in_place: bool,
    /// Path to output file
    pub path: PathBuf,
}

pub struct CommentRm {
    _guard: (),
}

impl Callback for CommentRm {
    type Res = std::io::Result<()>;
    type Cfg = CommentRmCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        if let Some(new_source) = rm_comments(parser) {
            if cfg.in_place {
                write_file(&cfg.path, &new_source)?;
            } else if let Ok(new_source) = std::str::from_utf8(&new_source) {
                println!("{new_source}");
            } else {
                io::stdout().write_all(&new_source)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{CcommentParser, ParserTrait};

    use super::rm_comments;

    const SOURCE_CODE: &str = "/* Remove this code block */\n\
                               int a = 42; // Remove this comment\n\
                               // Remove this comment\n\
                               int b = 42;\n\
                               /* Remove\n\
                                * this\n\
                                * comment\n\
                                */";

    const SOURCE_CODE_NO_COMMENTS: &str = "\n\
                                           int a = 42; \n\
                                           \n\
                                           int b = 42;\n\
                                           \n\
                                           \n\
                                           \n\
                                           \n";

    #[test]
    fn ccomment_remove_comments() {
        let path = PathBuf::from("foo.c");
        let mut trimmed_bytes = SOURCE_CODE.as_bytes().to_vec();
        trimmed_bytes.push(b'\n');
        let parser = CcommentParser::new(trimmed_bytes, &path, None);

        let no_comments = rm_comments(&parser).unwrap();

        assert_eq!(no_comments.as_slice(), SOURCE_CODE_NO_COMMENTS.as_bytes());
    }
}
