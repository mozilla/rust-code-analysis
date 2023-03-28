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
    exit_sum: usize,
    total_space_functions: usize,
    exit_min: usize,
    exit_max: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            exit: 0,
            exit_sum: 0,
            total_space_functions: 1,
            exit_min: usize::MAX,
            exit_max: 0,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("nexits", 4)?;
        st.serialize_field("sum", &self.exit_sum())?;
        st.serialize_field("average", &self.exit_average())?;
        st.serialize_field("min", &self.exit_min())?;
        st.serialize_field("max", &self.exit_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {} min: {}, max: {}",
            self.exit_sum(),
            self.exit_average(),
            self.exit_min(),
            self.exit_max()
        )
    }
}

impl Stats {
    /// Merges a second `NExit` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.exit_max = self.exit_max.max(other.exit_max);
        self.exit_min = self.exit_min.min(other.exit_min);
        self.exit_sum += other.exit_sum;
    }

    /// Returns the `NExit` metric value
    pub fn exit(&self) -> f64 {
        self.exit as f64
    }
    /// Returns the `NExit` metric sum value
    pub fn exit_sum(&self) -> f64 {
        self.exit_sum as f64
    }
    /// Returns the `NExit` metric  minimum value
    pub fn exit_min(&self) -> f64 {
        self.exit_min as f64
    }
    /// Returns the `NExit` metric maximum value
    pub fn exit_max(&self) -> f64 {
        self.exit_max as f64
    }

    /// Returns the `NExit` metric average value
    ///
    /// This value is computed dividing the `NExit` value
    /// for the total number of functions/closures in a space.
    ///
    /// If there are no functions in a code, its value is `NAN`.
    pub fn exit_average(&self) -> f64 {
        self.exit_sum() / self.total_space_functions as f64
    }
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.exit_sum += self.exit;
    }
    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        self.exit_max = self.exit_max.max(self.exit);
        self.exit_min = self.exit_min.min(self.exit);
        self.compute_sum();
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
        if matches!(node.object().kind_id().into(), Python::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.object().kind_id().into(), Mozjs::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.object().kind_id().into(), Javascript::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.object().kind_id().into(), Typescript::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.object().kind_id().into(), Tsx::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.object().kind_id().into(), Rust::ReturnExpression)
            || Self::is_func(node) && node.object().child_by_field_name("return_type").is_some()
        {
            stats.exit += 1;
        }
    }
}

impl Exit for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.object().kind_id().into(), Cpp::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for JavaCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if matches!(node.object().kind_id().into(), Java::ReturnStatement) {
            stats.exit += 1;
        }
    }
}

impl Exit for KotlinCode {}

impl Exit for PreprocCode {}
impl Exit for CcommentCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn python_no_exit() {
        check_metrics!(
            "a = 42",
            "foo.py",
            PythonParser,
            nexits,
            [
                (exit_sum, 0, usize),
                (exit_min, 0, usize),
                (exit_max, 0, usize)
            ],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }

    #[test]
    fn rust_no_exit() {
        check_metrics!(
            "let a = 42;",
            "foo.rs",
            RustParser,
            nexits,
            [
                (exit_sum, 0, usize),
                (exit_min, 0, usize),
                (exit_max, 0, usize)
            ],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }

    #[test]
    fn c_no_exit() {
        check_metrics!(
            "int a = 42;",
            "foo.c",
            CppParser,
            nexits,
            [
                (exit_sum, 0, usize),
                (exit_min, 0, usize),
                (exit_max, 0, usize)
            ],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }

    #[test]
    fn javascript_no_exit() {
        check_metrics!(
            "var a = 42;",
            "foo.js",
            JavascriptParser,
            nexits,
            [
                (exit_sum, 0, usize),
                (exit_min, 0, usize),
                (exit_max, 0, usize)
            ],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }

    #[test]
    fn python_simple_function() {
        check_metrics!(
            "def f(a, b):
                 if a:
                     return a",
            "foo.py",
            PythonParser,
            nexits,
            [
                (exit_sum, 1, usize),
                (exit_min, 0, usize),
                (exit_max, 1, usize)
            ],
            [(exit_average, 1.0)] // 1 function
        );
    }

    #[test]
    fn python_more_functions() {
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
            [
                (exit_sum, 2, usize),
                (exit_min, 0, usize),
                (exit_max, 1, usize)
            ],
            [(exit_average, 1.0)] // 2 functions
        );
    }

    #[test]
    fn python_nested_functions() {
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
            [
                (exit_sum, 2, usize),
                (exit_min, 0, usize),
                (exit_max, 1, usize)
            ],
            [(exit_average, 0.5)] // 2 functions + 2 lambdas = 4
        );
    }

    #[test]
    fn java_no_exit() {
        check_metrics!(
            "int a = 42;",
            "foo.java",
            JavaParser,
            nexits,
            [
                (exit_sum, 0, usize),
                (exit_min, 0, usize),
                (exit_max, 0, usize)
            ],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }

    #[test]
    fn java_simple_function() {
        check_metrics!(
            "class A {
              public int sum(int x, int y) {
                return x + y;
              }
            }",
            "foo.java",
            JavaParser,
            nexits,
            [
                (exit_sum, 1, usize),
                (exit_min, 0, usize),
                (exit_max, 1, usize)
            ],
            [(exit_average, 1.0)] // 1 exit / 1 space
        );
    }

    #[test]
    fn java_split_function() {
        check_metrics!(
            "class A {
              public int multiply(int x, int y) {
                if(x == 0 || y == 0){
                    return 0;
                }
                return x * y;
              }
            }",
            "foo.java",
            JavaParser,
            nexits,
            [
                (exit_sum, 2, usize),
                (exit_min, 0, usize),
                (exit_max, 2, usize)
            ],
            [(exit_average, 2.0)] // 2 exit / space 1
        );
    }
}
