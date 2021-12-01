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
        let mut st = serializer.serialize_struct("nargs", 6)?;
        st.serialize_field("total_functions", &self.fn_args_sum())?;
        st.serialize_field("total_closures", &self.closure_args_sum())?;
        st.serialize_field("average_functions", &self.fn_args_average())?;
        st.serialize_field("average_closures", &self.closure_args_average())?;
        st.serialize_field("total", &self.nargs_total())?;
        st.serialize_field("average", &self.nargs_average())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "total_functions: {}, total_closures: {}, average_functions: {}, average_closures: {}, total: {}, average: {}",
            self.fn_args(),
            self.closure_args(),
            self.fn_args_average(),
            self.closure_args_average(),
            self.nargs_total(),
            self.nargs_average()
        )
    }
}

impl Stats {
    /// Merges a second `NArgs` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
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
    pub fn compute_sum(&mut self)  {
        self.closure_nargs_sum += self.closure_nargs;
        self.fn_nargs_sum += self.fn_nargs;
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn python_no_functions_and_closures() {
        check_metrics!(
            "a = 42",
            "foo.py",
            PythonParser,
            nargs,
            [
                (fn_args_sum, 0, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 0, usize)
            ],
            [
                (fn_args_average, 0.0),
                (closure_args_average, 0.0),
                (nargs_average, 0.0)
            ] // 0 functions + 0 closures = 0
        );
    }

    #[test]
    fn rust_no_functions_and_closures() {
        check_metrics!(
            "let a = 42;",
            "foo.rs",
            RustParser,
            nargs,
            [
                (fn_args_sum, 0, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 0, usize)
            ],
            [
                (fn_args_average, 0.0),
                (closure_args_average, 0.0),
                (nargs_average, 0.0)
            ] // 0 functions + 0 closures = 0
        );
    }

    #[test]
    fn cpp_no_functions_and_closures() {
        check_metrics!(
            "int a = 42;",
            "foo.cpp",
            CppParser,
            nargs,
            [
                (fn_args_sum, 0, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 0, usize)
            ],
            [
                (fn_args_average, 0.0),
                (closure_args_average, 0.0),
                (nargs_average, 0.0)
            ] // 0 functions + 0 closures = 0
        );
    }

    #[test]
    fn javascript_no_functions_and_closures() {
        check_metrics!(
            "var a = 42;",
            "foo.js",
            JavascriptParser,
            nargs,
            [
                (fn_args_sum, 0, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 0, usize)
            ],
            [
                (fn_args_average, 0.0),
                (closure_args_average, 0.0),
                (nargs_average, 0.0)
            ] // 0 functions + 0 closures = 0
        );
    }

    #[test]
    fn python_single_function() {
        check_metrics!(
            "def f(a, b):
                 if a:
                     return a",
            "foo.py",
            PythonParser,
            nargs,
            [
                (fn_args_sum, 2, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 2, usize)
            ],
            [
                (fn_args_average, 2.0),
                (closure_args_average, 0.0),
                (nargs_average, 2.0)
            ] // 1 function
        );
    }

    #[test]
    fn rust_single_function() {
        check_metrics!(
            "fn f(a: bool, b: usize) {
                 if a {
                     return a;
                }
             }",
            "foo.rs",
            RustParser,
            nargs,
            [
                (fn_args_sum, 2, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 2, usize)
            ],
            [
                (fn_args_average, 2.0),
                (closure_args_average, 0.0),
                (nargs_average, 2.0)
            ] // 1 function
        );
    }

    #[test]
    fn c_single_function() {
        check_metrics!(
            "int f(int a, int b) {
                 if (a) {
                     return a;
                }
             }",
            "foo.c",
            CppParser,
            nargs,
            [
                (fn_args_sum, 2, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 2, usize)
            ],
            [
                (fn_args_average, 2.0),
                (closure_args_average, 0.0),
                (nargs_average, 2.0)
            ] // 1 function
        );
    }

    #[test]
    fn javascript_single_function() {
        check_metrics!(
            "function f(a, b) {
                 return a * b;
             }",
            "foo.js",
            JavascriptParser,
            nargs,
            [
                (fn_args_sum, 2, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 2, usize)
            ],
            [
                (fn_args_average, 2.0),
                (closure_args_average, 0.0),
                (nargs_average, 2.0)
            ] // 1 function
        );
    }

    #[test]
    fn python_single_lambda() {
        check_metrics!(
            "bar = lambda a: True",
            "foo.py",
            PythonParser,
            nargs,
            [
                (fn_args_sum, 0, usize),
                (closure_args_sum, 1, usize),
                (nargs_total, 1, usize)
            ],
            [
                (fn_args_average, 0.0),
                (closure_args_average, 1.0),
                (nargs_average, 1.0)
            ] // 1 lambda
        );
    }

    #[test]
    fn rust_single_closure() {
        check_metrics!(
            "let bar = |i: i32| -> i32 { i + 1 };",
            "foo.rs",
            RustParser,
            nargs,
            [
                (fn_args_sum, 0, usize),
                (closure_args_sum, 1, usize),
                (nargs_total, 1, usize)
            ],
            [
                (fn_args_average, 0.0),
                (closure_args_average, 1.0),
                (nargs_average, 1.0)
            ] // 1 lambda
        );
    }

    #[test]
    fn cpp_single_lambda() {
        check_metrics!(
            "auto bar = [](int x, int y) -> int { return x + y; };",
            "foo.cpp",
            CppParser,
            nargs,
            [
                (fn_args_sum, 0, usize),
                (closure_args_sum, 2, usize),
                (nargs_total, 2, usize)
            ],
            [
                (fn_args_average, 0.0),
                (closure_args_average, 2.0),
                (nargs_average, 2.0)
            ] // 1 lambda
        );
    }

    #[test]
    fn javascript_single_closure() {
        check_metrics!(
            "function (a, b) {return a + b};",
            "foo.js",
            JavascriptParser,
            nargs,
            [
                (fn_args_sum, 0, usize),
                (closure_args_sum, 2, usize),
                (nargs_total, 2, usize)
            ],
            [
                (fn_args_average, 0.0),
                (closure_args_average, 2.0),
                (nargs_average, 2.0)
            ] // 1 lambda
        );
    }

    #[test]
    fn python_functions() {
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
            [
                (fn_args_sum, 4, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 4, usize)
            ],
            [
                (fn_args_average, 2.0),
                (closure_args_average, 0.0),
                (nargs_average, 2.0)
            ] // 2 functions
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
            [
                (fn_args_sum, 5, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 5, usize)
            ],
            [
                (fn_args_average, 2.5),
                (closure_args_average, 0.0),
                (nargs_average, 2.5)
            ] // 2 functions
        );
    }

    #[test]
    fn rust_functions() {
        check_metrics!(
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
            RustParser,
            nargs,
            [
                (fn_args_sum, 4, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 4, usize)
            ],
            [
                (fn_args_average, 2.0),
                (closure_args_average, 0.0),
                (nargs_average, 2.0)
            ] // 2 functions
        );

        check_metrics!(
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
            RustParser,
            nargs,
            [
                (fn_args_sum, 5, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 5, usize)
            ],
            [
                (fn_args_average, 2.5),
                (closure_args_average, 0.0),
                (nargs_average, 2.5)
            ] // 2 functions
        );
    }

    #[test]
    fn c_functions() {
        check_metrics!(
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
            CppParser,
            nargs,
            [
                (fn_args_sum, 4, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 4, usize)
            ],
            [
                (fn_args_average, 2.0),
                (closure_args_average, 0.0),
                (nargs_average, 2.0)
            ] // 2 functions
        );

        check_metrics!(
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
            CppParser,
            nargs,
            [
                (fn_args_sum, 5, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 5, usize)
            ],
            [
                (fn_args_average, 2.5),
                (closure_args_average, 0.0),
                (nargs_average, 2.5)
            ] // 2 functions
        );
    }

    #[test]
    fn javascript_functions() {
        check_metrics!(
            "function f(a, b) {
                 return a * b;
             }
             function f1(a, b) {
                 return a * b;
             }",
            "foo.js",
            JavascriptParser,
            nargs,
            [
                (fn_args_sum, 4, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 4, usize)
            ],
            [
                (fn_args_average, 2.0),
                (closure_args_average, 0.0),
                (nargs_average, 2.0)
            ] // 2 functions
        );

        check_metrics!(
            "function f(a, b) {
                 return a * b;
             }
             function f1(a, b, c) {
                 return a * b;
             }",
            "foo.js",
            JavascriptParser,
            nargs,
            [
                (fn_args_sum, 5, usize),
                (closure_args_sum, 0, usize),
                (nargs_total, 5, usize)
            ],
            [
                (fn_args_average, 2.5),
                (closure_args_average, 0.0),
                (nargs_average, 2.5)
            ] // 2 functions
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
            nargs,
            [
                (fn_args_sum, 3, usize),
                (closure_args_sum, 2, usize),
                (nargs_total, 5, usize)
            ],
            [
                (fn_args_average, 1.5),
                (closure_args_average, 1.0),
                (nargs_average, 1.25)
            ] // 2 functions + 2 lambdas = 4
        );
    }

    #[test]
    fn rust_nested_functions() {
        check_metrics!(
            "fn f(a: i32, b: i32) -> i32 {
                 fn foo(a: i32) -> i32 {
                     return a;
                 }
                 let bar = |a: i32, b: i32| -> i32 { a + 1 };
                 let bar1 = |b: i32| -> i32 { b + 1 };
                 return bar(foo(a), a);
             }",
            "foo.rs",
            RustParser,
            nargs,
            [
                (fn_args_sum, 3, usize),
                (closure_args_sum, 3, usize),
                (nargs_total, 6, usize)
            ],
            [
                (fn_args_average, 1.5),
                (closure_args_average, 1.5),
                (nargs_average, 1.5)
            ] // 2 functions + 2 lambdas = 4
        );
    }

    #[test]
    fn cpp_nested_functions() {
        check_metrics!(
            "int f(int a, int b, int c) {
                 auto foo = [](int x) -> int { return x; };
                 auto bar = [](int x, int y) -> int { return x + y; };
                 return bar(foo(a), a);
             }",
            "foo.cpp",
            CppParser,
            nargs,
            [
                (fn_args_sum, 3, usize),
                (closure_args_sum, 3, usize),
                (nargs_total, 6, usize)
            ],
            [
                (fn_args_average, 3.0),
                (closure_args_average, 1.5),
                (nargs_average, 2.0)
            ] // 1 function + 2 lambdas = 3
        );
    }

    #[test]
    fn javascript_nested_functions() {
        check_metrics!(
            "function f(a, b) {
                 function foo(a, c) {
                     return a;
                 }
                 var bar = function (a, b) {return a + b};
                 function (a) {return a};
                 return bar(foo(a), a);
             }",
            "foo.js",
            JavascriptParser,
            nargs,
            [
                (fn_args_sum, 6, usize),
                (closure_args_sum, 1, usize),
                (nargs_total, 7, usize)
            ],
            [
                (fn_args_average, 2.),
                (closure_args_average, 1.),
                (nargs_average, 1.75)
            ] // 3 functions + 1 lambdas = 4
        );
    }
}
