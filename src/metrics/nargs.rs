use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::*;

/// The `NArgs` metric.
///
/// This metric counts the number of arguments
/// of functions/closures.
#[derive(Debug, Clone)]
pub struct Stats {
    fn_nargs: usize,
    closure_nargs: usize,
    fn_nargs_sum: usize,
    closure_nargs_sum: usize,
    fn_nargs_min: usize,
    closure_nargs_min: usize,
    fn_nargs_max: usize,
    closure_nargs_max: usize,
    total_functions: usize,
    total_closures: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            fn_nargs: 0,
            closure_nargs: 0,
            fn_nargs_sum: 0,
            closure_nargs_sum: 0,
            fn_nargs_min: usize::MAX,
            closure_nargs_min: usize::MAX,
            fn_nargs_max: 0,
            closure_nargs_max: 0,
            total_functions: 0,
            total_closures: 0,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("nargs", 10)?;
        st.serialize_field("total_functions", &self.fn_args_sum())?;
        st.serialize_field("total_closures", &self.closure_args_sum())?;
        st.serialize_field("average_functions", &self.fn_args_average())?;
        st.serialize_field("average_closures", &self.closure_args_average())?;
        st.serialize_field("total", &self.nargs_total())?;
        st.serialize_field("average", &self.nargs_average())?;
        st.serialize_field("functions_min", &self.fn_args_min())?;
        st.serialize_field("functions_max", &self.fn_args_max())?;
        st.serialize_field("closures_min", &self.closure_args_min())?;
        st.serialize_field("closures_max", &self.closure_args_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_functions: {}, total_closures: {}, average_functions: {}, average_closures: {}, total: {}, average: {}, functions_min: {}, functions_max: {}, closures_min: {}, closures_max: {}",
            self.fn_args(),
            self.closure_args(),
            self.fn_args_average(),
            self.closure_args_average(),
            self.nargs_total(),
            self.nargs_average(),
            self.fn_args_min(),
            self.fn_args_max(),
            self.closure_args_min(),
            self.closure_args_max()
        )
    }
}

impl Stats {
    /// Merges a second `NArgs` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.closure_nargs_min = self.closure_nargs_min.min(other.closure_nargs_min);
        self.closure_nargs_max = self.closure_nargs_max.max(other.closure_nargs_max);
        self.fn_nargs_min = self.fn_nargs_min.min(other.fn_nargs_min);
        self.fn_nargs_max = self.fn_nargs_max.max(other.fn_nargs_max);
        self.fn_nargs_sum += other.fn_nargs_sum;
        self.closure_nargs_sum += other.closure_nargs_sum;
    }

    /// Returns the number of function arguments in a space.
    #[inline(always)]
    pub fn fn_args(&self) -> f64 {
        self.fn_nargs as f64
    }

    /// Returns the number of closure arguments in a space.
    #[inline(always)]
    pub fn closure_args(&self) -> f64 {
        self.closure_nargs as f64
    }

    /// Returns the number of function arguments sum in a space.
    #[inline(always)]
    pub fn fn_args_sum(&self) -> f64 {
        self.fn_nargs_sum as f64
    }

    /// Returns the number of closure arguments sum in a space.
    #[inline(always)]
    pub fn closure_args_sum(&self) -> f64 {
        self.closure_nargs_sum as f64
    }

    /// Returns the average number of functions arguments in a space.
    #[inline(always)]
    pub fn fn_args_average(&self) -> f64 {
        self.fn_nargs_sum as f64 / self.total_functions.max(1) as f64
    }

    /// Returns the average number of closures arguments in a space.
    #[inline(always)]
    pub fn closure_args_average(&self) -> f64 {
        self.closure_nargs_sum as f64 / self.total_closures.max(1) as f64
    }

    /// Returns the total number of arguments of each function and
    /// closure in a space.
    #[inline(always)]
    pub fn nargs_total(&self) -> f64 {
        self.fn_args_sum() + self.closure_args_sum()
    }

    /// Returns the `NArgs` metric average value
    ///
    /// This value is computed dividing the `NArgs` value
    /// for the total number of functions/closures in a space.
    #[inline(always)]
    pub fn nargs_average(&self) -> f64 {
        self.nargs_total() / (self.total_functions + self.total_closures).max(1) as f64
    }
    /// Returns the minimum number of function arguments in a space.
    #[inline(always)]
    pub fn fn_args_min(&self) -> f64 {
        self.fn_nargs_min as f64
    }
    /// Returns the maximum number of function arguments in a space.
    #[inline(always)]
    pub fn fn_args_max(&self) -> f64 {
        self.fn_nargs_max as f64
    }
    /// Returns the minimum number of closure arguments in a space.
    #[inline(always)]
    pub fn closure_args_min(&self) -> f64 {
        self.closure_nargs_min as f64
    }
    /// Returns the maximum number of closure arguments in a space.
    #[inline(always)]
    pub fn closure_args_max(&self) -> f64 {
        self.closure_nargs_max as f64
    }
    #[inline(always)]
    pub(crate) fn compute_sum(&mut self) {
        self.closure_nargs_sum += self.closure_nargs;
        self.fn_nargs_sum += self.fn_nargs;
    }
    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        self.closure_nargs_min = self.closure_nargs_min.min(self.closure_nargs);
        self.closure_nargs_max = self.closure_nargs_max.max(self.closure_nargs);
        self.fn_nargs_min = self.fn_nargs_min.min(self.fn_nargs);
        self.fn_nargs_max = self.fn_nargs_max.max(self.fn_nargs);
        self.compute_sum();
    }
    pub(crate) fn finalize(&mut self, total_functions: usize, total_closures: usize) {
        self.total_functions = total_functions;
        self.total_closures = total_closures;
    }
}

#[inline(always)]
fn compute_args<T: Checker>(node: &Node, nargs: &mut usize) {
    if let Some(params) = node.object().child_by_field_name("parameters") {
        let node_params = Node::new(params);
        node_params.act_on_child(&mut |n| {
            if !T::is_non_arg(n) {
                *nargs += 1;
            }
        });
    }
}

#[doc(hidden)]
pub trait NArgs
where
    Self: Checker,
    Self: std::marker::Sized,
{
    fn compute(node: &Node, stats: &mut Stats) {
        if Self::is_func(node) {
            compute_args::<Self>(node, &mut stats.fn_nargs);
            return;
        }

        if Self::is_closure(node) {
            compute_args::<Self>(node, &mut stats.closure_nargs);
        }
    }
}

impl NArgs for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if Self::is_func(node) {
            if let Some(declarator) = node.object().child_by_field_name("declarator") {
                let new_node = Node::new(declarator);
                compute_args::<Self>(&new_node, &mut stats.fn_nargs);
            }
            return;
        }

        if Self::is_closure(node) {
            if let Some(declarator) = node.object().child_by_field_name("declarator") {
                let new_node = Node::new(declarator);
                compute_args::<Self>(&new_node, &mut stats.closure_nargs);
            }
        }
    }
}

impl NArgs for MozjsCode {}
impl NArgs for JavascriptCode {}
impl NArgs for TypescriptCode {}
impl NArgs for TsxCode {}
impl NArgs for PreprocCode {}
impl NArgs for CcommentCode {}
impl NArgs for RustCode {}
impl NArgs for PythonCode {}
impl NArgs for JavaCode {}
impl NArgs for KotlinCode {}

#[cfg(test)]
mod tests {
    use crate::tools::check_metrics;

    use super::*;

    #[test]
    fn python_no_functions_and_closures() {
        check_metrics::<PythonParser>("a = 42", "foo.py", |metric| {
            // 0 functions + 0 closures
            insta::assert_json_snapshot!(
                metric.nargs,
                @r###"
                    {
                      "total_functions": 0.0,
                      "total_closures": 0.0,
                      "average_functions": 0.0,
                      "average_closures": 0.0,
                      "total": 0.0,
                      "average": 0.0,
                      "functions_min": 0.0,
                      "functions_max": 0.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn rust_no_functions_and_closures() {
        check_metrics::<RustParser>("let a = 42;", "foo.rs", |metric| {
            // 0 functions + 0 closures
            insta::assert_json_snapshot!(
                metric.nargs,
                @r###"
                    {
                      "total_functions": 0.0,
                      "total_closures": 0.0,
                      "average_functions": 0.0,
                      "average_closures": 0.0,
                      "total": 0.0,
                      "average": 0.0,
                      "functions_min": 0.0,
                      "functions_max": 0.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn cpp_no_functions_and_closures() {
        check_metrics::<CppParser>("int a = 42;", "foo.cpp", |metric| {
            // 0 functions + 0 closures
            insta::assert_json_snapshot!(
                metric.nargs,
                @r###"
                    {
                      "total_functions": 0.0,
                      "total_closures": 0.0,
                      "average_functions": 0.0,
                      "average_closures": 0.0,
                      "total": 0.0,
                      "average": 0.0,
                      "functions_min": 0.0,
                      "functions_max": 0.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn javascript_no_functions_and_closures() {
        check_metrics::<JavascriptParser>("var a = 42;", "foo.js", |metric| {
            // 0 functions + 0 closures
            insta::assert_json_snapshot!(
                metric.nargs,
                @r###"
                    {
                      "total_functions": 0.0,
                      "total_closures": 0.0,
                      "average_functions": 0.0,
                      "average_closures": 0.0,
                      "total": 0.0,
                      "average": 0.0,
                      "functions_min": 0.0,
                      "functions_max": 0.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn python_single_function() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                 if a:
                     return a",
            "foo.py",
            |metric| {
                // 1 function
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 2.0,
                      "total_closures": 0.0,
                      "average_functions": 2.0,
                      "average_closures": 0.0,
                      "total": 2.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_single_function() {
        check_metrics::<RustParser>(
            "fn f(a: bool, b: usize) {
                 if a {
                     return a;
                }
             }",
            "foo.rs",
            |metric| {
                // 1 function
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 2.0,
                      "total_closures": 0.0,
                      "average_functions": 2.0,
                      "average_closures": 0.0,
                      "total": 2.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_single_function() {
        check_metrics::<CppParser>(
            "int f(int a, int b) {
                 if (a) {
                     return a;
                }
             }",
            "foo.c",
            |metric| {
                // 1 function
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 2.0,
                      "total_closures": 0.0,
                      "average_functions": 2.0,
                      "average_closures": 0.0,
                      "total": 2.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn javascript_single_function() {
        check_metrics::<JavascriptParser>(
            "function f(a, b) {
                 return a * b;
             }",
            "foo.js",
            |metric| {
                // 1 function
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 2.0,
                      "total_closures": 0.0,
                      "average_functions": 2.0,
                      "average_closures": 0.0,
                      "total": 2.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_single_lambda() {
        check_metrics::<PythonParser>("bar = lambda a: True", "foo.py", |metric| {
            // 1 lambda
            insta::assert_json_snapshot!(
                metric.nargs,
                @r###"
                    {
                      "total_functions": 0.0,
                      "total_closures": 1.0,
                      "average_functions": 0.0,
                      "average_closures": 1.0,
                      "total": 1.0,
                      "average": 1.0,
                      "functions_min": 0.0,
                      "functions_max": 0.0,
                      "closures_min": 1.0,
                      "closures_max": 1.0
                    }"###
            );
        });
    }

    #[test]
    fn rust_single_closure() {
        check_metrics::<RustParser>("let bar = |i: i32| -> i32 { i + 1 };", "foo.rs", |metric| {
            // 1 lambda
            insta::assert_json_snapshot!(
                metric.nargs,
                @r###"
                    {
                      "total_functions": 0.0,
                      "total_closures": 1.0,
                      "average_functions": 0.0,
                      "average_closures": 1.0,
                      "total": 1.0,
                      "average": 1.0,
                      "functions_min": 0.0,
                      "functions_max": 0.0,
                      "closures_min": 0.0,
                      "closures_max": 1.0
                    }"###
            );
        });
    }

    #[test]
    fn cpp_single_lambda() {
        check_metrics::<CppParser>(
            "auto bar = [](int x, int y) -> int { return x + y; };",
            "foo.cpp",
            |metric| {
                // 1 lambda
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 0.0,
                      "total_closures": 2.0,
                      "average_functions": 0.0,
                      "average_closures": 2.0,
                      "total": 2.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 0.0,
                      "closures_min": 2.0,
                      "closures_max": 2.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn javascript_single_closure() {
        check_metrics::<JavascriptParser>("function (a, b) {return a + b};", "foo.js", |metric| {
            // 1 lambda
            insta::assert_json_snapshot!(
                metric.nargs,
                @r###"
                    {
                      "total_functions": 0.0,
                      "total_closures": 2.0,
                      "average_functions": 0.0,
                      "average_closures": 2.0,
                      "total": 2.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 0.0,
                      "closures_min": 0.0,
                      "closures_max": 2.0
                    }"###
            );
        });
    }

    #[test]
    fn python_functions() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                 if a:
                     return a
            def f(a, b):
                 if b:
                     return b",
            "foo.py",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 4.0,
                      "total_closures": 0.0,
                      "average_functions": 2.0,
                      "average_closures": 0.0,
                      "total": 4.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );

        check_metrics::<PythonParser>(
            "def f(a, b):
                 if a:
                     return a
            def f(a, b, c):
                 if b:
                     return b",
            "foo.py",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 5.0,
                      "total_closures": 0.0,
                      "average_functions": 2.5,
                      "average_closures": 0.0,
                      "total": 5.0,
                      "average": 2.5,
                      "functions_min": 0.0,
                      "functions_max": 3.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_functions() {
        check_metrics::<RustParser>(
            "fn f(a: bool, b: usize) {
                 if a {
                     return a;
                }
             }
             fn f1(a: bool, b: usize) {
                 if a {
                     return a;
                }
             }",
            "foo.rs",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 4.0,
                      "total_closures": 0.0,
                      "average_functions": 2.0,
                      "average_closures": 0.0,
                      "total": 4.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );

        check_metrics::<RustParser>(
            "fn f(a: bool, b: usize) {
                 if a {
                     return a;
                }
             }
             fn f1(a: bool, b: usize, c: usize) {
                 if a {
                     return a;
                }
             }",
            "foo.rs",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 5.0,
                      "total_closures": 0.0,
                      "average_functions": 2.5,
                      "average_closures": 0.0,
                      "total": 5.0,
                      "average": 2.5,
                      "functions_min": 0.0,
                      "functions_max": 3.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_functions() {
        check_metrics::<CppParser>(
            "int f(int a, int b) {
                 if (a) {
                     return a;
                }
             }
             int f1(int a, int b) {
                 if (a) {
                     return a;
                }
             }",
            "foo.c",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 4.0,
                      "total_closures": 0.0,
                      "average_functions": 2.0,
                      "average_closures": 0.0,
                      "total": 4.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );

        check_metrics::<CppParser>(
            "int f(int a, int b) {
                 if (a) {
                     return a;
                }
             }
             int f1(int a, int b, int c) {
                 if (a) {
                     return a;
                }
             }",
            "foo.c",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 5.0,
                      "total_closures": 0.0,
                      "average_functions": 2.5,
                      "average_closures": 0.0,
                      "total": 5.0,
                      "average": 2.5,
                      "functions_min": 0.0,
                      "functions_max": 3.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn javascript_functions() {
        check_metrics::<JavascriptParser>(
            "function f(a, b) {
                 return a * b;
             }
             function f1(a, b) {
                 return a * b;
             }",
            "foo.js",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 4.0,
                      "total_closures": 0.0,
                      "average_functions": 2.0,
                      "average_closures": 0.0,
                      "total": 4.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );

        check_metrics::<JavascriptParser>(
            "function f(a, b) {
                 return a * b;
             }
             function f1(a, b, c) {
                 return a * b;
             }",
            "foo.js",
            |metric| {
                // 2 functions
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 5.0,
                      "total_closures": 0.0,
                      "average_functions": 2.5,
                      "average_closures": 0.0,
                      "total": 5.0,
                      "average": 2.5,
                      "functions_min": 0.0,
                      "functions_max": 3.0,
                      "closures_min": 0.0,
                      "closures_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_nested_functions() {
        check_metrics::<PythonParser>(
            "def f(a, b):
                 def foo(a):
                     if a:
                         return 1
                 bar = lambda a: lambda b: b or True or True
                 return bar(foo(a))(a)",
            "foo.py",
            |metric| {
                // 2 functions + 2 lambdas = 4
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 3.0,
                      "total_closures": 2.0,
                      "average_functions": 1.5,
                      "average_closures": 1.0,
                      "total": 5.0,
                      "average": 1.25,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 2.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_nested_functions() {
        check_metrics::<RustParser>(
            "fn f(a: i32, b: i32) -> i32 {
                 fn foo(a: i32) -> i32 {
                     return a;
                 }
                 let bar = |a: i32, b: i32| -> i32 { a + 1 };
                 let bar1 = |b: i32| -> i32 { b + 1 };
                 return bar(foo(a), a);
             }",
            "foo.rs",
            |metric| {
                // 2 functions + 2 lambdas = 4
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 3.0,
                      "total_closures": 3.0,
                      "average_functions": 1.5,
                      "average_closures": 1.5,
                      "total": 6.0,
                      "average": 1.5,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 2.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_nested_functions() {
        check_metrics::<CppParser>(
            "int f(int a, int b, int c) {
                 auto foo = [](int x) -> int { return x; };
                 auto bar = [](int x, int y) -> int { return x + y; };
                 return bar(foo(a), a);
             }",
            "foo.cpp",
            |metric| {
                // 1 functions + 2 lambdas = 3
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 3.0,
                      "total_closures": 3.0,
                      "average_functions": 3.0,
                      "average_closures": 1.5,
                      "total": 6.0,
                      "average": 2.0,
                      "functions_min": 0.0,
                      "functions_max": 3.0,
                      "closures_min": 0.0,
                      "closures_max": 3.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn javascript_nested_functions() {
        check_metrics::<JavascriptParser>(
            "function f(a, b) {
                 function foo(a, c) {
                     return a;
                 }
                 var bar = function (a, b) {return a + b};
                 function (a) {return a};
                 return bar(foo(a), a);
             }",
            "foo.js",
            |metric| {
                // 3 functions + 1 lambdas = 4
                insta::assert_json_snapshot!(
                    metric.nargs,
                    @r###"
                    {
                      "total_functions": 6.0,
                      "total_closures": 1.0,
                      "average_functions": 2.0,
                      "average_closures": 1.0,
                      "total": 7.0,
                      "average": 1.75,
                      "functions_min": 0.0,
                      "functions_max": 2.0,
                      "closures_min": 0.0,
                      "closures_max": 1.0
                    }"###
                );
            },
        );
    }
}
