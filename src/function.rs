use std::io::Write;
use std::path::PathBuf;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, StandardStreamLock, WriteColor};

use crate::traits::*;

use crate::checker::Checker;
use crate::getter::Getter;

#[derive(Debug, Serialize)]
pub struct FunctionSpan {
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub error: bool,
}

pub fn function<T: TSParserTrait>(parser: &T) -> Vec<FunctionSpan> {
    let root = parser.get_root();
    let code = parser.get_code();
    let mut spans = Vec::new();
    root.act_on_node(&mut |n| {
        if T::Checker::is_func(n) {
            let start_line = n.start_position().row + 1;
            let end_line = n.end_position().row + 1;
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

    color!(stdout, Blue);
    write!(stdout, "{}", pref)?;

    if span.error {
        color!(stdout, Red, true);
        write!(stdout, "error: ")?;
    } else {
        color!(stdout, Magenta, true);
        write!(stdout, "{}: ", span.name)?;
    }

    color!(stdout, Green);
    write!(stdout, "from line ")?;

    color!(stdout, White);
    write!(stdout, "{}", span.start_line)?;

    color!(stdout, Green);
    write!(stdout, " to line ")?;

    color!(stdout, White);
    writeln!(stdout, "{}.", span.end_line)
}

fn dump_spans(mut spans: Vec<FunctionSpan>, path: PathBuf) -> std::io::Result<()> {
    if !spans.is_empty() {
        let stdout = StandardStream::stdout(ColorChoice::Always);
        let mut stdout = stdout.lock();

        color!(stdout, Yellow, true);
        writeln!(&mut stdout, "In file {}", path.to_str().unwrap_or("..."))?;

        for span in spans.drain(..spans.len() - 1) {
            dump_span(span, &mut stdout, false)?;
        }
        dump_span(spans.pop().unwrap(), &mut stdout, true)?;
        color!(stdout, White);
    }
    Ok(())
}

pub struct FunctionCfg {
    pub path: PathBuf,
}

pub struct Function {}

impl Callback for Function {
    type Res = std::io::Result<()>;
    type Cfg = FunctionCfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        dump_spans(function(parser), cfg.path)
    }
}
