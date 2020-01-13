use serde::ser::Serializer;
use serde::Serialize;
use std::fmt;
use tree_sitter::Node;

use crate::checker::Checker;
use crate::*;

#[derive(Debug)]
pub struct Stats {
    exit: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self { exit: 0 }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f64(self.exit())
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.exit)
    }
}

impl Stats {
    pub fn merge(&mut self, other: &Stats) {
        self.exit += other.exit;
    }

    pub fn exit(&self) -> f64 {
        self.exit as f64
    }
}

pub trait Exit
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Exit for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind_id().into() {
            Python::ReturnStatement => {
                stats.exit += 1;
            }
            _ => {}
        }
    }
}

impl Exit for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind_id().into() {
            Mozjs::ReturnStatement => {
                stats.exit += 1;
            }
            _ => {}
        }
    }
}

impl Exit for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind_id().into() {
            Javascript::ReturnStatement => {
                stats.exit += 1;
            }
            _ => {}
        }
    }
}

impl Exit for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind_id().into() {
            Typescript::ReturnStatement => {
                stats.exit += 1;
            }
            _ => {}
        }
    }
}

impl Exit for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind_id().into() {
            Tsx::ReturnStatement => {
                stats.exit += 1;
            }
            _ => {}
        }
    }
}

impl Exit for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Rust::*;

        match node.kind_id().into() {
            ReturnExpression => {
                stats.exit += 1;
            }
            _ => {
                if Self::is_func(node) {
                    if let Some(_) = node.child_by_field_name("return_type") {
                        stats.exit += 1;
                    }
                }
            }
        }
    }
}

impl Exit for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        match node.kind_id().into() {
            Cpp::ReturnStatement => {
                stats.exit += 1;
            }
            _ => {}
        }
    }
}

impl Exit for PreprocCode {}
impl Exit for CcommentCode {}
impl Exit for CSharpCode {}
impl Exit for JavaCode {}
impl Exit for GoCode {}
impl Exit for CssCode {}
impl Exit for HtmlCode {}
