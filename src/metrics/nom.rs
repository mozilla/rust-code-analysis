use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;

use crate::*;

/// The `Nom` metric suite.
#[derive(Default, Clone, Debug)]
pub struct Stats {
    functions: usize,
    closures: usize,
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("nom", 3)?;
        st.serialize_field("functions", &self.functions())?;
        st.serialize_field("closures", &self.closures())?;
        st.serialize_field("total", &self.total())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "functions: {}, \
             closures: {}, \
             total: {}",
            self.functions(),
            self.closures(),
            self.total(),
        )
    }
}

impl Stats {
    /// Merges a second `Nom` metric suite into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.functions += other.functions;
        self.closures += other.closures;
    }

    /// Counts the number of function definitions in a scope
    #[inline(always)]
    pub fn functions(&self) -> f64 {
        // Only function definitions are considered, not general declarations
        self.functions as f64
    }

    /// Counts the number of closures in a scope
    #[inline(always)]
    pub fn closures(&self) -> f64 {
        self.closures as f64
    }

    /// Returns the total number of function definitions and
    /// closures in a scope
    #[inline(always)]
    pub fn total(&self) -> f64 {
        self.functions() + self.closures()
    }
}

#[doc(hidden)]
pub trait Nom
where
    Self: Checker,
{
    fn compute(node: &Node, stats: &mut Stats) {
        if Self::is_func(node) {
            stats.functions += 1;
            return;
        }
        if Self::is_closure(node) {
            stats.closures += 1;
        }
    }
}

impl Nom for PythonCode {}
impl Nom for MozjsCode {}
impl Nom for JavascriptCode {}
impl Nom for TypescriptCode {}
impl Nom for TsxCode {}
impl Nom for RustCode {}
impl Nom for CppCode {}
impl Nom for PreprocCode {}
impl Nom for CcommentCode {}
impl Nom for JavaCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn python_nom() {
        check_metrics!(
            "def a():
                 pass
             def b():
                 pass
             def c():
                 pass
             x = lambda a : a + 42",
            "foo.py",
            PythonParser,
            nom,
            [
                (functions, 3, usize),
                (closures, 1, usize),
                (total, 4, usize)
            ]
        );
    }

    #[test]
    fn rust_nom() {
        check_metrics!(
            "mod A { fn foo() {}}
             mod B { fn foo() {}}
             let closure = |i: i32| -> i32 { i + 42 };",
            "foo.rs",
            RustParser,
            nom,
            [
                (functions, 2, usize),
                (closures, 1, usize),
                (total, 3, usize)
            ]
        );
    }

    #[test]
    fn c_nom() {
        check_metrics!(
            "int foo();

             int foo() {
                 return 0;
             }",
            "foo.c",
            CppParser,
            nom,
            [
                (functions, 1, usize),
                (closures, 0, usize),
                (total, 1, usize)
            ]
        );
    }

    #[test]
    fn cpp_nom() {
        check_metrics!(
            "struct A {
                 void foo(int) {}
                 void foo(double) {}
             };
             int b = [](int x) -> int { return x + 42; };",
            "foo.cpp",
            CppParser,
            nom,
            [
                (functions, 2, usize),
                (closures, 1, usize),
                (total, 3, usize)
            ]
        );
    }

    #[test]
    fn javascript_nom() {
        check_metrics!(
            "function f(a, b) {
                 function foo(a) {
                     return a;
                 }
                 var bar = (function () {
                     var counter = 0;
                     return function () {
                         counter += 1;
                         return counter
                     }
                 })();
                 return bar(foo(a), a);
             }",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 3, usize), // f, foo, bar
                (closures, 1, usize),  // return function ()
                (total, 4, usize)
            ]
        );
    }

    #[test]
    fn javascript_call_nom() {
        check_metrics!(
            "add_task(async function test_safe_mode() {
                 gAppInfo.inSafeMode = true;
             });",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 1, usize), // test_safe_mode
                (closures, 0, usize),
                (total, 1, usize)
            ]
        );
    }

    #[test]
    fn javascript_assignment_nom() {
        check_metrics!(
            "AnimationTest.prototype.enableDisplay = function(element) {};",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 1, usize),
                (closures, 0, usize),
                (total, 1, usize)
            ]
        );
    }

    #[test]
    fn javascript_labeled_nom() {
        check_metrics!(
            "toJSON: function() {
                 return this.inspect(true);
             }",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 1, usize),
                (closures, 0, usize),
                (total, 1, usize)
            ]
        );
    }

    #[test]
    fn javascript_labeled_arrow_nom() {
        check_metrics!(
            "const dimConverters = {
                pt: x => x,
             };",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 1, usize),
                (closures, 0, usize),
                (total, 1, usize)
            ]
        );
    }

    #[test]
    fn javascript_pair_nom() {
        check_metrics!(
            "return {
                 initialize: function(object) {
                     this._object = object.toObject();
                 },
             }",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 1, usize),
                (closures, 0, usize),
                (total, 1, usize)
            ]
        );
    }

    #[test]
    fn javascript_unnamed_nom() {
        check_metrics!(
            "Ajax.getTransport = Try.these(
                 function() {
                     return function(){ return new XMLHttpRequest()}
                 }
             );",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 0, usize),
                (closures, 2, usize),
                (total, 2, usize)
            ]
        );
    }

    #[test]
    fn javascript_arrow_nom() {
        check_metrics!(
            "var materials = [\"Hydrogen\"];
             materials.map(material => material.length);
             let add = (a, b)  => a + b;",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 1, usize), // add
                (closures, 1, usize),  // materials.map
                (total, 2, usize)
            ]
        );
    }

    #[test]
    fn javascript_arrow_assignment_nom() {
        check_metrics!(
            "sink.onPull = () => { };",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 1, usize),
                (closures, 0, usize),
                (total, 1, usize)
            ]
        );
    }

    #[test]
    fn javascript_arrow_new_nom() {
        check_metrics!(
            "const response = new Promise(resolve => channel.port1.onmessage = resolve);",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 0, usize),
                (closures, 1, usize),
                (total, 1, usize)
            ]
        );
    }

    #[test]
    fn javascript_arrow_call_nom() {
        check_metrics!(
            "let notDisabled = TestUtils.waitForCondition(
                 () => !backbutton.hasAttribute(\"disabled\")
             );",
            "foo.js",
            JavascriptParser,
            nom,
            [
                (functions, 0, usize),
                (closures, 1, usize),
                (total, 1, usize)
            ]
        );
    }
}
