use std::io::Write;
use termcolor::{Color, ColorChoice, StandardStream, StandardStreamLock};

use crate::ops::Ops;

use crate::tools::{color, intense_color};

/// Dumps all operands and operators of a code.
///
/// Returns a [`Result`] value, when an error occurs.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::{dump_ops, operands_and_operators, CppParser, ParserTrait};
///
/// # fn main() {
/// let source_code = "int a = 42;";
///
/// // The path to a dummy file used to contain the source code
/// let path = PathBuf::from("foo.c");
/// let source_as_vec = source_code.as_bytes().to_vec();
///
/// // The parser of the code, in this case a CPP parser
/// let parser = CppParser::new(source_as_vec, &path, None);
///
/// // Retrieve all operands and operators
/// let ops = operands_and_operators(&parser, &path).unwrap();
///
/// // Dump all operands and operators
/// dump_ops(&ops).unwrap();
/// # }
/// ```
///
/// [`Result`]: #variant.Result
pub fn dump_ops(ops: &Ops) -> std::io::Result<()> {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();
    dump_space(ops, "", true, &mut stdout)?;
    color(&mut stdout, Color::White)?;

    Ok(())
}

fn dump_space(
    space: &Ops,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    color(stdout, Color::Blue)?;
    write!(stdout, "{prefix}{pref}")?;

    intense_color(stdout, Color::Yellow)?;
    write!(stdout, "{}: ", space.kind)?;

    intense_color(stdout, Color::Cyan)?;
    write!(stdout, "{}", space.name.as_ref().map_or("", |name| name))?;

    intense_color(stdout, Color::Red)?;
    writeln!(stdout, " (@{})", space.start_line)?;

    let prefix = format!("{prefix}{pref_child}");
    dump_space_ops(space, &prefix, space.spaces.is_empty(), stdout)?;

    if let Some((last, spaces)) = space.spaces.split_last() {
        for space in spaces {
            dump_space(space, &prefix, false, stdout)?;
        }
        dump_space(last, &prefix, true, stdout)?;
    }

    Ok(())
}

fn dump_space_ops(
    ops: &Ops,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    dump_ops_values("operators", &ops.operators, prefix, last, stdout)?;
    dump_ops_values("operands", &ops.operands, prefix, last, stdout)
}

fn dump_ops_values(
    name: &str,
    ops: &[String],
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    color(stdout, Color::Blue)?;
    write!(stdout, "{prefix}{pref}")?;

    intense_color(stdout, Color::Green)?;
    writeln!(stdout, "{name}")?;

    let prefix = format!("{prefix}{pref_child}");
    for op in ops.iter().take(ops.len() - 1) {
        color(stdout, Color::Blue)?;
        write!(stdout, "{prefix}|- ")?;

        color(stdout, Color::White)?;
        writeln!(stdout, "{op}")?;
    }

    color(stdout, Color::Blue)?;
    write!(stdout, "{prefix}`- ")?;

    color(stdout, Color::White)?;
    writeln!(stdout, "{}", ops.last().unwrap())
}
