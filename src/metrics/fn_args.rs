use serde::ser::{SerializeStruct, Serializer};
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
    total_space_functions: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            nargs: 0,
            total_space_functions: 1,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("nargs", 2)?;
        st.serialize_field("sum", &self.nargs())?;
        st.serialize_field("average", &self.nargs_average())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {}",
            self.nargs(),
            self.nargs_average()
        )
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

    /// Returns the `NArgs` metric average value
    ///
    /// This value is computed dividing the `NArgs` value
    /// for the total number of functions/closures in a space.
    pub fn nargs_average(&self) -> f64 {
        self.nargs() / self.total_space_functions as f64
    }

    pub(crate) fn finalize(&mut self, total_space_functions: usize) {
        self.total_space_functions = total_space_functions;
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
impl NArgs for JavaCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_function_nargs() {
        check_metrics!(
            "def f(a, b):
                 if a:
                     return a",
            "foo.py",
            PythonParser,
            nargs,
            [(nargs, 2, usize)],
            [(nargs_average, 2.0)] // 1 function
        );
    }

    #[test]
    fn test_functions_nargs() {
        check_metrics!(
            "def f(a, b):
                 if a:
                     return a
            def f(a, b):
                 if b:
                     return b",
            "foo.py",
            PythonParser,
            nargs,
            [(nargs, 4, usize)],
            [(nargs_average, 2.0)] // 2 functions
        );

        check_metrics!(
            "def f(a, b):
                 if a:
                     return a
            def f(a, b, c):
                 if b:
                     return b",
            "foo.py",
            PythonParser,
            nargs,
            [(nargs, 5, usize)],
            [(nargs_average, 2.5)] // 2 functions
        );
    }

    #[test]
    fn test_nested_functions_nargs() {
        check_metrics!(
            "def f(a, b):
                 def foo(a):
                     if a:
                         return 1
                 bar = lambda a: lambda b: b or True or True
                 return bar(foo(a))(a)",
            "foo.py",
            PythonParser,
            nargs,
            // FIXME: Consider lambda arguments also
            [(nargs, 3, usize)],
            [(nargs_average, 0.75)] // 2 functions + 2 lambdas = 4
        );
    }
}
