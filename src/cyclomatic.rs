use serde::ser::Serializer;
use serde::Serialize;
use std::fmt;
use tree_sitter::Node;

use crate::checker::Checker;
use crate::*;

#[derive(Debug)]
pub struct Stats {
    cyclomatic: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Self { cyclomatic: 1. }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.cyclomatic)
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.cyclomatic)
    }
}

impl Stats {
    pub fn merge(&mut self, _other: &Stats) {}

    pub fn cyclomatic(&self) -> f64 {
        self.cyclomatic
    }
}

pub trait Cyclomatic
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Cyclomatic for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Python::*;

        match node.kind_id().into() {
            If | Elif | For | While | Except | With | Assert | And | Or => {
                stats.cyclomatic += 1.;
            }
            Else => {
                if has_ancestors!(node, ForStatement | WhileStatement, ElseClause) {
                    stats.cyclomatic += 1.;
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for PreprocCode {}
impl Cyclomatic for CcommentCode {}
impl Cyclomatic for CCode {}
impl Cyclomatic for CppCode {}
impl Cyclomatic for CSharpCode {}
impl Cyclomatic for JavaCode {}
impl Cyclomatic for MozjsCode {}
impl Cyclomatic for JavascriptCode {}
impl Cyclomatic for TypescriptCode {}
impl Cyclomatic for TsxCode {}
impl Cyclomatic for GoCode {}
impl Cyclomatic for CssCode {}
impl Cyclomatic for HtmlCode {}
impl Cyclomatic for RustCode {}
