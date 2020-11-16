use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::*;

/// The `NExit` metric.
///
/// This metric counts the number of possible exit points
/// from a function/method.
#[derive(Debug, Clone)]
pub struct Stats {
    exit: usize,
    total_space_functions: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            exit: 0,
            total_space_functions: 1,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("nexits", 2)?;
        st.serialize_field("sum", &self.exit())?;
        st.serialize_field("average", &self.exit_average())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sum: {}, average: {}", self.exit(), self.exit_average())
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

    /// Returns the `NExit` metric average value
    ///
    /// This value is computed dividing the `NExit` value
    /// for the total number of functions/closures in a space.
    pub fn exit_average(&self) -> f64 {
        self.exit() / self.total_space_functions as f64
    }

    pub(crate) fn finalize(&mut self, total_space_functions: usize) {
        self.total_space_functions = total_space_functions;
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
impl Exit for JavaCode {}
impl Exit for GoCode {}
impl Exit for HtmlCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_function_exit() {
        check_metrics!(
            "def f(a, b):
                 if a:
                     return a",
            "foo.py",
            PythonParser,
            nexits,
            [(exit, 1, usize)],
            [(exit_average, 1.0)] // 1 function
        );
    }

    #[test]
    fn test_functions_exit() {
        check_metrics!(
            "def f(a, b):
                 if a:
                     return a
            def f(a, b):
                 if b:
                     return b",
            "foo.py",
            PythonParser,
            nexits,
            [(exit, 2, usize)],
            [(exit_average, 1.0)] // 2 functions
        );
    }

    #[test]
    fn test_nested_functions_exit() {
        check_metrics!(
            "def f(a, b):
                 def foo(a):
                     if a:
                         return 1
                 bar = lambda a: lambda b: b or True or True
                 return bar(foo(a))(a)",
            "foo.py",
            PythonParser,
            nexits,
            [(exit, 2, usize)],
            [(exit_average, 0.5)] // 2 functions + 2 lambdas = 4
        );
    }
}
