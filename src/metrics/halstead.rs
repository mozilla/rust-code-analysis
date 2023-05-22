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

#[derive(Debug, Default, Clone)]
pub struct HalsteadMaps<'a> {
    pub(crate) operators: FxHashMap<u16, u64>,
    pub(crate) operands: FxHashMap<&'a [u8], u64>,
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

pub trait Halstead
where
    Self: Checker,
{
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>);
}

#[inline(always)]
fn get_id<'a>(node: &Node<'a>, code: &'a [u8]) -> &'a [u8] {
    &code[node.start_byte()..node.end_byte()]
}

#[inline(always)]
fn compute_halstead<'a, T: Getter>(
    node: &Node<'a>,
    code: &'a [u8],
    halstead_maps: &mut HalsteadMaps<'a>,
) {
    match T::get_op_type(node) {
        HalsteadType::Operator => {
            *halstead_maps.operators.entry(node.kind_id()).or_insert(0) += 1;
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

impl Halstead for JavaCode {
    fn compute<'a>(node: &Node<'a>, code: &'a [u8], halstead_maps: &mut HalsteadMaps<'a>) {
        compute_halstead::<Self>(node, code, halstead_maps);
    }
}

implement_metric_trait!(Halstead, KotlinCode, PreprocCode, CcommentCode);

#[cfg(test)]
mod tests {
    use crate::tools::check_metrics;

    use super::*;

    #[test]
    fn python_operators_and_operands() {
        check_metrics::<PythonParser>(
            "def foo():
                 def bar():
                     def toto():
                        a = 1 + 1
                     b = 2 + a
                 c = 3 + 3",
            "foo.py",
            |metric| {
                // unique operators: def, =, +
                // operators: def, def, def, =, =, =, +, +, +
                // unique operands: foo, bar, toto, a, b, c, 1, 2, 3
                // operands: foo, bar, toto, a, b, c, 1, 1, 2, a, 3, 3
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 3.0,
                      "N1": 9.0,
                      "n2": 9.0,
                      "N2": 12.0,
                      "length": 21.0,
                      "estimated_program_length": 33.284212515144276,
                      "purity_ratio": 1.584962500721156,
                      "vocabulary": 12.0,
                      "volume": 75.28421251514428,
                      "difficulty": 2.0,
                      "level": 0.5,
                      "effort": 150.56842503028855,
                      "time": 8.364912501682698,
                      "bugs": 0.0094341190071077
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_operators_and_operands() {
        // Define operators and operands for C/C++ grammar according to this specification:
        // https://www.verifysoft.com/en_halstead_metrics.html
        // The only difference with the specification above is that
        // primitive types are treated as operators, since the definition of a
        // primitive type can be seen as the creation of a slot of a certain size.
        // i.e. The `int a;` definition creates a n-bytes slot.
        check_metrics::<CppParser>(
            "main()
            {
              int a, b, c, avg;
              scanf(\"%d %d %d\", &a, &b, &c);
              avg = (a + b + c) / 3;
              printf(\"avg = %d\", avg);
            }",
            "foo.c",
            |metric| {
                // unique operators: (), {}, int, &, =, +, /, ,, ;
                // unique operands: main, a, b, c, avg, scanf, "%d %d %d", 3, printf, "avg = %d"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 9.0,
                      "N1": 24.0,
                      "n2": 10.0,
                      "N2": 18.0,
                      "length": 42.0,
                      "estimated_program_length": 61.74860596185444,
                      "purity_ratio": 1.470204903853677,
                      "vocabulary": 19.0,
                      "volume": 178.41295556463058,
                      "difficulty": 8.1,
                      "level": 0.1234567901234568,
                      "effort": 1445.1449400735075,
                      "time": 80.28583000408375,
                      "bugs": 0.04260752914034329
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_operators_and_operands() {
        check_metrics::<RustParser>(
            "fn main() {
              let a = 5; let b = 5; let c = 5;
              let avg = (a + b + c) / 3;
              println!(\"{}\", avg);
            }",
            "foo.rs",
            |metric| {
                // FIXME tree-sitter-rust does not parse the comma inside the println! macro
                // unique operators: fn, (), {}, let, =, +, /, ;, !
                // unique operands: main, a, b, c, avg, 5, 3, println, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 9.0,
                      "N1": 22.0,
                      "n2": 9.0,
                      "N2": 15.0,
                      "length": 37.0,
                      "estimated_program_length": 57.05865002596162,
                      "purity_ratio": 1.542125676377341,
                      "vocabulary": 18.0,
                      "volume": 154.28722505336555,
                      "difficulty": 7.5,
                      "level": 0.13333333333333333,
                      "effort": 1157.1541879002416,
                      "time": 64.28634377223564,
                      "bugs": 0.03674003504721376
                    }"###
                );
            },
        );
    }

    #[test]
    fn javascript_operators_and_operands() {
        check_metrics::<JavascriptParser>(
            "function main() {
              var a, b, c, avg;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.js",
            |metric| {
                // unique operators: function, (), {}, var, =, +, /, ,, ., ;
                // unique operands: main, a, b, c, avg, 3, 5, console.log, console, log, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 10.0,
                      "N1": 24.0,
                      "n2": 11.0,
                      "N2": 21.0,
                      "length": 45.0,
                      "estimated_program_length": 71.27302875388389,
                      "purity_ratio": 1.583845083419642,
                      "vocabulary": 21.0,
                      "volume": 197.65428402504423,
                      "difficulty": 9.545454545454545,
                      "level": 0.10476190476190476,
                      "effort": 1886.699983875422,
                      "time": 104.81666577085679,
                      "bugs": 0.05089564733125986
                    }"###
                );
            },
        );
    }

    #[test]
    fn mozjs_operators_and_operands() {
        check_metrics::<MozjsParser>(
            "function main() {
              var a, b, c, avg;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.js",
            |metric| {
                // unique operators: function, (), {}, var, =, +, /, ,, ., ;
                // unique operands: main, a, b, c, avg, 3, 5, console.log, console, log, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 10.0,
                      "N1": 24.0,
                      "n2": 11.0,
                      "N2": 21.0,
                      "length": 45.0,
                      "estimated_program_length": 71.27302875388389,
                      "purity_ratio": 1.583845083419642,
                      "vocabulary": 21.0,
                      "volume": 197.65428402504423,
                      "difficulty": 9.545454545454545,
                      "level": 0.10476190476190476,
                      "effort": 1886.699983875422,
                      "time": 104.81666577085679,
                      "bugs": 0.05089564733125986
                    }"###
                );
            },
        );
    }

    #[test]
    fn typescript_operators_and_operands() {
        check_metrics::<TypescriptParser>(
            "function main() {
              var a, b, c, avg;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.ts",
            |metric| {
                // unique operators: function, (), {}, var, =, +, /, ,, ., ;
                // unique operands: main, a, b, c, avg, 3, 5, console.log, console, log, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 10.0,
                      "N1": 24.0,
                      "n2": 11.0,
                      "N2": 21.0,
                      "length": 45.0,
                      "estimated_program_length": 71.27302875388389,
                      "purity_ratio": 1.583845083419642,
                      "vocabulary": 21.0,
                      "volume": 197.65428402504423,
                      "difficulty": 9.545454545454545,
                      "level": 0.10476190476190476,
                      "effort": 1886.699983875422,
                      "time": 104.81666577085679,
                      "bugs": 0.05089564733125986
                    }"###
                );
            },
        );
    }

    #[test]
    fn tsx_operators_and_operands() {
        check_metrics::<TsxParser>(
            "function main() {
              var a, b, c, avg;
              a = 5; b = 5; c = 5;
              avg = (a + b + c) / 3;
              console.log(\"{}\", avg);
            }",
            "foo.ts",
            |metric| {
                // unique operators: function, (), {}, var, =, +, /, ,, ., ;
                // unique operands: main, a, b, c, avg, 3, 5, console.log, console, log, "{}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 10.0,
                      "N1": 24.0,
                      "n2": 11.0,
                      "N2": 21.0,
                      "length": 45.0,
                      "estimated_program_length": 71.27302875388389,
                      "purity_ratio": 1.583845083419642,
                      "vocabulary": 21.0,
                      "volume": 197.65428402504423,
                      "difficulty": 9.545454545454545,
                      "level": 0.10476190476190476,
                      "effort": 1886.699983875422,
                      "time": 104.81666577085679,
                      "bugs": 0.05089564733125986
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_wrong_operators() {
        check_metrics::<PythonParser>("()[]{}", "foo.py", |metric| {
            insta::assert_json_snapshot!(
                metric.halstead,
                @r###"
                    {
                      "n1": 0.0,
                      "N1": 0.0,
                      "n2": 0.0,
                      "N2": 0.0,
                      "length": 0.0,
                      "estimated_program_length": null,
                      "purity_ratio": null,
                      "vocabulary": 0.0,
                      "volume": null,
                      "difficulty": null,
                      "level": null,
                      "effort": null,
                      "time": null,
                      "bugs": null
                    }"###
            );
        });
    }

    #[test]
    fn python_check_metrics() {
        check_metrics::<PythonParser>(
            "def f():
                 pass",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 2.0,
                      "N1": 2.0,
                      "n2": 1.0,
                      "N2": 1.0,
                      "length": 3.0,
                      "estimated_program_length": 2.0,
                      "purity_ratio": 0.6666666666666666,
                      "vocabulary": 3.0,
                      "volume": 4.754887502163468,
                      "difficulty": 1.0,
                      "level": 1.0,
                      "effort": 4.754887502163468,
                      "time": 0.26416041678685936,
                      "bugs": 0.0009425525573729414
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_operators_and_operands() {
        check_metrics::<JavaParser>(
            "public class Main {
            public static void main(String args[]) {
                  int a, b, c, avg;
                  a = 5; b = 5; c = 5;
                  avg = (a + b + c) / 3;
                  MessageFormat.format(\"{0}\", avg);
                }
            }",
            "foo.java",
            |metric| {
                // { void ; ( String [ ] ) , int = + / format . }
                // Main main args a b c avg 5 3 MessageFormat format "{0}"
                insta::assert_json_snapshot!(
                    metric.halstead,
                    @r###"
                    {
                      "n1": 16.0,
                      "N1": 34.0,
                      "n2": 12.0,
                      "N2": 22.0,
                      "length": 56.0,
                      "estimated_program_length": 107.01955000865388,
                      "purity_ratio": 1.9110633930116765,
                      "vocabulary": 28.0,
                      "volume": 269.2118756352258,
                      "difficulty": 14.666666666666666,
                      "level": 0.06818181818181819,
                      "effort": 3948.440842649978,
                      "time": 219.35782459166546,
                      "bugs": 0.08327139413010551
                    }"###
                );
            },
        );
    }
}
