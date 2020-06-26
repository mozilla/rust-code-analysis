use serde::Serialize;
use std::fmt;
use std::path::PathBuf;

use crate::checker::Checker;
use crate::node::Node;

use crate::cyclomatic::{self, Cyclomatic};
use crate::exit::{self, Exit};
use crate::fn_args::{self, NArgs};
use crate::getter::Getter;
use crate::halstead::{self, Halstead, HalsteadMaps};
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
#[derive(Debug, Clone, Serialize)]
pub struct CodeMetrics {
    /// `NArgs` data
    pub nargs: fn_args::Stats,
    /// `NExits` data
    pub nexits: exit::Stats,
    /// `Cyclomatic` data
    pub cyclomatic: cyclomatic::Stats,
    /// `Halstead` data
    pub halstead: halstead::Stats,
    /// `Loc` data
    pub loc: loc::Stats,
    /// `Nom` data
    pub nom: nom::Stats,
    /// `Mi` data
    pub mi: mi::Stats,
}

impl Default for CodeMetrics {
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

impl fmt::Display for CodeMetrics {
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

impl CodeMetrics {
    pub fn merge(&mut self, other: &CodeMetrics) {
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
#[derive(Debug, Clone, Serialize)]
pub struct FuncSpace {
    /// The name of a function space
    ///
    /// If `None`, an error is occured in parsing
    /// the name of a function space
    pub name: Option<String>,
    /// The first line of a function space
    pub start_line: usize,
    /// The last line of a function space
    pub end_line: usize,
    /// The space kind
    pub kind: SpaceKind,
    /// All subspaces contained in a function space
    pub spaces: Vec<FuncSpace>,
    /// All metrics of a function space
    pub metrics: CodeMetrics,
}

impl FuncSpace {
    fn new<T: Getter>(node: &Node, code: &[u8], kind: SpaceKind) -> Self {
        let (start_position, end_position) = match kind {
            SpaceKind::Unit => {
                if node.object().child_count() == 0 {
                    (0, 0)
                } else {
                    (
                        node.object().start_position().row + 1,
                        node.object().end_position().row,
                    )
                }
            }
            _ => (
                node.object().start_position().row + 1,
                node.object().end_position().row + 1,
            ),
        };
        Self {
            name: T::get_func_space_name(&node, code).map(|name| name.to_string()),
            spaces: Vec::new(),
            metrics: CodeMetrics::default(),
            kind,
            start_line: start_position,
            end_line: end_position,
        }
    }
}

#[inline(always)]
fn compute_halstead_and_mi<'a, T: ParserTrait>(state: &mut State<'a>) {
    state
        .halstead_maps
        .finalize(&mut state.space.metrics.halstead);
    T::Mi::compute(
        &state.space.metrics.loc,
        &state.space.metrics.cyclomatic,
        &state.space.metrics.halstead,
        &mut state.space.metrics.mi,
    );
}

fn finalize<'a, T: ParserTrait>(state_stack: &mut Vec<State<'a>>, diff_level: usize) {
    for _ in 0..diff_level {
        if state_stack.len() <= 1 {
            break;
        }

        let mut state = state_stack.pop().unwrap();
        compute_halstead_and_mi::<T>(&mut state);

        let mut last_state = state_stack.last_mut().unwrap();
        last_state.halstead_maps.merge(&state.halstead_maps);
        compute_halstead_and_mi::<T>(&mut last_state);

        // Merge function spaces
        last_state.space.metrics.merge(&state.space.metrics);
        last_state.space.spaces.push(state.space);
    }
}

#[derive(Debug, Clone)]
struct State<'a> {
    space: FuncSpace,
    halstead_maps: HalsteadMaps<'a>,
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
pub fn metrics<'a, T: ParserTrait>(parser: &'a T, path: &'a PathBuf) -> Option<FuncSpace> {
    let code = parser.get_code();
    let node = parser.get_root();
    let mut cursor = node.object().walk();
    let mut stack = Vec::new();
    let mut children = Vec::new();
    let mut state_stack: Vec<State> = Vec::new();
    let mut last_level = 0;

    stack.push((node, 0));

    while let Some((node, level)) = stack.pop() {
        if level < last_level {
            finalize::<T>(&mut state_stack, last_level - level);
            last_level = level;
        }

        let kind = T::Getter::get_space_kind(&node);

        let func_space = T::Checker::is_func(&node) || T::Checker::is_func_space(&node);
        let unit = kind == SpaceKind::Unit;

        let new_level = if func_space {
            let state = State {
                space: FuncSpace::new::<T::Getter>(&node, code, kind),
                halstead_maps: HalsteadMaps::new(),
            };
            state_stack.push(state);
            last_level = level + 1;
            last_level
        } else {
            level
        };

        if let Some(state) = state_stack.last_mut() {
            let last = &mut state.space;
            T::Cyclomatic::compute(&node, &mut last.metrics.cyclomatic);
            T::Halstead::compute(&node, code, &mut state.halstead_maps);
            T::Loc::compute(&node, &mut last.metrics.loc, func_space, unit);
            T::Nom::compute(&node, &mut last.metrics.nom);
            T::NArgs::compute(&node, &mut last.metrics.nargs);
            T::Exit::compute(&node, &mut last.metrics.nexits);
        }

        cursor.reset(node.object());
        if cursor.goto_first_child() {
            loop {
                children.push((Node::new(cursor.node()), new_level));
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
            for child in children.drain(..).rev() {
                stack.push(child);
            }
        }
    }

    finalize::<T>(&mut state_stack, std::usize::MAX);

    state_stack.pop().map(|mut state| {
        state.space.name = path.to_str().map(|name| name.to_string());
        state.space
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
