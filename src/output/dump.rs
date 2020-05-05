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
) -> std::io::Result<()> {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();
    let ret = dump_tree_helper(
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

    ret
}

#[allow(clippy::too_many_arguments)]
fn dump_tree_helper(
    code: &[u8],
    node: &Node,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
    depth: i32,
    line_start: &Option<usize>,
    line_end: &Option<usize>,
) -> std::io::Result<()> {
    if depth == 0 {
        return Ok(());
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
        write!(stdout, "{}{}", prefix, pref)?;

        color!(stdout, Yellow, true);
        write!(stdout, "{{{}:{}}} ", node.kind(), node.kind_id())?;

        color!(stdout, White);
        write!(stdout, "from ")?;

        color!(stdout, Green);
        let pos = node.start_position();
        write!(stdout, "({}, {}) ", pos.row + 1, pos.column + 1)?;

        color!(stdout, White);
        write!(stdout, "to ")?;

        color!(stdout, Green);
        let pos = node.end_position();
        write!(stdout, "({}, {}) ", pos.row + 1, pos.column + 1)?;

        if node.start_position().row == node.end_position().row {
            color!(stdout, White);
            write!(stdout, ": ")?;

            color!(stdout, Red, true);
            let code = &code[node.start_byte()..node.end_byte()];
            if let Ok(code) = String::from_utf8(code.to_vec()) {
                write!(stdout, "{} ", code)?;
            } else {
                stdout.write_all(code).unwrap();
            }
        }

        writeln!(stdout)?;
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
                stdout,
                depth - 1,
                line_start,
                line_end,
            )?;
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }

    Ok(())
}

pub struct DumpCfg {
    pub line_start: Option<usize>,
    pub line_end: Option<usize>,
}

pub struct Dump {}

impl Callback for Dump {
    type Res = std::io::Result<()>;
    type Cfg = DumpCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        dump_node(
            &parser.get_code(),
            &parser.get_root(),
            -1,
            cfg.line_start,
            cfg.line_end,
        )
    }
}
