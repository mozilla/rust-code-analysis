use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};
use tree_sitter::Node;

use crate::traits::*;

pub fn dump_node(
    code: &[u8],
    node: &Node,
    depth: i32,
    line_start: Option<usize>,
    line_end: Option<usize>,
) {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();
    dump_tree_helper(
        &code,
        &node,
        "",
        true,
        &mut stdout,
        depth,
        &line_start,
        &line_end,
    );
    color!(stdout, White);
}

#[allow(clippy::too_many_arguments)]
fn dump_tree_helper(
    code: &[u8],
    node: &Node,
    prefix: &str,
    last: bool,
    mut stdout: &mut StandardStreamLock,
    depth: i32,
    line_start: &Option<usize>,
    line_end: &Option<usize>,
) {
    if depth == 0 {
        return;
    }

    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    let node_row = node.start_position().row + 1;
    let mut display = true;
    if let Some(line_start) = line_start {
        display = node_row >= *line_start
    }
    if let Some(line_end) = line_end {
        display = display && node_row <= *line_end
    }

    if display {
        color!(stdout, Blue);
        write!(&mut stdout, "{}{}", prefix, pref).unwrap();

        color!(stdout, Yellow, true);
        write!(&mut stdout, "{{{}}} ", node.kind()).unwrap();

        color!(stdout, White);
        write!(&mut stdout, "from ").unwrap();

        color!(stdout, Green);
        let pos = node.start_position();
        write!(&mut stdout, "({}, {}) ", pos.row + 1, pos.column + 1).unwrap();

        color!(stdout, White);
        write!(&mut stdout, "to ").unwrap();

        color!(stdout, Green);
        let pos = node.end_position();
        write!(&mut stdout, "({}, {}) ", pos.row + 1, pos.column + 1).unwrap();

        if node.start_position().row == node.end_position().row {
            color!(stdout, White);
            write!(&mut stdout, ": ").unwrap();

            color!(stdout, Red, true);
            let code = &code[node.start_byte()..node.end_byte()];
            if let Ok(code) = String::from_utf8(code.to_vec()) {
                write!(&mut stdout, "{} ", code).unwrap();
            } else {
                stdout.write_all(code).unwrap();
            }
        }

        writeln!(&mut stdout).unwrap();
    }

    let count = node.child_count();
    if count != 0 {
        let prefix = format!("{}{}", prefix, pref_child);
        let mut i = count;
        let mut cursor = node.walk();
        cursor.goto_first_child();

        loop {
            i -= 1;
            dump_tree_helper(
                &code,
                &cursor.node(),
                &prefix,
                i == 0,
                &mut stdout,
                depth - 1,
                line_start,
                line_end,
            );
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

pub struct DumpCfg {
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
}

pub struct Dump {}

impl Callback for Dump {
    type Res = ();
    type Cfg = DumpCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        dump_node(
            &parser.get_code(),
            &parser.get_root(),
            -1,
            cfg.line_start,
            cfg.line_end,
        );
    }
}
