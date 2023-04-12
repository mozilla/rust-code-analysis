use std::io::Write;

use termcolor::{Color, ColorChoice, StandardStream, StandardStreamLock};

use crate::node::Node;
use crate::tools::{color, intense_color};

use crate::traits::*;

/// Dumps the `AST` of a code.
///
/// Returns a [`Result`] value, when an error occurs.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::{dump_node, CppParser, ParserTrait};
///
/// let source_code = "int a = 42;";
///
/// // The path to a dummy file used to contain the source code
/// let path = PathBuf::from("foo.c");
/// let source_as_vec = source_code.as_bytes().to_vec();
///
/// // The parser of the code, in this case a CPP parser
/// let parser = CppParser::new(source_as_vec.clone(), &path, None);
///
/// // The root of the AST
/// let root = parser.get_root();
///
/// // Dump the AST from the first line of code in a file to the last one
/// dump_node(&source_as_vec, &root, -1, None, None).unwrap();
/// ```
///
/// [`Result`]: #variant.Result
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
        code,
        node,
        "",
        true,
        &mut stdout,
        depth,
        &line_start,
        &line_end,
    );

    color(&mut stdout, Color::White)?;

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

    let (pref_child, pref) = if node.parent().is_none() {
        ("", "")
    } else if last {
        ("   ", "╰─ ")
    } else {
        ("│  ", "├─ ")
    };

    let node_row = node.start_row() + 1;
    let mut display = true;
    if let Some(line_start) = line_start {
        display = node_row >= *line_start
    }
    if let Some(line_end) = line_end {
        display = display && node_row <= *line_end
    }

    if display {
        color(stdout, Color::Blue)?;
        write!(stdout, "{prefix}{pref}")?;

        intense_color(stdout, Color::Yellow)?;
        write!(stdout, "{{{}:{}}} ", node.kind(), node.kind_id())?;

        color(stdout, Color::White)?;
        write!(stdout, "from ")?;

        color(stdout, Color::Green)?;
        let (pos_row, pos_column) = node.start_position();
        write!(stdout, "({}, {}) ", pos_row + 1, pos_column + 1)?;

        color(stdout, Color::White)?;
        write!(stdout, "to ")?;

        color(stdout, Color::Green)?;
        let (pos_row, pos_column) = node.end_position();
        write!(stdout, "({}, {}) ", pos_row + 1, pos_column + 1)?;

        if node.start_row() == node.end_row() {
            color(stdout, Color::White)?;
            write!(stdout, ": ")?;

            intense_color(stdout, Color::Red)?;
            let code = &code[node.start_byte()..node.end_byte()];
            if let Ok(code) = String::from_utf8(code.to_vec()) {
                write!(stdout, "{code} ")?;
            } else {
                stdout.write_all(code).unwrap();
            }
        }

        writeln!(stdout)?;
    }

    let count = node.child_count();
    if count != 0 {
        let prefix = format!("{prefix}{pref_child}");
        let mut i = count;
        let mut cursor = node.cursor();
        cursor.goto_first_child();

        loop {
            i -= 1;
            dump_tree_helper(
                code,
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

/// Configuration options for dumping the `AST` of a code.
pub struct DumpCfg {
    /// The first line of code to dump
    ///
    /// If `None`, the code is dumped from the first line of code
    /// in a file
    pub line_start: Option<usize>,
    /// The last line of code to dump
    ///
    /// If `None`, the code is dumped until the last line of code
    /// in a file
    pub line_end: Option<usize>,
}

pub struct Dump {
    _guard: (),
}

impl Callback for Dump {
    type Res = std::io::Result<()>;
    type Cfg = DumpCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        dump_node(
            parser.get_code(),
            &parser.get_root(),
            -1,
            cfg.line_start,
            cfg.line_end,
        )
    }
}
