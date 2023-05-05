use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use super::cyclomatic;
use super::halstead;
use super::loc;

use crate::checker::Checker;

use crate::*;

/// The `Mi` metric.
#[derive(Default, Clone, Debug)]
pub struct Stats {
    halstead_length: f64,
    halstead_vocabulary: f64,
    halstead_volume: f64,
    cyclomatic: f64,
    sloc: f64,
    comments_percentage: f64,
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("maintanability_index", 3)?;
        st.serialize_field("mi_original", &self.mi_original())?;
        st.serialize_field("mi_sei", &self.mi_sei())?;
        st.serialize_field("mi_visual_studio", &self.mi_visual_studio())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "mi_original: {}, mi_sei: {}, mi_visual_studio: {}",
            self.mi_original(),
            self.mi_sei(),
            self.mi_visual_studio()
        )
    }
}

impl Stats {
    pub(crate) fn merge(&mut self, _other: &Stats) {}

    /// Returns the `Mi` metric calculated using the original formula.
    ///
    /// Its value can be negative.
    #[inline(always)]
    pub fn mi_original(&self) -> f64 {
        // http://www.projectcodemeter.com/cost_estimation/help/GL_maintainability.htm
        171.0 - 5.2 * (self.halstead_volume).ln() - 0.23 * self.cyclomatic - 16.2 * self.sloc.ln()
    }

    /// Returns the `Mi` metric calculated using the derivative formula
    /// employed by the Software Engineering Insitute (SEI).
    ///
    /// Its value can be negative.
    #[inline(always)]
    pub fn mi_sei(&self) -> f64 {
        // http://www.projectcodemeter.com/cost_estimation/help/GL_maintainability.htm
        171.0 - 5.2 * self.halstead_volume.log2() - 0.23 * self.cyclomatic - 16.2 * self.sloc.log2()
            + 50.0 * (self.comments_percentage * 2.4).sqrt().sin()
    }

    /// Returns the `Mi` metric calculated using the derivative formula
    /// employed by Microsoft Visual Studio.
    #[inline(always)]
    pub fn mi_visual_studio(&self) -> f64 {
        // http://www.projectcodemeter.com/cost_estimation/help/GL_maintainability.htm
        let formula = 171.0
            - 5.2 * self.halstead_volume.ln()
            - 0.23 * self.cyclomatic
            - 16.2 * self.sloc.ln();
        (formula * 100.0 / 171.0).max(0.)
    }
}

pub trait Mi
where
    Self: Checker,
{
    fn compute(
        loc: &loc::Stats,
        cyclomatic: &cyclomatic::Stats,
        halstead: &halstead::Stats,
        stats: &mut Stats,
    ) {
        stats.halstead_length = halstead.length();
        stats.halstead_vocabulary = halstead.vocabulary();
        stats.halstead_volume = halstead.volume();
        stats.cyclomatic = cyclomatic.cyclomatic_sum();
        stats.sloc = loc.sloc();
        stats.comments_percentage = loc.cloc() / stats.sloc;
    }
}

implement_metric_trait!(
    [Mi],
    PythonCode,
    MozjsCode,
    JavascriptCode,
    TypescriptCode,
    TsxCode,
    RustCode,
    CppCode,
    PreprocCode,
    CcommentCode,
    JavaCode,
    KotlinCode
);

#[cfg(test)]
mod tests {
    use crate::tools::check_metrics;

    use super::*;

    #[test]
    fn check_mi_metrics() {
        // This test checks that MI metric is computed correctly, so it verifies
        // the calculations are correct, the adopted source code is irrelevant
        check_metrics::<PythonParser>(
            "def f():
                 pass",
            "foo.py",
            |metric| {
                insta::assert_json_snapshot!(
                    metric.mi,
                    @r###"
                    {
                      "mi_original": 151.2033158832232,
                      "mi_sei": 142.64306171748976,
                      "mi_visual_studio": 88.42299174457497
                    }"###
                );
            },
        );
    }
}
