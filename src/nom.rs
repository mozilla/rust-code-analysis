use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;
use tree_sitter::Node;

use crate::checker::Checker;

use crate::*;

#[derive(Default, Debug)]
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
    pub fn merge(&mut self, other: &Stats) {
        self.functions += other.functions;
        self.closures += other.closures;
    }

    #[inline(always)]
    pub fn functions(&self) -> f64 {
        // Only function definitions are considered, not general declarations
        self.functions as f64
    }

    #[inline(always)]
    pub fn closures(&self) -> f64 {
        self.closures as f64
    }

    #[inline(always)]
    pub fn total(&self) -> f64 {
        self.functions() + self.closures()
    }
}

pub trait Nom
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Nom for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Python::*;

        match node.kind_id().into() {
            FunctionDefinition => {
                stats.functions += 1;
            }
            Lambda | Lambda2 | Lambda3 => {
                stats.closures += 1;
            }
            _ => {}
        }
    }
}

impl Nom for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Mozjs::*;

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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
impl Nom for CSharpCode {}
impl Nom for JavaCode {}
impl Nom for GoCode {}
impl Nom for CssCode {}
impl Nom for HtmlCode {}
