use fxhash::FxHashMap;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::getter::Getter;

use crate::*;

/// The `Halstead` metric suite.
#[derive(Default, Clone, Debug)]
pub struct Stats {
    u_operators: u64,
    operators: u64,
    u_operands: u64,
    operands: u64,
}

/// Specifies the type of nodes accepted by the `Halstead` metric.
pub enum HalsteadType {
    /// The node is an `Halstead` operator
    Operator,
    /// The node is an `Halstead` operand
    Operand,
    /// The node is unknown to the `Halstead` metric
    Unknown,
}

#[doc(hidden)]
#[derive(Debug, Default, Clone)]
pub struct HalsteadMaps<'a> {
    operators: FxHashMap<u16, u64>,
    operands: FxHashMap<&'a [u8], u64>,
}

impl<'a> HalsteadMaps<'a> {
    pub(crate) fn new() -> Self {
        HalsteadMaps {
            operators: FxHashMap::default(),
            operands: FxHashMap::default(),
        }
    }

    pub(crate) fn merge(&mut self, other: &HalsteadMaps<'a>) {
        for (k, v) in other.operators.iter() {
            *self.operators.entry(*k).or_insert(0) += v;
        }
        for (k, v) in other.operands.iter() {
            *self.operands.entry(*k).or_insert(0) += v;
        }
    }

    pub(crate) fn finalize(&self, stats: &mut Stats) {
        stats.u_operators = self.operators.len() as u64;
        stats.operators = self.operators.values().sum::<u64>();
        stats.u_operands = self.operands.len() as u64;
        stats.operands = self.operands.values().sum::<u64>();
    }
}

impl Serialize for Stats {
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

impl fmt::Display for Stats {
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

impl Stats {
    pub(crate) fn merge(&mut self, _other: &Stats) {}

    /// Returns `η1`, the number of distinct operators
    #[inline(always)]
    pub fn u_operators(&self) -> f64 {
        self.u_operators as f64
    }

    /// Returns `N1`, the number of total operators
    #[inline(always)]
    pub fn operators(&self) -> f64 {
        self.operators as f64
    }

    /// Returns `η2`, the number of distinct operands
    #[inline(always)]
    pub fn u_operands(&self) -> f64 {
        self.u_operands as f64
    }

    /// Returns `N2`, the number of total operands
    #[inline(always)]
    pub fn operands(&self) -> f64 {
        self.operands as f64
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

    /// Returns the program volume.
    ///
    /// Unit of measurement: bits
    #[inline(always)]
    pub fn volume(&self) -> f64 {
        // Assumes a uniform binary encoding for the vocabulary is used.
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

    /// Returns the estimated time required to program.
    ///
    /// Unit of measurement: seconds
    #[inline(always)]
    pub fn time(&self) -> f64 {
        // The floating point `18.` aims to describe the processing rate of the
        // human brain. It is called Stoud number, S, and its
        // unit of measurement is moments/seconds.
        // A moment is the time required by the human brain to carry out the
        // most elementary decision.
        // 5 <= S <= 20. Halstead uses 18.
        // The value of S has been empirically developed from psychological
        // reasoning, and its recommended value for
        // programming applications is 18.
        //
        // Source: https://www.geeksforgeeks.org/software-engineering-halsteads-software-metrics/
        self.effort() / 18.
    }

    /// Returns the estimated number of delivered bugs.
    ///
    /// This metric represents the average amount of work a programmer can do
    /// without introducing an error.
    #[inline(always)]
    pub fn bugs(&self) -> f64 {
        // The floating point `3000.` represents the number of elementary
        // mental discriminations.
        // A mental discrimination, in psychology, is the ability to perceive
        // and respond to differences among stimuli.
        //
        // The value above is obtained starting from a constant that
        // is different for every language and assumes that natural language is
        // the language of the brain.
        // For programming languages, the English language constant
        // has been considered.
        //
        // After every 3000 mental discriminations a result is produced.
        // This result, whether correct or incorrect, is more than likely
        // either used as an input for the next operation or is output to the
        // environment.
        // If incorrect the error should become apparent.
        // Thus, an opportunity for error occurs every 3000
        // mental discriminations.
        //
        // Source: https://docs.lib.purdue.edu/cgi/viewcontent.cgi?article=1145&context=cstech
        self.effort().powf(2. / 3.) / 3000.
    }
}

#[doc(hidden)]
pub trait Halstead
where
    Self: Checker,
{
    fn compute<'a>(_node: &Node<'a>, _code: &'a [u8], _halstead_maps: &mut HalsteadMaps<'a>) {}
}

#[inline(always)]
fn get_id<'a>(node: &Node<'a>, code: &'a [u8]) -> &'a [u8] {
    &code[node.object().start_byte()..node.object().end_byte()]
}

#[inline(always)]
fn compute_halstead<'a, T: Getter>(
    node: &Node<'a>,
    code: &'a [u8],
    halstead_maps: &mut HalsteadMaps<'a>,
) {
    match T::get_op_type(&node) {
        HalsteadType::Operator => {
            *halstead_maps
                .operators
                .entry(node.object().kind_id())
                .or_insert(0) += 1;
        }
        HalsteadType::Operand => {
            *halstead_maps
                .operands
                .entry(get_id(node, code))
                .or_insert(0) += 1;
        }
        _ => {}
    }
}

impl Halstead for PythonCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for MozjsCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for JavascriptCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for TypescriptCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for TsxCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for RustCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for CppCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

impl Halstead for PreprocCode {}
impl Halstead for CcommentCode {}
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
            "def foo():
                 def bar():
                     def toto():
                        a = 1 + 1
                     b = 2 + a
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
    fn test_wrong_halstead_operators() {
        check_metrics!(
            "()[]{}",
            "foo.py",
            PythonParser,
            halstead,
            [(u_operators, 0, usize), (operators, 0, usize)]
        );
    }

    #[test]
    fn test_halstead_formulas() {
        check_metrics!(
            "def f():
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
