use serde::Serialize;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::checker::Checker;
use crate::node::Node;

use crate::cognitive::{self, Cognitive};
use crate::cyclomatic::{self, Cyclomatic};
use crate::exit::{self, Exit};
use crate::getter::Getter;
use crate::halstead::{self, Halstead, HalsteadMaps};
use crate::loc::{self, Loc};
use crate::mi::{self, Mi};
use crate::nargs::{self, NArgs};
use crate::nom::{self, Nom};

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
    pub nargs: nargs::Stats,
    /// `NExits` data
    pub nexits: exit::Stats,
    pub cognitive: cognitive::Stats,
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
            cognitive: cognitive::Stats::default(),
            cyclomatic: cyclomatic::Stats::default(),
            halstead: halstead::Stats::default(),
            loc: loc::Stats::default(),
            nom: nom::Stats::default(),
            mi: mi::Stats::default(),
            nargs: nargs::Stats::default(),
            nexits: exit::Stats::default(),
        }
    }
}

impl fmt::Display for CodeMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.nargs)?;
        writeln!(f, "{}", self.nexits)?;
        writeln!(f, "{}", self.cognitive)?;
        writeln!(f, "{}", self.cyclomatic)?;
        writeln!(f, "{}", self.halstead)?;
        writeln!(f, "{}", self.loc)?;
        writeln!(f, "{}", self.nom)?;
        write!(f, "{}", self.mi)
    }
}

impl CodeMetrics {
    pub fn merge(&mut self, other: &CodeMetrics) {
        self.cognitive.merge(&other.cognitive);
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
    /// If `None`, an error is occurred in parsing
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
            name: T::get_func_space_name(node, code).map(|name| name.to_string()),
            spaces: Vec::new(),
            metrics: CodeMetrics::default(),
            kind,
            start_line: start_position,
            end_line: end_position,
        }
    }
}

#[inline(always)]
fn compute_halstead_and_mi<T: ParserTrait>(state: &mut State) {
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

#[inline(always)]
fn compute_averages(state: &mut State) {
    let nom_functions = state.space.metrics.nom.functions_sum() as usize;
    let nom_closures = state.space.metrics.nom.closures_sum() as usize;
    let nom_total = state.space.metrics.nom.total() as usize;
    // Cognitive average
    state.space.metrics.cognitive.finalize(nom_total);
    // Nexit average
    state.space.metrics.nexits.finalize(nom_total);
    // Nargs average
    state
        .space
        .metrics
        .nargs
        .finalize(nom_functions, nom_closures);
}
#[inline(always)]
fn compute_minmax(state: &mut State) {
    state.space.metrics.cyclomatic.compute_minmax();
    state.space.metrics.nexits.compute_minmax();
    state.space.metrics.cognitive.compute_minmax();
    state.space.metrics.nargs.compute_minmax();
    state.space.metrics.nom.compute_sum();
}

fn finalize<T: ParserTrait>(state_stack: &mut Vec<State>, diff_level: usize) {
    if state_stack.is_empty() {
        return;
    }
    for _ in 0..diff_level {
        if state_stack.len() == 1 {
            let mut last_state = state_stack.last_mut().unwrap();
            compute_minmax(&mut last_state);
            compute_halstead_and_mi::<T>(&mut last_state);
            compute_averages(&mut last_state);
            break;
        } else {
            let mut state = state_stack.pop().unwrap();
            compute_minmax(&mut state);
            compute_halstead_and_mi::<T>(&mut state);
            compute_averages(&mut state);

            let mut last_state = state_stack.last_mut().unwrap();
            last_state.halstead_maps.merge(&state.halstead_maps);
            compute_halstead_and_mi::<T>(&mut last_state);

            // Merge function spaces
            last_state.space.metrics.merge(&state.space.metrics);
            last_state.space.spaces.push(state.space);
        }
    }
}

#[derive(Debug, Clone)]
struct State<'a> {
    space: FuncSpace,
    halstead_maps: HalsteadMaps<'a>,
}

/// Returns all function spaces data of a code. This function needs a parser to
/// be created a priori in order to work.
///
/// # Examples
///
/// ```
/// use std::path::Path;
///
/// use rust_code_analysis::{CppParser, metrics, ParserTrait};
///
/// let source_code = "int a = 42;";
///
/// // The path to a dummy file used to contain the source code
/// let path = Path::new("foo.c");
/// let source_as_vec = source_code.as_bytes().to_vec();
///
/// // The parser of the code, in this case a CPP parser
/// let parser = CppParser::new(source_as_vec, &path, None);
///
/// // Gets all function spaces data of the code contained in foo.c
/// metrics(&parser, &path).unwrap();
/// ```
pub fn metrics<'a, T: ParserTrait>(parser: &'a T, path: &'a Path) -> Option<FuncSpace> {
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
            T::Cognitive::compute(&node, &mut last.metrics.cognitive);
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
}

pub struct Metrics {
    _guard: (),
}

impl Callback for Metrics {
    type Res = std::io::Result<()>;
    type Cfg = MetricsCfg;

    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res {
        if let Some(space) = metrics(parser, &cfg.path) {
            dump_root(&space)
        } else {
            Ok(())
        }
    }
}
