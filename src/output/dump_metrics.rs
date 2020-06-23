use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};

use crate::cognitive;
use crate::cyclomatic;
use crate::exit;
use crate::fn_args;
use crate::halstead;
use crate::loc;
use crate::mi;
use crate::nom;

use crate::spaces::{CodeMetrics, FuncSpace};

/// Dumps the metrics of a code.
///
/// Returns a [`Result`] value, when an error occurs.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::{dump_root, metrics, CppParser, ParserTrait};
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
/// // Compute metrics
/// let space = metrics(&parser, &path).unwrap();
///
/// // Dump all metrics
/// dump_root(&space).unwrap();
/// # }
/// ```
///
/// [`Result`]: #variant.Result
pub fn dump_root(space: &FuncSpace) -> std::io::Result<()> {
    let stdout = StandardStream::stdout(ColorChoice::Always);
    let mut stdout = stdout.lock();
    dump_space(&space, "", true, &mut stdout)?;
    color!(stdout, White);

    Ok(())
}

fn dump_space(
    space: &FuncSpace,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Yellow, true);
    write!(stdout, "{}: ", space.kind)?;

    color!(stdout, Cyan, true);
    write!(stdout, "{}", space.name.as_ref().map_or("", |name| &name))?;

    color!(stdout, Red, true);
    writeln!(stdout, " (@{})", space.start_line)?;

    let prefix = format!("{}{}", prefix, pref_child);
    dump_metrics(&space.metrics, &prefix, space.spaces.is_empty(), stdout)?;

    if let Some((last, spaces)) = space.spaces.split_last() {
        for space in spaces {
            dump_space(space, &prefix, false, stdout)?;
        }
        dump_space(last, &prefix, true, stdout)?;
    }

    Ok(())
}

fn dump_metrics(
    metrics: &CodeMetrics,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Yellow, true);
    writeln!(stdout, "metrics")?;

    let prefix = format!("{}{}", prefix, pref_child);
    dump_cognitive(&metrics.cognitive, &prefix, false, stdout)?;
    dump_cyclomatic(&metrics.cyclomatic, &prefix, false, stdout)?;
    dump_nargs(&metrics.nargs, &prefix, false, stdout)?;
    dump_nexits(&metrics.nexits, &prefix, false, stdout)?;
    dump_halstead(&metrics.halstead, &prefix, false, stdout)?;
    dump_loc(&metrics.loc, &prefix, false, stdout)?;
    dump_nom(&metrics.nom, &prefix, false, stdout)?;
    dump_mi(&metrics.mi, &prefix, true, stdout)
}

fn dump_cognitive(
    stats: &cognitive::Stats,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let pref = if last { "`- " } else { "|- " };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Green, true);
    write!(stdout, "cognitive: ")?;

    color!(stdout, White);
    writeln!(stdout, "{}", stats.cognitive())
}

fn dump_cyclomatic(
    stats: &cyclomatic::Stats,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let pref = if last { "`- " } else { "|- " };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Green, true);
    write!(stdout, "cyclomatic: ")?;

    color!(stdout, White);
    writeln!(stdout, "{}", stats.cyclomatic())
}

fn dump_halstead(
    stats: &halstead::Stats,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Green, true);
    writeln!(stdout, "halstead")?;

    let prefix = format!("{}{}", prefix, pref_child);

    dump_value("n1", stats.u_operators(), &prefix, false, stdout)?;
    dump_value("N1", stats.operators(), &prefix, false, stdout)?;
    dump_value("n2", stats.u_operands(), &prefix, false, stdout)?;
    dump_value("N2", stats.operands(), &prefix, false, stdout)?;

    dump_value("length", stats.length(), &prefix, false, stdout)?;
    dump_value(
        "estimated program length",
        stats.estimated_program_length(),
        &prefix,
        false,
        stdout,
    )?;
    dump_value("purity ratio", stats.purity_ratio(), &prefix, false, stdout)?;
    dump_value("vocabulary", stats.vocabulary(), &prefix, false, stdout)?;
    dump_value("volume", stats.volume(), &prefix, false, stdout)?;
    dump_value("difficulty", stats.difficulty(), &prefix, false, stdout)?;
    dump_value("level", stats.level(), &prefix, false, stdout)?;
    dump_value("effort", stats.effort(), &prefix, false, stdout)?;
    dump_value("time", stats.time(), &prefix, false, stdout)?;
    dump_value("bugs", stats.bugs(), &prefix, true, stdout)
}

fn dump_loc(
    stats: &loc::Stats,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Green, true);
    writeln!(stdout, "loc")?;

    let prefix = format!("{}{}", prefix, pref_child);
    dump_value("sloc", stats.sloc(), &prefix, false, stdout)?;
    dump_value("ploc", stats.ploc(), &prefix, false, stdout)?;
    dump_value("lloc", stats.lloc(), &prefix, false, stdout)?;
    dump_value("cloc", stats.cloc(), &prefix, false, stdout)?;
    dump_value("blank", stats.blank(), &prefix, true, stdout)
}

fn dump_nom(
    stats: &nom::Stats,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Green, true);
    writeln!(stdout, "nom")?;

    let prefix = format!("{}{}", prefix, pref_child);
    dump_value("functions", stats.functions(), &prefix, false, stdout)?;
    dump_value("closures", stats.closures(), &prefix, false, stdout)?;
    dump_value("total", stats.total(), &prefix, true, stdout)
}

fn dump_mi(
    stats: &mi::Stats,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let (pref_child, pref) = if last { ("   ", "`- ") } else { ("|  ", "|- ") };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Green, true);
    writeln!(stdout, "mi")?;

    let prefix = format!("{}{}", prefix, pref_child);
    dump_value("mi_original", stats.mi_original(), &prefix, false, stdout)?;
    dump_value("mi_sei", stats.mi_sei(), &prefix, false, stdout)?;
    dump_value(
        "mi_visual_studio",
        stats.mi_visual_studio(),
        &prefix,
        true,
        stdout,
    )
}

fn dump_nargs(
    stats: &fn_args::Stats,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let pref = if last { "`- " } else { "|- " };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Green, true);
    write!(stdout, "n_args: ")?;

    color!(stdout, White);
    writeln!(stdout, "{}", stats.n_args())
}

fn dump_nexits(
    stats: &exit::Stats,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let pref = if last { "`- " } else { "|- " };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Green, true);
    write!(stdout, "n_exits: ")?;

    color!(stdout, White);
    writeln!(stdout, "{}", stats.exit())
}

fn dump_value(
    name: &str,
    val: f64,
    prefix: &str,
    last: bool,
    stdout: &mut StandardStreamLock,
) -> std::io::Result<()> {
    let pref = if last { "`- " } else { "|- " };

    color!(stdout, Blue);
    write!(stdout, "{}{}", prefix, pref)?;

    color!(stdout, Magenta, true);
    write!(stdout, "{}: ", name)?;

    color!(stdout, White);
    writeln!(stdout, "{}", val)
}
