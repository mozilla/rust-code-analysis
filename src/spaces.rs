use serde::Serialize;
use std::fmt;
use std::path::PathBuf;
use tree_sitter::Node;

use crate::checker::Checker;
use crate::cyclomatic::{self, Cyclomatic};
use crate::exit::{self, Exit};
use crate::fn_args::{self, NArgs};
use crate::getter::Getter;
use crate::halstead::{self, Halstead};
use crate::loc::{self, Loc};
use crate::mi::{self, Mi};
use crate::nom::{self, Nom};

use crate::dump_formats::*;
use crate::dump_metrics::*;
use crate::traits::*;

/// The list of supported space kinds.
#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum SpaceKind {
    /// An unknown space
    Unknown,
    /// A function space
    Function,
    /// A class space
    Class,
    /// A struct space
    Struct,
    /// A `Rust` trait space
    Trait,
    /// A `Rust` implementation space
    Impl,
    /// A general space
    Unit,
    /// A `C/C++` namespace
    Namespace,
}

impl fmt::Display for SpaceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            SpaceKind::Unknown => "unknown",
            SpaceKind::Function => "function",
            SpaceKind::Class => "class",
            SpaceKind::Struct => "struct",
            SpaceKind::Trait => "trait",
            SpaceKind::Impl => "impl",
            SpaceKind::Unit => "unit",
            SpaceKind::Namespace => "namespace",
        };
        write!(f, "{}", s)
    }
}

/// All metrics data.
#[derive(Debug, Serialize)]
pub struct CodeMetrics<'a> {
    /// `NArgs` data
    pub nargs: fn_args::Stats,
    /// `NExits` data
    pub nexits: exit::Stats,
    /// `Cyclomatic` data
    pub cyclomatic: cyclomatic::Stats,
    /// `Halstead` data
    pub halstead: halstead::Stats<'a>,
    /// `Loc` data
    pub loc: loc::Stats,
    /// `Nom` data
    pub nom: nom::Stats,
    /// `Mi` data
    pub mi: mi::Stats,
}

impl<'a> Default for CodeMetrics<'a> {
    fn default() -> Self {
        Self {
            cyclomatic: cyclomatic::Stats::default(),
            halstead: halstead::Stats::default(),
            loc: loc::Stats::default(),
            nom: nom::Stats::default(),
            mi: mi::Stats::default(),
            nargs: fn_args::Stats::default(),
            nexits: exit::Stats::default(),
        }
    }
}

impl<'a> fmt::Display for CodeMetrics<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.nargs)?;
        writeln!(f, "{}", self.nexits)?;
        writeln!(f, "{}", self.cyclomatic)?;
        writeln!(f, "{}", self.halstead)?;
        writeln!(f, "{}", self.loc)?;
        writeln!(f, "{}", self.nom)?;
        write!(f, "{}", self.mi)
    }
}

impl<'a> CodeMetrics<'a> {
    pub fn merge(&mut self, other: &CodeMetrics<'a>) {
        self.cyclomatic.merge(&other.cyclomatic);
        self.halstead.merge(&other.halstead);
        self.loc.merge(&other.loc);
        self.nom.merge(&other.nom);
        self.mi.merge(&other.mi);
        self.nargs.merge(&other.nargs);
        self.nexits.merge(&other.nexits);
    }
}

/// Function space data.
#[derive(Debug, Serialize)]
pub struct FuncSpace<'a> {
    /// The name of a function space
    ///
    /// If `None`, an error is occured in parsing
    /// the name of a function space
    pub name: Option<&'a str>,
    /// The first line of a function space
    pub start_line: usize,
    /// The last line of a function space
    pub end_line: usize,
    /// The space kind
    pub kind: SpaceKind,
    /// All subspaces contained in a function space
    pub spaces: Vec<FuncSpace<'a>>,
    /// All metrics of a function space
    pub metrics: CodeMetrics<'a>,
}

impl<'a> FuncSpace<'a> {
    fn new<T: Getter>(node: &Node<'a>, code: &'a [u8], kind: SpaceKind) -> Self {
        let (start_position, end_position) = match kind {
            SpaceKind::Unit => {
                if node.child_count() == 0 {
                    (0, 0)
                } else {
                    (node.start_position().row + 1, node.end_position().row)
                }
            }
            _ => (node.start_position().row + 1, node.end_position().row + 1),
        };
        Self {
            name: T::get_func_space_name(&node, code),
            spaces: Vec::new(),
            metrics: CodeMetrics::default(),
            kind,
            start_line: start_position,
            end_line: end_position,
        }
    }
}

fn finalize<'a>(space_stack: &mut Vec<FuncSpace<'a>>, diff_level: usize) {
    for _ in 0..diff_level {
        if space_stack.len() <= 1 {
            break;
        }
        let space = space_stack.pop().unwrap();
        let last_space = space_stack.last_mut().unwrap();
        last_space.metrics.merge(&space.metrics);
        last_space.spaces.push(space);
    }
}

/// Returns function space data of the code in a file.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
///
/// use rust_code_analysis::{CppParser, metrics, ParserTrait};
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
/// // Gets the function space data of the code contained in foo.c
/// metrics(&parser, &path).unwrap();
/// # }
/// ```
pub fn metrics<'a, T: ParserTrait>(parser: &'a T, path: &'a PathBuf) -> Option<FuncSpace<'a>> {
    let code = parser.get_code();
    let node = parser.get_root();
    let mut cursor = node.walk();
    let mut stack = Vec::new();
    let mut children = Vec::new();
    let mut space_stack: Vec<FuncSpace> = Vec::new();
    let mut last_level = 0;

    stack.push((node, 0));

    while let Some((node, level)) = stack.pop() {
        if level < last_level {
            finalize(&mut space_stack, last_level - level);
            last_level = level;
        }

        let kind = T::Getter::get_space_kind(&node);

        let func_space = T::Checker::is_func(&node) || T::Checker::is_func_space(&node);
        let unit = kind == SpaceKind::Unit;

        let new_level = if func_space {
            space_stack.push(FuncSpace::new::<T::Getter>(&node, code, kind));
            last_level = level + 1;
            last_level
        } else {
            level
        };

        if let Some(last) = space_stack.last_mut() {
            T::Cyclomatic::compute(&node, &mut last.metrics.cyclomatic);
            T::Halstead::compute(&node, code, &mut last.metrics.halstead);
            T::Loc::compute(&node, &mut last.metrics.loc, func_space, unit);
            T::Nom::compute(&node, &mut last.metrics.nom);
            T::Mi::compute(
                &node,
                &last.metrics.loc,
                &last.metrics.cyclomatic,
                &last.metrics.halstead,
                &mut last.metrics.mi,
            );
            T::NArgs::compute(&node, &mut last.metrics.nargs);
            T::Exit::compute(&node, &mut last.metrics.nexits);
        }

        cursor.reset(node);
        if cursor.goto_first_child() {
            loop {
                children.push((cursor.node(), new_level));
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            for child in children.drain(..).rev() {
                stack.push(child);
            }
        }
    }

    finalize(&mut space_stack, std::usize::MAX);

    space_stack.pop().map(|mut space| {
        space.name = path.to_str();
        space
    })
}

/// Configuration options for computing
/// the metrics of a code.
pub struct MetricsCfg {
    /// Path to the file containing the code
    pub path: PathBuf,
    /// The output format
    pub output_format: Option<Format>,
    /// If `true`, the `CBOR` and `JSON` output formats are
    /// pretty-printed
    pub pretty: bool,
    /// Path to the output file containing the metrics
    ///
    /// If `None`, the metrics are dumped on shell
    pub output_path: Option<PathBuf>,
}

pub struct Metrics {
    _guard: (),
}

impl Callback for Metrics {
    type Res = std::io::Result<()>;
    type Cfg = MetricsCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        if let Some(space) = metrics(parser, &cfg.path) {
            if let Some(output_format) = cfg.output_format {
                dump_formats(
                    &space,
                    &cfg.path,
                    &cfg.output_path,
                    output_format,
                    cfg.pretty,
                )
            } else {
                dump_root(&space)
            }
        } else {
            Ok(())
        }
    }
}
