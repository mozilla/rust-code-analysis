use std::fmt;

use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

use crate::checker::Checker;
use crate::*;

mod cpp;
mod javascript;
mod mozjs;
mod python;
mod rust;
mod tsx;
mod typescript;

/// The `NExit` metric.
///
/// This metric counts the number of possible exit points
/// from a function/method.
#[derive(Debug, Clone)]
pub struct Stats {
    exit: usize,
    total_space_functions: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            exit: 0,
            total_space_functions: 1,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("nexits", 2)?;
        st.serialize_field("sum", &self.exit())?;
        st.serialize_field("average", &self.exit_average())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "sum: {}, average: {}", self.exit(), self.exit_average())
    }
}

impl Stats {
    /// Merges a second `NExit` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.exit += other.exit;
    }

    /// Returns the `NExit` metric value
    pub fn exit(&self) -> f64 {
        self.exit as f64
    }

    /// Returns the `NExit` metric average value
    ///
    /// This value is computed dividing the `NExit` value
    /// for the total number of functions/closures in a space.
    ///
    /// If there are no functions in a code, its value is `NAN`.
    pub fn exit_average(&self) -> f64 {
        self.exit() / self.total_space_functions as f64
    }

    pub(crate) fn finalize(&mut self, total_space_functions: usize) {
        self.total_space_functions = total_space_functions;
    }
}

#[doc(hidden)]
pub trait Exit
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

impl Exit for PreprocCode {}
impl Exit for CcommentCode {}
impl Exit for JavaCode {}
