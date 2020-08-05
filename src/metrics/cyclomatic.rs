use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::*;

/// The `Cyclomatic` metric.
#[derive(Debug, Clone)]
pub struct Stats {
    cyclomatic: f64,
    n: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            cyclomatic: 1.,
            n: 1,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("cyclomatic", 2)?;
        st.serialize_field("sum", &self.cyclomatic())?;
        st.serialize_field("average", &self.cyclomatic_average())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {}",
            self.cyclomatic(),
            self.cyclomatic_average()
        )
    }
}

impl Stats {
    /// Merges a second `Cyclomatic` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.cyclomatic += other.cyclomatic;
        self.n += other.n;
    }

    /// Returns the `Cyclomatic` metric value
    pub fn cyclomatic(&self) -> f64 {
        self.cyclomatic
    }

    /// Returns the `Cyclomatic` metric average value
    ///
    /// This value is computed dividing the `Cyclomatic` value for the
    /// number of spaces.
    pub fn cyclomatic_average(&self) -> f64 {
        self.cyclomatic() / self.n as f64
    }
}

#[doc(hidden)]
pub trait Cyclomatic
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Cyclomatic for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Python::*;

        match node.object().kind_id().into() {
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

impl Cyclomatic for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Mozjs::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Javascript::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Typescript::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Tsx::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Rust::*;

        match node.object().kind_id().into() {
            If | For | While | Loop | MatchArm | MatchArm2 | QMARK | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Cpp::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | ConditionalExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for PreprocCode {}
impl Cyclomatic for CcommentCode {}
impl Cyclomatic for CSharpCode {}
impl Cyclomatic for JavaCode {}
impl Cyclomatic for GoCode {}
impl Cyclomatic for CssCode {}
impl Cyclomatic for HtmlCode {}
