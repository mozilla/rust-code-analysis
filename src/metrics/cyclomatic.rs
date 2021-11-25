use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::*;

/// The `Cyclomatic` metric.
#[derive(Debug, Clone)]
pub struct Stats {
    cyclomatic_sum: f64,
    cyclomatic: f64,
    n: usize,
    cyclomatic_max: f64,
    cyclomatic_min: f64,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            cyclomatic_sum: 0.,
            cyclomatic: 1.,
            n: 1,
            cyclomatic_max: 0.,
            cyclomatic_min: f64::MAX,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("cyclomatic", 2)?;
        st.serialize_field("sum", &self.cyclomatic_sum())?;
        st.serialize_field("average", &self.cyclomatic_average())?;
        st.serialize_field("min", &self.cyclomatic_min())?;
        st.serialize_field("max", &self.cyclomatic_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {}, min: {}, max: {}",
            self.cyclomatic_sum(),
            self.cyclomatic_average(),
            self.cyclomatic_min(),
            self.cyclomatic_max()
        )
    }
}

impl Stats {
    /// Merges a second `Cyclomatic` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        //Calculate minimum and maximum values
        self.cyclomatic_max = self.cyclomatic_max.max(other.cyclomatic_max);
        self.cyclomatic_min = self.cyclomatic_min.min(other.cyclomatic_min);

        self.cyclomatic_sum += other.cyclomatic_sum;
        self.n += other.n;
    }

    /// Returns the `Cyclomatic` metric value
    pub fn cyclomatic(&self) -> f64 {
        self.cyclomatic
    }
    /// Returns the sum
    pub fn cyclomatic_sum(&self) -> f64 {
        self.cyclomatic_sum
    }

    /// Returns the `Cyclomatic` metric average value
    ///
    /// This value is computed dividing the `Cyclomatic` value for the
    /// number of spaces.
    pub fn cyclomatic_average(&self) -> f64 {
        self.cyclomatic_sum() / self.n as f64
    }
    /// Returns the `Cyclomatic` maximum value
    pub fn cyclomatic_max(&self) -> f64 {
        self.cyclomatic_max
    }
    /// Returns the `Cyclomatic` minimum value
    pub fn cyclomatic_min(&self) -> f64 {
        self.cyclomatic_min
    }
    /// Last step for computing minimum and maximum value and update cyclomatic_sum
    pub fn compute_minmax(&mut self) {
        self.cyclomatic_max = self.cyclomatic_max.max(self.cyclomatic);
        self.cyclomatic_min = self.cyclomatic_min.min(self.cyclomatic);
        self.cyclomatic_sum += self.cyclomatic;
    }
}

#[doc(hidden)]
pub trait Cyclomatic
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Cyclomatic for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Python::*;

        match node.object().kind_id().into() {
            If | Elif | For | While | Except | With | Assert | And | Or => {
                stats.cyclomatic += 1.;
            }
            Else => {
                if has_ancestors!(node, ForStatement | WhileStatement, ElseClause) {
                    stats.cyclomatic += 1.;
                }
            }
            _ => {}
        }
    }
}

impl Cyclomatic for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Mozjs::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Javascript::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Typescript::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Tsx::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | TernaryExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Rust::*;

        match node.object().kind_id().into() {
            If | For | While | Loop | MatchArm | MatchArm2 | QMARK | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Cpp::*;

        match node.object().kind_id().into() {
            If | For | While | Case | Catch | ConditionalExpression | AMPAMP | PIPEPIPE => {
                stats.cyclomatic += 1.;
            }
            _ => {}
        }
    }
}

impl Cyclomatic for PreprocCode {}
impl Cyclomatic for CcommentCode {}
impl Cyclomatic for JavaCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn python_simple_function() {
        check_metrics!(
            "def f(a, b): # +2 (+1 unit space)
                if a and b:  # +2 (+1 and)
                   return 1
                if c and d: # +2 (+1 and)
                   return 1",
            "foo.py",
            PythonParser,
            cyclomatic,
            [(cyclomatic_sum, 6, usize)],
            [
                (cyclomatic_average, 3.0), // nspace = 2 (func and unit)
                (cyclomatic_max, 5.0),
                (cyclomatic_min, 1.0)
            ]
        );
    }

    #[test]
    fn python_1_level_nesting() {
        check_metrics!(
            "def f(a, b): # +2 (+1 unit space)
                if a:  # +1
                    for i in range(b):  # +1
                        return 1",
            "foo.py",
            PythonParser,
            cyclomatic,
            [(cyclomatic_sum, 4, usize)],
            [
                (cyclomatic_average, 2.0), // nspace = 2 (func and unit)
                (cyclomatic_max, 3.0),
                (cyclomatic_min, 1.0)
            ]
        );
    }

    #[test]
    fn rust_1_level_nesting() {
        check_metrics!(
            "fn f() { // +2 (+1 unit space)
                 if true { // +1
                     match true {
                         true => println!(\"test\"), // +1
                         false => println!(\"test\"), // +1
                     }
                 }
             }",
            "foo.rs",
            RustParser,
            cyclomatic,
            [(cyclomatic_sum, 5, usize)],
            [
                (cyclomatic_average, 2.5), // nspace = 2 (func and unit)
                (cyclomatic_max, 4.0),
                (cyclomatic_min, 1.0)
            ]
        );
    }

    #[test]
    fn c_switch() {
        check_metrics!(
            "void f() { // +2 (+1 unit space)
                 switch (1) {
                     case 1: // +1
                         printf(\"one\");
                         break;
                     case 2: // +1
                         printf(\"two\");
                         break;
                     case 3: // +1
                         printf(\"three\");
                         break;
                     default:
                         printf(\"all\");
                         break;
                 }
             }",
            "foo.c",
            CppParser,
            cyclomatic,
            [(cyclomatic_sum, 5, usize)],
            [
                (cyclomatic_average, 2.5), // nspace = 2 (func and unit)
                (cyclomatic_max, 4.0),
                (cyclomatic_min, 1.0)
            ]
        );
    }

    #[test]
    fn c_real_function() {
        check_metrics!(
            "int sumOfPrimes(int max) { // +2 (+1 unit space)
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }",
            "foo.c",
            CppParser,
            cyclomatic,
            [(cyclomatic_sum, 5, usize)],
            [
                (cyclomatic_average, 2.5), // nspace = 2 (func and unit)
                (cyclomatic_max, 4.0),
                (cyclomatic_min, 1.0)
            ]
        );
    }
    #[test]
    fn c_unit_before() {
        check_metrics!(
            "
            int a=42;
            if(a==42) //+2(+1 unit space)
            {

            }
            if(a==34) //+1
            {
                
            }
            int sumOfPrimes(int max) { // +1 
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }",
            "foo.c",
            CppParser,
            cyclomatic,
            [(cyclomatic_sum, 7, usize)],
            [
                (cyclomatic_average, 3.5), // nspace = 2 (func and unit)
                (cyclomatic_max, 4.0),
                (cyclomatic_min, 3.0)
            ]
        );
    }
    /// Test to handle the case of min and max when merge happen before the final value of one module are setted.
    /// In this case the min value should be 3 because the unit space has 2 branches and a complexity of 3
    /// while the function sumOfPrimes has a complexity of 4.
    #[test]
    fn c_unit_after() {
        check_metrics!(
            "
            int sumOfPrimes(int max) { // +1 
                 int total = 0;
                 OUT: for (int i = 1; i <= max; ++i) { // +1
                   for (int j = 2; j < i; ++j) { // +1
                       if (i % j == 0) { // +1
                          continue OUT;
                       }
                   }
                   total += i;
                 }
                 return total;
            }
            
            int a=42;
            if(a==42) //+2(+1 unit space)
            {

            }
            if(a==34) //+1
            {
                
            }",
            "foo.c",
            CppParser,
            cyclomatic,
            [(cyclomatic_sum, 7, usize)],
            [
                (cyclomatic_average, 3.5), // nspace = 2 (func and unit)
                (cyclomatic_max, 4.0),
                (cyclomatic_min, 3.0)
            ]
        );
    }
}
