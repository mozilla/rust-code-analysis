use serde::ser::Serializer;
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::*;

/// The `NArgs` metric.
///
/// This metric counts the number of arguments
/// of a function/method.
#[derive(Debug, Clone)]
pub struct Stats {
    nargs: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self { nargs: 0 }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.nargs())
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.nargs())
    }
}

impl Stats {
    /// Merges a second `NArgs` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.nargs += other.nargs;
    }

    /// Returns the `NArgs` metric value
    pub fn nargs(&self) -> f64 {
        self.nargs as f64
    }
}

#[doc(hidden)]
pub trait NArgs
where
    Self: Checker,
{
    fn compute(node: &Node, stats: &mut Stats) {
        if !Self::is_func(node) {
            return;
        }

        if let Some(params) = node.object().child_by_field_name("parameters") {
            let node_params = Node::new(params);
            node_params.act_on_child(&mut |n| {
                if !Self::is_non_arg(n) {
                    stats.nargs += 1;
                }
            });
        }
    }
}

impl NArgs for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if !Self::is_func(node) {
            return;
        }

        if let Some(declarator) = node.object().child_by_field_name("declarator") {
            if let Some(params) = declarator.child_by_field_name("parameters") {
                let node_params = Node::new(params);
                node_params.act_on_child(&mut |n| {
                    if !Self::is_non_arg(n) {
                        stats.nargs += 1;
                    }
                });
            }
        }
    }
}

impl NArgs for PythonCode {}
impl NArgs for MozjsCode {}
impl NArgs for JavascriptCode {}
impl NArgs for TypescriptCode {}
impl NArgs for TsxCode {}
impl NArgs for RustCode {}
impl NArgs for PreprocCode {}
impl NArgs for CcommentCode {}
impl NArgs for CSharpCode {}
impl NArgs for JavaCode {}
impl NArgs for GoCode {}
impl NArgs for CssCode {}
impl NArgs for HtmlCode {}
