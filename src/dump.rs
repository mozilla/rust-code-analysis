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
        "".to_string(),
        true,
        &mut stdout,
        depth,
        &line_start,
        &line_end,
    );
    stdout
        .set_color(ColorSpec::new().set_fg(Some(Color::White)))
        .unwrap();
}

fn dump_tree_helper(
    code: &[u8],
    node: &Node,
    prefix: String,
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
        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Blue)))
            .unwrap();
        write!(&mut stdout, "{}{}", prefix, pref).unwrap();

        stdout
            .set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::Yellow))
                    .set_intense(true),
            )
            .unwrap();
        write!(&mut stdout, "{{{}}} ", node.kind()).unwrap();

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .unwrap();
        write!(&mut stdout, "from ").unwrap();

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();
        let pos = node.start_position();
        write!(&mut stdout, "({}, {}) ", pos.row + 1, pos.column + 1).unwrap();

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::White)))
            .unwrap();
        write!(&mut stdout, "to ").unwrap();

        stdout
            .set_color(ColorSpec::new().set_fg(Some(Color::Green)))
            .unwrap();
        let pos = node.end_position();
        write!(&mut stdout, "({}, {}) ", pos.row + 1, pos.column + 1).unwrap();

        if node.start_position().row == node.end_position().row {
            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::White)))
                .unwrap();
            write!(&mut stdout, ": ").unwrap();

            stdout
                .set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_intense(true))
                .unwrap();
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
        let prefix = prefix + pref_child;
        let mut i = count;
        let mut cursor = node.walk();
        cursor.goto_first_child();

        loop {
            i -= 1;
            dump_tree_helper(
                &code,
                &cursor.node(),
                prefix.to_string(),
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
