use fxhash::FxHashMap;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;
use tree_sitter::Node;

use crate::checker::Checker;
use crate::getter::Getter;

use crate::*;

/// The `Halstead` metric suite.
#[derive(Default, Debug)]
pub struct Stats<'a> {
    operators: FxHashMap<u16, u64>,
    operands: FxHashMap<&'a [u8], u64>,
}

impl Serialize for Stats<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("halstead", 14)?;
        st.serialize_field("n1", &self.u_operators())?;
        st.serialize_field("N1", &self.operators())?;
        st.serialize_field("n2", &self.u_operands())?;
        st.serialize_field("N2", &self.operands())?;
        st.serialize_field("length", &self.length())?;
        st.serialize_field("estimated_program_length", &self.estimated_program_length())?;
        st.serialize_field("purity_ratio", &self.purity_ratio())?;
        st.serialize_field("vocabulary", &self.vocabulary())?;
        st.serialize_field("volume", &self.volume())?;
        st.serialize_field("difficulty", &self.difficulty())?;
        st.serialize_field("level", &self.level())?;
        st.serialize_field("effort", &self.effort())?;
        st.serialize_field("time", &self.time())?;
        st.serialize_field("bugs", &self.bugs())?;
        st.end()
    }
}

impl<'a> fmt::Display for Stats<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "n1: {}, \
             N1: {}, \
             n2: {}, \
             N2: {}, \
             length: {}, \
             estimated program length: {}, \
             purity ratio: {}, \
             size: {}, \
             volume: {}, \
             difficulty: {}, \
             level: {}, \
             effort: {}, \
             time: {}, \
             bugs: {}",
            self.u_operators(),
            self.operators(),
            self.u_operands(),
            self.operands(),
            self.length(),
            self.estimated_program_length(),
            self.purity_ratio(),
            self.vocabulary(),
            self.volume(),
            self.difficulty(),
            self.level(),
            self.effort(),
            self.time(),
            self.bugs(),
        )
    }
}

impl<'a> Stats<'a> {
    /// Merges a second `Halstead` metric suite into the first one
    pub fn merge(&mut self, other: &Stats<'a>) {
        for (k, v) in other.operators.iter() {
            *self.operators.entry(*k).or_insert(0) += v;
        }
        for (k, v) in other.operands.iter() {
            *self.operands.entry(*k).or_insert(0) += v;
        }
    }

    /// Returns `η1`, the number of distinct operators
    #[inline(always)]
    pub fn u_operators(&self) -> f64 {
        self.operators.len() as f64
    }

    /// Returns `N1`, the number of total operators
    #[inline(always)]
    pub fn operators(&self) -> f64 {
        self.operators.values().sum::<u64>() as f64
    }

    /// Returns `η2`, the number of distinct operands
    #[inline(always)]
    pub fn u_operands(&self) -> f64 {
        self.operands.len() as f64
    }

    /// Returns `N2`, the number of total operands
    #[inline(always)]
    pub fn operands(&self) -> f64 {
        self.operands.values().sum::<u64>() as f64
    }

    /// Returns the program length
    #[inline(always)]
    pub fn length(&self) -> f64 {
        self.operands() + self.operators()
    }

    /// Returns the calculated estimated program length
    #[inline(always)]
    pub fn estimated_program_length(&self) -> f64 {
        self.u_operators() * self.u_operators().log2()
            + self.u_operands() * self.u_operands().log2()
    }

    /// Returns the purity ratio
    #[inline(always)]
    pub fn purity_ratio(&self) -> f64 {
        self.estimated_program_length() / self.length()
    }

    /// Returns the program vocabulary
    #[inline(always)]
    pub fn vocabulary(&self) -> f64 {
        self.u_operands() + self.u_operators()
    }

    /// Returns the program volume
    #[inline(always)]
    pub fn volume(&self) -> f64 {
        self.length() * self.vocabulary().log2()
    }

    /// Returns the estimated difficulty required to program
    #[inline(always)]
    pub fn difficulty(&self) -> f64 {
        self.u_operators() / 2. * self.operands() / self.u_operands()
    }

    /// Returns the estimated level of difficulty required to program
    #[inline(always)]
    pub fn level(&self) -> f64 {
        1. / self.difficulty()
    }

    /// Returns the estimated effort required to program
    #[inline(always)]
    pub fn effort(&self) -> f64 {
        self.difficulty() * self.volume()
    }

    /// Returns the estimated time required to program
    #[inline(always)]
    pub fn time(&self) -> f64 {
        self.effort() / 18.
    }

    /// Returns the number of delivered bugs
    #[inline(always)]
    pub fn bugs(&self) -> f64 {
        self.effort().powf(2. / 3.) / 3000.
    }
}

#[doc(hidden)]
pub trait Halstead
where
    Self: Checker,
{
    fn compute<'a>(_node: &Node<'a>, _code: &'a [u8], _stats: &mut Stats<'a>) {}
}

#[doc(hidden)]
pub enum HalsteadType {
    Operator,
    Operand,
    Unknown,
}

#[inline(always)]
fn get_id<'a>(node: &Node<'a>, code: &'a [u8]) -> &'a [u8] {
    &code[node.start_byte()..node.end_byte()]
}

#[inline(always)]
fn compute_halstead<'a, T: Getter>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
    match T::get_op_type(&node) {
        HalsteadType::Operator => *stats.operators.entry(node.kind_id()).or_insert(0) += 1,
        HalsteadType::Operand => *stats.operands.entry(get_id(node, code)).or_insert(0) += 1,
        _ => {}
    }
}

impl Halstead for PythonCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        compute_halstead::<Self>(node, code, stats);
    }
}

impl Halstead for MozjsCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        compute_halstead::<Self>(node, code, stats);
    }
}

impl Halstead for JavascriptCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        compute_halstead::<Self>(node, code, stats);
    }
}

impl Halstead for TypescriptCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        compute_halstead::<Self>(node, code, stats);
    }
}

impl Halstead for TsxCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        compute_halstead::<Self>(node, code, stats);
    }
}

impl Halstead for RustCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        compute_halstead::<Self>(node, code, stats);
    }
}

impl Halstead for CppCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], stats: &mut Stats<'a>) {
        compute_halstead::<Self>(node, code, stats);
    }
}

impl Halstead for PreprocCode {}
impl Halstead for CcommentCode {}
impl Halstead for CSharpCode {}
impl Halstead for JavaCode {}
impl Halstead for GoCode {}
impl Halstead for CssCode {}
impl Halstead for HtmlCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_halstead_operators_and_operands() {
        check_metrics!(
            "def foo():\n
                 def bar():\n
                     def toto():\n
                        a = 1 + 1\n
                     b = 2 + a\n
                 c = 3 + 3\n",
            "foo.py",
            PythonParser,
            halstead,
            [
                (u_operators, 3, usize), // def, =, +
                (operators, 9, usize),   // def, def, def, =, =, =, +, +, +
                (u_operands, 9, usize),  // foo, bar, toto, a, b, c, 1, 2, 3
                (operands, 12, usize)    // foo, bar, toto, a, b, c, 1, 1, 2, a, 3, 3
            ]
        );
    }

    #[test]
    fn test_halstead_formulas() {
        check_metrics!(
            "def f():\n
                 pass\n",
            "foo.py",
            PythonParser,
            halstead,
            [(vocabulary, 3, usize), (length, 3, usize)],
            [
                (volume, 4.754_887_502_163_468),
                (estimated_program_length, 2.0),
                (difficulty, 1.0),
                (effort, 4.754_887_502_163_468),
                (purity_ratio, 0.666_666_666_666_666_6),
                (level, 1.0),
                (time, 0.264_160_416_786_859_36),
                (bugs, 0.000_942_552_557_372_941_4)
            ]
        );
    }
}
