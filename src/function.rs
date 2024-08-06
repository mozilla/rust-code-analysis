use std::io::Write;
use std::path::PathBuf;

use serde::Serialize;
use termcolor::{Color, ColorChoice, StandardStream, StandardStreamLock};

use crate::traits::*;

use crate::checker::Checker;
use crate::getter::Getter;

use crate::tools::{color, intense_color};

/// Function span data.
#[derive(Debug, Serialize)]
pub struct FunctionSpan {
    /// The function name
    pub name: String,
    /// The first line of a function
    pub start_line: usize,
    /// The last line of a function
    pub end_line: usize,
    /// If `true`, an error is occurred in determining the span
    /// of a function
    pub error: bool,
}

/// Detects the span of each function in a code.
///
/// Returns a vector containing the [`FunctionSpan`] of each function
///
/// [`FunctionSpan`]: struct.FunctionSpan.html
pub fn function<T: ParserTrait>(parser: &T) -> Vec<FunctionSpan> {
    let root = parser.get_root();
    let code = parser.get_code();
    let mut spans = Vec::new();
    root.act_on_node(&mut |n| {
        if T::Checker::is_func(n) {
            let start_line = n.start_row() + 1;
            let end_line = n.end_row() + 1;
            if let Some(name) = T::Getter::get_func_name(n, code) {
                spans.push(FunctionSpan {
                    name: name.to_string(),
                    start_line,
                    end_line,
                    error: false,
                });
            } else {
                spans.push(FunctionSpan {
                    name: "".to_string(),
                    start_line,
                    end_line,
                    error: true,
                });
            }
        }
    });

    spans
}

fn dump_span(
    span: FunctionSpan,
    stdout: &mut StandardStreamLock,
    last: bool,
) -> std::io::Result<()> {
    /*if !span.error {
        return Ok(());
    }*/

    let pref = if last { "   `- " } else { "   |- " };

    color(stdout, Color::Blue)?;
    write!(stdout, "{pref}")?;

    if span.error {
        intense_color(stdout, Color::Red)?;
        write!(stdout, "error: ")?;
    } else {
        intense_color(stdout, Color::Magenta)?;
        write!(stdout, "{}: ", span.name)?;
    }

    color(stdout, Color::Green)?;
    write!(stdout, "from line ")?;

    color(stdout, Color::White)?;
    write!(stdout, "{}", span.start_line)?;

    color(stdout, Color::Green)?;
    write!(stdout, " to line ")?;

    color(stdout, Color::White)?;
    writeln!(stdout, "{}.", span.end_line)
}

fn dump_spans(mut spans: Vec<FunctionSpan>, path: PathBuf) -> std::io::Result<()> {
    if !spans.is_empty() {
        let stdout = StandardStream::stdout(ColorChoice::Always);
        let mut stdout = stdout.lock();

        intense_color(&mut stdout, Color::Yellow)?;
        writeln!(&mut stdout, "In file {}", path.to_str().unwrap_or("..."))?;

        for span in spans.drain(..spans.len() - 1) {
            dump_span(span, &mut stdout, false)?;
        }
        dump_span(spans.pop().unwrap(), &mut stdout, true)?;
        color(&mut stdout, Color::White)?;
    }
    Ok(())
}

/// Configuration options for detecting the span of
/// each function in a code.
#[derive(Debug)]
pub struct FunctionCfg {
    /// Path to the file containing the code
    pub path: PathBuf,
}

pub struct Function {
    _guard: (),
}

impl Callback for Function {
    type Res = std::io::Result<()>;
    type Cfg = FunctionCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        dump_spans(function(parser), cfg.path)
    }
}
