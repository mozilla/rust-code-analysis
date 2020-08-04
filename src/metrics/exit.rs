use serde::ser::Serializer;
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::*;

/// The `NExit` metric.
///
/// This metric counts the number of possible exit points
/// from a function/method.
#[derive(Debug)]
pub struct Stats {
    exit: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self { exit: 0 }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.exit())
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.exit)
    }
}

impl Stats {
    /// Merges a second `NExit` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.exit += other.exit;
    }

    /// Returns the `NExit` metric value
    pub fn exit(&self) -> f64 {
        self.exit as f64
    }
}

#[doc(hidden)]
pub trait Exit
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Exit for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Python::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

impl Exit for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Mozjs::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

impl Exit for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Javascript::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

impl Exit for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Typescript::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

impl Exit for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Tsx::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

impl Exit for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Rust::ReturnExpression = node.object().kind_id().into() {
            stats.exit += 1;
        } else if Self::is_func(node) && node.object().child_by_field_name("return_type").is_some()
        {
            stats.exit += 1;
        }
    }
}

impl Exit for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Cpp::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

impl Exit for PreprocCode {}
impl Exit for CcommentCode {}
impl Exit for CSharpCode {}
impl Exit for JavaCode {}
impl Exit for GoCode {}
impl Exit for CssCode {}
impl Exit for HtmlCode {}
