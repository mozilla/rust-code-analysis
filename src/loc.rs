use crate::checker::Checker;
use fxhash::FxHashSet;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;
use tree_sitter::Node;

use crate::*;

#[derive(Debug, Default)]
pub struct Stats {
    start: usize,
    end: usize,
    unit: bool,
    lines: FxHashSet<usize>,
    comment_lines: FxHashSet<usize>,
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("loc", 3)?;
        st.serialize_field("sloc", &self.sloc())?;
        st.serialize_field("lloc", &self.lloc())?;
        st.serialize_field("cloc", &self.cloc())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sloc: {}, lloc: {}, cloc: {}",
            self.sloc(),
            self.lloc(),
            self.cloc()
        )
    }
}

impl Stats {
    pub fn merge(&mut self, other: &Stats) {
        for l in other.lines.iter() {
            self.lines.insert(*l);
        }

        for c in other.comment_lines.iter() {
            self.comment_lines.insert(*c);
        }
    }

    #[inline(always)]
    pub fn sloc(&self) -> f64 {
        // The if construct is needed to count the line that represents
        // the signature of a function in a function space
        let sloc = if self.unit {
            self.end - self.start
        } else {
            (self.end - self.start) + 1
        };
        sloc as f64
    }

    #[inline(always)]
    pub fn lloc(&self) -> f64 {
        self.lines.len() as f64
    }

    #[inline(always)]
    pub fn cloc(&self) -> f64 {
        // Comments are counted regardless of their placement
        // https://en.wikipedia.org/wiki/Source_lines_of_code
        self.comment_lines.len() as f64
    }
}

pub trait Loc
where
    Self: Checker,
{
    fn compute(
        _node: &Node,
        _code: &[u8],
        _stats: &mut Stats,
        _is_func_space: bool,
        _is_unit: bool,
    ) {
    }
}

#[inline(always)]
fn init(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) -> usize {
    let start = node.start_position().row;

    if is_func_space {
        let end = node.end_position().row;

        stats.start = start;
        stats.end = end;
        stats.unit = is_unit;
    }
    start
}

impl Loc for PythonCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Python::*;

        let start = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | DQUOTE2 | ExpressionStatement | Block | Module => {}
            Comment => {
                stats.comment_lines.insert(start);
            }
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for MozjsCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Mozjs::*;

        let start = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | ExpressionStatement | StatementBlock | Program => {}
            Comment => {
                stats.comment_lines.insert(start);
            }
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for JavascriptCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Javascript::*;

        let start = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | ExpressionStatement | StatementBlock | Program => {}
            Comment => {
                stats.comment_lines.insert(start);
            }
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for TypescriptCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Typescript::*;

        let start = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | ExpressionStatement | StatementBlock | Program => {}
            Comment => {
                stats.comment_lines.insert(start);
            }
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for TsxCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Tsx::*;

        let start = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | ExpressionStatement | StatementBlock | Program => {}
            Comment => {
                stats.comment_lines.insert(start);
            }
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for RustCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Rust::*;

        let start = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            StringLiteral | RawStringLiteral | ExpressionStatement | Block | SourceFile => {}
            LineComment | BlockComment => {
                stats.comment_lines.insert(start);
            }
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for CppCode {
    fn compute(node: &Node, _code: &[u8], stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Cpp::*;

        let start = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            RawStringLiteral | StringLiteral | ExpressionStatement | CompoundStatement
            | LabeledStatement | DeclarationList | FieldDeclarationList | TranslationUnit => {}
            Comment => {
                stats.comment_lines.insert(start);
            }
            _ => {
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for PreprocCode {}
impl Loc for CcommentCode {}
impl Loc for CSharpCode {}
impl Loc for JavaCode {}
impl Loc for GoCode {}
impl Loc for CssCode {}
impl Loc for HtmlCode {}
