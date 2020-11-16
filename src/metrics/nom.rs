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
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Nom for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Python::*;

        match node.object().kind_id().into() {
            FunctionDefinition => {
                stats.functions += 1;
            }
            Lambda => {
                stats.closures += 1;
            }
            _ => {}
        }
    }
}

impl Nom for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Mozjs::*;

        match node.object().kind_id().into() {
            Function | FunctionDeclaration | MethodDefinition => {
                stats.functions += 1;
            }
            GeneratorFunction | GeneratorFunctionDeclaration | ArrowFunction => {
                stats.closures += 1;
            }
            _ => {}
        }
    }
}

impl Nom for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Javascript::*;

        match node.object().kind_id().into() {
            Function | FunctionDeclaration | MethodDefinition => {
                stats.functions += 1;
            }
            GeneratorFunction | GeneratorFunctionDeclaration | ArrowFunction => {
                stats.closures += 1;
            }
            _ => {}
        }
    }
}

impl Nom for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Typescript::*;

        match node.object().kind_id().into() {
            Function | FunctionDeclaration | MethodDefinition => {
                stats.functions += 1;
            }
            GeneratorFunction | GeneratorFunctionDeclaration | ArrowFunction => {
                stats.closures += 1;
            }
            _ => {}
        }
    }
}

impl Nom for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Tsx::*;

        match node.object().kind_id().into() {
            Function | FunctionDeclaration | MethodDefinition => {
                stats.functions += 1;
            }
            GeneratorFunction | GeneratorFunctionDeclaration | ArrowFunction => {
                stats.closures += 1;
            }
            _ => {}
        }
    }
}

impl Nom for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Rust::*;

        match node.object().kind_id().into() {
            FunctionItem => {
                stats.functions += 1;
            }
            ClosureExpression => {
                stats.closures += 1;
            }
            _ => {}
        }
    }
}

impl Nom for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Cpp::*;

        match node.object().kind_id().into() {
            FunctionDefinition
            | FunctionDefinition2
            | FunctionDefinition3
            | FunctionDefinition4
            | FunctionDefinitionRepeat1 => {
                stats.functions += 1;
            }
            LambdaExpression => {
                stats.closures += 1;
            }
            _ => {}
        }
    }
}

impl Nom for PreprocCode {}
impl Nom for CcommentCode {}
impl Nom for JavaCode {}
impl Nom for GoCode {}
impl Nom for CssCode {}
impl Nom for HtmlCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_nom_python() {
        check_metrics!(
            "def a():
                 pass
             def b():
                 pass
             def c():
                 pass
             x = lambda a : a + 42\n",
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
    fn test_nom_rust() {
        check_metrics!(
            "mod A { fn foo() {}}
             mod B { fn foo() {}}
             let closure = |i: i32| -> i32 { i + 42 };\n",
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
    fn test_nom_cpp() {
        check_metrics!(
            "struct A {
                 void foo(int) {}
                 void foo(double) {}
             };
             int b = [](int x) -> int { return x + 42; };\n",
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
    fn test_nom_c() {
        check_metrics!(
            "int foo();

             int foo() {
                 return 0;
             }\n",
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
}
