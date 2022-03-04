use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;

use crate::*;

/// The `Nom` metric suite.
#[derive(Clone, Debug)]
pub struct Stats {
    functions: usize,
    closures: usize,
    functions_sum: usize,
    closures_sum: usize,
    functions_min: usize,
    functions_max: usize,
    closures_min: usize,
    closures_max: usize,
    space_count: usize,
}
impl Default for Stats {
    fn default() -> Self {
        Self {
            functions: 0,
            closures: 0,
            functions_sum: 0,
            closures_sum: 0,
            functions_min: usize::MAX,
            functions_max: 0,
            closures_min: usize::MAX,
            closures_max: 0,
            space_count: 1,
        }
    }
}
impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("nom", 10)?;
        st.serialize_field("functions", &self.functions_sum())?;
        st.serialize_field("closures", &self.closures_sum())?;
        st.serialize_field("functions_average", &self.functions_average())?;
        st.serialize_field("closures_average", &self.closures_average())?;
        st.serialize_field("total", &self.total())?;
        st.serialize_field("average", &self.average())?;
        st.serialize_field("functions_min", &self.functions_min())?;
        st.serialize_field("functions_max", &self.functions_max())?;
        st.serialize_field("closures_min", &self.closures_min())?;
        st.serialize_field("closures_max", &self.closures_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "functions: {}, \
             closures: {}, \
             functions_average: {}, \
             closures_average: {}, \
             total: {} \
             average: {} \
             functions_min: {} \
             functions_max: {} \
             closures_min: {} \
             closures_max: {}",
            self.functions_sum(),
            self.closures_sum(),
            self.functions_average(),
            self.closures_average(),
            self.total(),
            self.average(),
            self.functions_min(),
            self.functions_max(),
            self.closures_min(),
            self.closures_max(),
        )
    }
}

impl Stats {
    /// Merges a second `Nom` metric suite into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.functions_min = self.functions_min.min(other.functions_min);
        self.functions_max = self.functions_max.max(other.functions_max);
        self.closures_min = self.closures_min.min(other.closures_min);
        self.closures_max = self.closures_max.max(other.closures_max);
        self.functions_sum += other.functions_sum;
        self.closures_sum += other.closures_sum;
        self.space_count += other.space_count;
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

    /// Return the sum metric for functions
    #[inline(always)]
    pub fn functions_sum(&self) -> f64 {
        // Only function definitions are considered, not general declarations
        self.functions_sum as f64
    }

    /// Return the sum metric for closures
    #[inline(always)]
    pub fn closures_sum(&self) -> f64 {
        self.closures_sum as f64
    }

    /// Returns the average number of function definitions over all spaces
    #[inline(always)]
    pub fn functions_average(&self) -> f64 {
        self.functions_sum() / self.space_count as f64
    }

    /// Returns the average number of closures over all spaces
    #[inline(always)]
    pub fn closures_average(&self) -> f64 {
        self.closures_sum() / self.space_count as f64
    }

    /// Returns the average number of function definitions and closures over all spaces
    #[inline(always)]
    pub fn average(&self) -> f64 {
        self.total() / self.space_count as f64
    }

    /// Counts the number of function definitions in a scope
    #[inline(always)]
    pub fn functions_min(&self) -> f64 {
        // Only function definitions are considered, not general declarations
        self.functions_min as f64
    }

    /// Counts the number of closures in a scope
    #[inline(always)]
    pub fn closures_min(&self) -> f64 {
        self.closures_min as f64
    }
    /// Counts the number of function definitions in a scope
    #[inline(always)]
    pub fn functions_max(&self) -> f64 {
        // Only function definitions are considered, not general declarations
        self.functions_max as f64
    }

    /// Counts the number of closures in a scope
    #[inline(always)]
    pub fn closures_max(&self) -> f64 {
        self.closures_max as f64
    }
    /// Returns the total number of function definitions and
    /// closures in a scope
    #[inline(always)]
    pub fn total(&self) -> f64 {
        self.functions_sum() + self.closures_sum()
    }
    pub(crate) fn compute_minmax(&mut self) {
        self.functions_min = self.functions_min.min(self.functions);
        self.functions_max = self.functions_max.max(self.functions);
        self.closures_min = self.closures_min.min(self.closures);
        self.closures_max = self.closures_max.max(self.closures);
        self.functions_sum += self.functions;
        self.closures_sum += self.closures;
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
                (functions_sum, 3, usize),
                (closures_sum, 1, usize),
                (total, 4, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 1, usize),
            ],
            [
                (functions_average, 0.75), // number of spaces = 4
                (closures_average, 0.25),
                (average, 1.0)
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
                (functions_sum, 2, usize),
                (closures_sum, 1, usize),
                (total, 3, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 1, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 4
                (closures_average, 0.25),
                (average, 0.75)
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
                (functions_sum, 1, usize),
                (closures_sum, 0, usize),
                (total, 1, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 0, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 2
                (closures_average, 0.0),
                (average, 0.5)
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
                (functions_sum, 2, usize),
                (closures_sum, 1, usize),
                (total, 3, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 1, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 4
                (closures_average, 0.25),
                (average, 0.75)
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
                (functions_sum, 3, usize), // f, foo, bar
                (closures_sum, 1, usize),  // return function ()
                (total, 4, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 1, usize),
            ],
            [
                (functions_average, 0.6), // number of spaces = 5
                (closures_average, 0.2),
                (average, 0.8)
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
                (functions_sum, 1, usize), // test_safe_mode
                (closures_sum, 0, usize),
                (total, 1, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 0, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 2
                (closures_average, 0.0),
                (average, 0.5)
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
                (functions_sum, 1, usize),
                (closures_sum, 0, usize),
                (total, 1, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 0, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 2
                (closures_average, 0.0),
                (average, 0.5)
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
                (functions_sum, 1, usize),
                (closures_sum, 0, usize),
                (total, 1, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 0, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 2
                (closures_average, 0.0),
                (average, 0.5)
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
                (functions_sum, 1, usize),
                (closures_sum, 0, usize),
                (total, 1, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 0, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 2
                (closures_average, 0.0),
                (average, 0.5)
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
                (functions_sum, 1, usize),
                (closures_sum, 0, usize),
                (total, 1, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 0, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 2
                (closures_average, 0.0),
                (average, 0.5)
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
                (functions_sum, 0, usize),
                (closures_sum, 2, usize),
                (total, 2, usize),
                (functions_max, 0, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 1, usize),
            ],
            [
                (functions_average, 0.0), // number of spaces = 3
                (closures_average, 0.6666666666666666),
                (average, 0.6666666666666666)
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
                (functions_sum, 1, usize), // add
                (closures_sum, 1, usize),  // materials.map
                (total, 2, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 1, usize),
            ],
            [
                (functions_average, 0.3333333333333333), // number of spaces = 3
                (closures_average, 0.3333333333333333),
                (average, 0.6666666666666666)
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
                (functions_sum, 1, usize),
                (closures_sum, 0, usize),
                (total, 1, usize),
                (functions_max, 1, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 0, usize),
            ],
            [
                (functions_average, 0.5), // number of spaces = 2
                (closures_average, 0.0),
                (average, 0.5)
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
                (functions_sum, 0, usize),
                (closures_sum, 1, usize),
                (total, 1, usize),
                (functions_max, 0, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 1, usize),
            ],
            [
                (functions_average, 0.0), // number of spaces = 2
                (closures_average, 0.5),
                (average, 0.5)
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
                (functions_sum, 0, usize),
                (closures_sum, 1, usize),
                (total, 1, usize),
                (functions_max, 0, usize),
                (functions_min, 0, usize),
                (closures_min, 0, usize),
                (closures_max, 1, usize),
            ],
            [
                (functions_average, 0.0), // number of spaces = 2
                (closures_average, 0.5),
                (average, 0.5)
            ]
        );
    }
}
