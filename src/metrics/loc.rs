use crate::checker::Checker;
use fxhash::FxHashSet;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::*;

/// The `SLoc` metric suite.
#[derive(Debug, Clone)]
pub struct Sloc {
    start: usize,
    end: usize,
    unit: bool,
    sloc_min: usize,
    sloc_max: usize,
}

impl Default for Sloc {
    fn default() -> Self {
        Self {
            start: 0,
            end: 0,
            unit: false,
            sloc_min: usize::MAX,
            sloc_max: 0,
        }
    }
}

impl Sloc {
    #[inline(always)]
    pub fn sloc(&self) -> f64 {
        // This metric counts the number of lines in a file
        // The if construct is needed to count the line of code that represents
        // the function signature in a function space
        let sloc = if self.unit {
            self.end - self.start
        } else {
            (self.end - self.start) + 1
        };
        sloc as f64
    }

    /// The `Sloc` metric minimum value.
    #[inline(always)]
    pub fn sloc_min(&self) -> f64 {
        self.sloc_min as f64
    }

    /// The `Sloc` metric maximum value.
    #[inline(always)]
    pub fn sloc_max(&self) -> f64 {
        self.sloc_max as f64
    }

    #[inline(always)]
    pub fn merge(&mut self, other: &Sloc) {
        self.sloc_min = self.sloc_min.min(other.sloc() as usize);
        self.sloc_max = self.sloc_max.max(other.sloc() as usize);
    }

    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        if self.sloc_min == usize::MAX {
            self.sloc_min = self.sloc_min.min(self.sloc() as usize);
            self.sloc_max = self.sloc_max.max(self.sloc() as usize);
        }
    }
}

/// The `PLoc` metric suite.
#[derive(Debug, Clone)]
pub struct Ploc {
    lines: FxHashSet<usize>,
    ploc_min: usize,
    ploc_max: usize,
}

impl Default for Ploc {
    fn default() -> Self {
        Self {
            lines: FxHashSet::default(),
            ploc_min: usize::MAX,
            ploc_max: 0,
        }
    }
}

impl Ploc {
    #[inline(always)]
    pub fn ploc(&self) -> f64 {
        // This metric counts the number of instruction lines in a code
        // https://en.wikipedia.org/wiki/Source_lines_of_code
        self.lines.len() as f64
    }

    /// The `Ploc` metric minimum value.
    #[inline(always)]
    pub fn ploc_min(&self) -> f64 {
        self.ploc_min as f64
    }

    /// The `Ploc` metric maximum value.
    #[inline(always)]
    pub fn ploc_max(&self) -> f64 {
        self.ploc_max as f64
    }

    #[inline(always)]
    pub fn merge(&mut self, other: &Ploc) {
        // Merge ploc lines
        for l in other.lines.iter() {
            self.lines.insert(*l);
        }

        self.ploc_min = self.ploc_min.min(other.ploc() as usize);
        self.ploc_max = self.ploc_max.max(other.ploc() as usize);
    }

    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        if self.ploc_min == usize::MAX {
            self.ploc_min = self.ploc_min.min(self.ploc() as usize);
            self.ploc_max = self.ploc_max.max(self.ploc() as usize);
        }
    }
}

/// The `CLoc` metric suite.
#[derive(Debug, Clone)]
pub struct Cloc {
    only_comment_lines: usize,
    code_comment_lines: usize,
    comment_line_end: Option<usize>,
    cloc_min: usize,
    cloc_max: usize,
}

impl Default for Cloc {
    fn default() -> Self {
        Self {
            only_comment_lines: 0,
            code_comment_lines: 0,
            comment_line_end: Option::default(),
            cloc_min: usize::MAX,
            cloc_max: 0,
        }
    }
}

impl Cloc {
    #[inline(always)]
    pub fn cloc(&self) -> f64 {
        // Comments are counted regardless of their placement
        // https://en.wikipedia.org/wiki/Source_lines_of_code
        (self.only_comment_lines + self.code_comment_lines) as f64
    }

    /// The `Ploc` metric minimum value.
    #[inline(always)]
    pub fn cloc_min(&self) -> f64 {
        self.cloc_min as f64
    }

    /// The `Ploc` metric maximum value.
    #[inline(always)]
    pub fn cloc_max(&self) -> f64 {
        self.cloc_max as f64
    }

    #[inline(always)]
    pub fn merge(&mut self, other: &Cloc) {
        // Merge cloc lines
        self.only_comment_lines += other.only_comment_lines;
        self.code_comment_lines += other.code_comment_lines;

        self.cloc_min = self.cloc_min.min(other.cloc() as usize);
        self.cloc_max = self.cloc_max.max(other.cloc() as usize);
    }

    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        if self.cloc_min == usize::MAX {
            self.cloc_min = self.cloc_min.min(self.cloc() as usize);
            self.cloc_max = self.cloc_max.max(self.cloc() as usize);
        }
    }
}

/// The `LLoc` metric suite.
#[derive(Debug, Clone)]
pub struct Lloc {
    logical_lines: usize,
    lloc_min: usize,
    lloc_max: usize,
}

impl Default for Lloc {
    fn default() -> Self {
        Self {
            logical_lines: 0,
            lloc_min: usize::MAX,
            lloc_max: 0,
        }
    }
}

impl Lloc {
    #[inline(always)]
    pub fn lloc(&self) -> f64 {
        // This metric counts the number of statements in a code
        // https://en.wikipedia.org/wiki/Source_lines_of_code
        self.logical_lines as f64
    }

    /// The `Lloc` metric minimum value.
    #[inline(always)]
    pub fn lloc_min(&self) -> f64 {
        self.lloc_min as f64
    }

    /// The `Lloc` metric maximum value.
    #[inline(always)]
    pub fn lloc_max(&self) -> f64 {
        self.lloc_max as f64
    }

    #[inline(always)]
    pub fn merge(&mut self, other: &Lloc) {
        // Merge lloc lines
        self.logical_lines += other.logical_lines;
        self.lloc_min = self.lloc_min.min(other.lloc() as usize);
        self.lloc_max = self.lloc_max.max(other.lloc() as usize);
    }

    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        if self.lloc_min == usize::MAX {
            self.lloc_min = self.lloc_min.min(self.lloc() as usize);
            self.lloc_max = self.lloc_max.max(self.lloc() as usize);
        }
    }
}

/// The `Loc` metric suite.
#[derive(Debug, Clone)]
pub struct Stats {
    sloc: Sloc,
    ploc: Ploc,
    cloc: Cloc,
    lloc: Lloc,
    space_count: usize,
    blank_min: usize,
    blank_max: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            sloc: Sloc::default(),
            ploc: Ploc::default(),
            cloc: Cloc::default(),
            lloc: Lloc::default(),
            space_count: 1,
            blank_min: usize::MAX,
            blank_max: 0,
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("loc", 20)?;
        st.serialize_field("sloc", &self.sloc())?;
        st.serialize_field("ploc", &self.ploc())?;
        st.serialize_field("lloc", &self.lloc())?;
        st.serialize_field("cloc", &self.cloc())?;
        st.serialize_field("blank", &self.blank())?;
        st.serialize_field("sloc_average", &self.sloc_average())?;
        st.serialize_field("ploc_average", &self.ploc_average())?;
        st.serialize_field("lloc_average", &self.lloc_average())?;
        st.serialize_field("cloc_average", &self.cloc_average())?;
        st.serialize_field("blank_average", &self.blank_average())?;
        st.serialize_field("sloc_min", &self.sloc_min())?;
        st.serialize_field("sloc_max", &self.sloc_max())?;
        st.serialize_field("cloc_min", &self.cloc_min())?;
        st.serialize_field("cloc_max", &self.cloc_max())?;
        st.serialize_field("ploc_min", &self.ploc_min())?;
        st.serialize_field("ploc_max", &self.ploc_max())?;
        st.serialize_field("lloc_min", &self.lloc_min())?;
        st.serialize_field("lloc_max", &self.lloc_max())?;
        st.serialize_field("blank_min", &self.blank_min())?;
        st.serialize_field("blank_max", &self.blank_max())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sloc: {}, ploc: {}, lloc: {}, cloc: {}, blank: {}, sloc_average: {}, ploc_average: {}, lloc_average: {}, cloc_average: {}, blank_average: {}, sloc_min: {}, sloc_max: {}, cloc_min: {}, cloc_max: {}, ploc_min: {}, ploc_max: {}, lloc_min: {}, lloc_max: {}, blank_min: {}, blank_max: {}",
            self.sloc(),
            self.ploc(),
            self.lloc(),
            self.cloc(),
            self.blank(),
            self.sloc_average(),
            self.ploc_average(),
            self.lloc_average(),
            self.cloc_average(),
            self.blank_average(),
            self.sloc_min(),
            self.sloc_max(),
            self.cloc_min(),
            self.cloc_max(),
            self.ploc_min(),
            self.ploc_max(),
            self.lloc_min(),
            self.lloc_max(),
            self.blank_min(),
            self.blank_max(),
        )
    }
}

impl Stats {
    /// Merges a second `Loc` metric suite into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.sloc.merge(&other.sloc);
        self.ploc.merge(&other.ploc);
        self.cloc.merge(&other.cloc);
        self.lloc.merge(&other.lloc);

        // Count spaces
        self.space_count += other.space_count;

        // min and max

        self.blank_min = self.blank_min.min(other.blank() as usize);
        self.blank_max = self.blank_max.max(other.blank() as usize);
    }

    /// The `Sloc` metric.
    ///
    /// Counts the number of lines in a scope
    #[inline(always)]
    pub fn sloc(&self) -> f64 {
        self.sloc.sloc()
    }

    /// The `Ploc` metric.
    ///
    /// Counts the number of instruction lines in a scope
    #[inline(always)]
    pub fn ploc(&self) -> f64 {
        self.ploc.ploc()
    }

    /// The `Lloc` metric.
    ///
    /// Counts the number of statements in a scope
    #[inline(always)]
    pub fn lloc(&self) -> f64 {
        self.lloc.lloc()
    }

    /// The `Cloc` metric.
    ///
    /// Counts the number of comments in a scope
    #[inline(always)]
    pub fn cloc(&self) -> f64 {
        self.cloc.cloc()
    }

    /// The `Blank` metric.
    ///
    /// Counts the number of blank lines in a scope
    #[inline(always)]
    pub fn blank(&self) -> f64 {
        self.sloc() - self.ploc() - self.cloc.only_comment_lines as f64
    }

    /// The `Sloc` metric average value.
    ///
    /// This value is computed dividing the `Sloc` value for the number of spaces
    #[inline(always)]
    pub fn sloc_average(&self) -> f64 {
        self.sloc() / self.space_count as f64
    }

    /// The `Ploc` metric average value.
    ///
    /// This value is computed dividing the `Ploc` value for the number of spaces
    #[inline(always)]
    pub fn ploc_average(&self) -> f64 {
        self.ploc() / self.space_count as f64
    }

    /// The `Lloc` metric average value.
    ///
    /// This value is computed dividing the `Lloc` value for the number of spaces
    #[inline(always)]
    pub fn lloc_average(&self) -> f64 {
        self.lloc() / self.space_count as f64
    }

    /// The `Cloc` metric average value.
    ///
    /// This value is computed dividing the `Cloc` value for the number of spaces
    #[inline(always)]
    pub fn cloc_average(&self) -> f64 {
        self.cloc() / self.space_count as f64
    }

    /// The `Blank` metric average value.
    ///
    /// This value is computed dividing the `Blank` value for the number of spaces
    #[inline(always)]
    pub fn blank_average(&self) -> f64 {
        self.blank() / self.space_count as f64
    }

    /// The `Sloc` metric minimum value.
    #[inline(always)]
    pub fn sloc_min(&self) -> f64 {
        self.sloc.sloc_min()
    }

    /// The `Sloc` metric maximum value.
    #[inline(always)]
    pub fn sloc_max(&self) -> f64 {
        self.sloc.sloc_max()
    }

    /// The `Cloc` metric minimum value.
    #[inline(always)]
    pub fn cloc_min(&self) -> f64 {
        self.cloc.cloc_min()
    }

    /// The `Cloc` metric maximum value.
    #[inline(always)]
    pub fn cloc_max(&self) -> f64 {
        self.cloc.cloc_max()
    }

    /// The `Ploc` metric minimum value.
    #[inline(always)]
    pub fn ploc_min(&self) -> f64 {
        self.ploc.ploc_min()
    }

    /// The `Ploc` metric maximum value.
    #[inline(always)]
    pub fn ploc_max(&self) -> f64 {
        self.ploc.ploc_max()
    }

    /// The `Lloc` metric minimum value.
    #[inline(always)]
    pub fn lloc_min(&self) -> f64 {
        self.lloc.lloc_min()
    }

    /// The `Lloc` metric maximum value.
    #[inline(always)]
    pub fn lloc_max(&self) -> f64 {
        self.lloc.lloc_max()
    }

    /// The `Blank` metric minimum value.
    #[inline(always)]
    pub fn blank_min(&self) -> f64 {
        self.blank_min as f64
    }

    /// The `Blank` metric maximum value.
    #[inline(always)]
    pub fn blank_max(&self) -> f64 {
        self.blank_max as f64
    }

    #[inline(always)]
    pub(crate) fn compute_minmax(&mut self) {
        self.sloc.compute_minmax();
        self.ploc.compute_minmax();
        self.cloc.compute_minmax();
        self.lloc.compute_minmax();

        if self.blank_min == usize::MAX {
            self.blank_min = self.blank_min.min(self.blank() as usize);
            self.blank_max = self.blank_max.max(self.blank() as usize);
        }
    }
}

#[doc(hidden)]
pub trait Loc
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats, _is_func_space: bool, _is_unit: bool) {}
}

#[inline(always)]
fn init(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) -> (usize, usize) {
    let start = node.start_row();
    let end = node.end_row();

    if is_func_space {
        stats.sloc.start = start;
        stats.sloc.end = end;
        stats.sloc.unit = is_unit;
    }
    (start, end)
}

#[inline(always)]
// Discriminates among the comments that are *after* a code line and
// the ones that are on an independent line.
// This difference is necessary in order to avoid having
// a wrong count for the blank metric.
fn add_cloc_lines(stats: &mut Stats, start: usize, end: usize) {
    let comment_diff = end - start;
    let is_comment_after_code_line = stats.ploc.lines.contains(&start);
    if is_comment_after_code_line && comment_diff == 0 {
        // A comment is *entirely* next to a code line
        stats.cloc.code_comment_lines += 1;
    } else if is_comment_after_code_line && comment_diff > 0 {
        // A block comment that starts next to a code line and ends on
        // independent lines.
        stats.cloc.code_comment_lines += 1;
        stats.cloc.only_comment_lines += comment_diff;
    } else {
        // A comment on an independent line AND
        // a block comment on independent lines OR
        // a comment *before* a code line
        stats.cloc.only_comment_lines += (end - start) + 1;
        // Save line end of a comment to check whether
        // a comment *before* a code line is considered
        stats.cloc.comment_line_end = Some(end);
    }
}

#[inline(always)]
// Detects the comments that are on a code line but *before* the code part.
// This difference is necessary in order to avoid having
// a wrong count for the blank metric.
fn check_comment_ends_on_code_line(stats: &mut Stats, start_code_line: usize) {
    if let Some(end) = stats.cloc.comment_line_end {
        if end == start_code_line && !stats.ploc.lines.contains(&start_code_line) {
            // Comment entirely *before* a code line
            stats.cloc.only_comment_lines -= 1;
            stats.cloc.code_comment_lines += 1;
        }
    }
}

impl Loc for PythonCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Python::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            DQUOTE | DQUOTE2 | Block | Module => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            String => {
                let parent = node.object().parent().unwrap();
                if let ExpressionStatement = parent.kind_id().into() {
                    add_cloc_lines(stats, start, end);
                } else if parent.start_position().row != start {
                    check_comment_ends_on_code_line(stats, start);
                    stats.ploc.lines.insert(start);
                }
            }
            Statement
            | SimpleStatements
            | ImportStatement
            | FutureImportStatement
            | ImportFromStatement
            | PrintStatement
            | AssertStatement
            | ReturnStatement
            | DeleteStatement
            | RaiseStatement
            | PassStatement
            | BreakStatement
            | ContinueStatement
            | IfStatement
            | ForStatement
            | WhileStatement
            | TryStatement
            | WithStatement
            | GlobalStatement
            | NonlocalStatement
            | ExecStatement
            | ExpressionStatement => {
                stats.lloc.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.ploc.lines.insert(start);
            }
        }
    }
}

impl Loc for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Mozjs::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | Program => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.lloc.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.ploc.lines.insert(start);
            }
        }
    }
}

impl Loc for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Javascript::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | Program => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.lloc.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.ploc.lines.insert(start);
            }
        }
    }
}

impl Loc for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Typescript::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | Program => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.lloc.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.ploc.lines.insert(start);
            }
        }
    }
}

impl Loc for TsxCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Tsx::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            String | DQUOTE | Program => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.lloc.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.ploc.lines.insert(start);
            }
        }
    }
}

impl Loc for RustCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Rust::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            StringLiteral | RawStringLiteral | Block | SourceFile => {}
            LineComment | BlockComment => {
                add_cloc_lines(stats, start, end);
            }
            Statement
            | EmptyStatement
            | ExpressionStatement
            | LetDeclaration
            | AssignmentExpression
            | CompoundAssignmentExpr => {
                stats.lloc.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.ploc.lines.insert(start);
            }
        }
    }
}

impl Loc for CppCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Cpp::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.kind_id().into() {
            RawStringLiteral | StringLiteral | DeclarationList | FieldDeclarationList
            | TranslationUnit => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            WhileStatement | SwitchStatement | CaseStatement | IfStatement | ForStatement
            | ReturnStatement | BreakStatement | ContinueStatement | GotoStatement
            | ThrowStatement | TryStatement | ExpressionStatement | LabeledStatement
            | StatementIdentifier => {
                stats.lloc.logical_lines += 1;
            }
            Declaration => {
                if count_specific_ancestors!(
                    node,
                    WhileStatement | ForStatement | IfStatement,
                    CompoundStatement
                ) == 0
                {
                    stats.lloc.logical_lines += 1;
                }
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.ploc.lines.insert(start);
            }
        }
    }
}

impl Loc for JavaCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Java::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);
        let kind_id: Java = node.kind_id().into();
        // LLOC in Java is counted for statements only
        // https://docs.oracle.com/javase/tutorial/java/nutsandbolts/expressions.html
        match kind_id {
            Program => {}
            LineComment | BlockComment => {
                add_cloc_lines(stats, start, end);
            }
            AssertStatement | BreakStatement | ContinueStatement | DoStatement
            | EnhancedForStatement | ExpressionStatement | ForStatement | IfStatement
            | ReturnStatement | SwitchExpression | ThrowStatement | TryStatement
            | WhileStatement => {
                stats.lloc.logical_lines += 1;
            }
            LocalVariableDeclaration => {
                if count_specific_ancestors!(node, ForStatement, Block) == 0 {
                    // The initializer, condition, and increment in a for loop are expressions.
                    // Don't count the variable declaration if in a ForStatement.
                    // https://docs.oracle.com/javase/tutorial/java/nutsandbolts/for.html
                    stats.lloc.logical_lines += 1;
                }
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.ploc.lines.insert(start);
            }
        }
    }
}

impl Loc for KotlinCode {}

impl Loc for PreprocCode {}
impl Loc for CcommentCode {}

#[cfg(test)]
mod tests {
    use crate::tools::check_metrics;

    use super::*;

    #[test]
    fn python_sloc() {
        check_metrics::<PythonParser>(
            "

            a = 42

            ",
            "foo.py",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_blank() {
        check_metrics::<PythonParser>(
            "
            a = 42

            b = 43

            ",
            "foo.py",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 2.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 1.0,
                      "sloc_average": 3.0,
                      "ploc_average": 2.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 1.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_blank() {
        check_metrics::<RustParser>(
            "

            let a = 42;

            let b = 43;

            ",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 2.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 1.0,
                      "sloc_average": 3.0,
                      "ploc_average": 2.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 1.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );

        check_metrics::<RustParser>("fn func() { /* comment */ }", "foo.rs", |metric| {
            // Spaces: 2
            insta::assert_json_snapshot!(
                metric.loc,
                @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 0.0,
                      "cloc": 1.0,
                      "blank": 0.0,
                      "sloc_average": 0.5,
                      "ploc_average": 0.5,
                      "lloc_average": 0.0,
                      "cloc_average": 0.5,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 1.0,
                      "cloc_max": 1.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 0.0,
                      "lloc_max": 0.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn c_blank() {
        check_metrics::<CppParser>(
            "

            int a = 42;

            int b = 43;

            ",
            "foo.c",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 2.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 1.0,
                      "sloc_average": 3.0,
                      "ploc_average": 2.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 1.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_no_zero_blank() {
        // Checks that the blank metric is not equal to 0 when there are some
        // comments next to code lines.
        check_metrics::<PythonParser>(
            "def ConnectToUpdateServer():
                 pool = 4

                 updateServer = -42
                 isConnected = False
                 currTry = 0
                 numRetries = 10 # Number of IPC connection retries before
                                 # giving up.
                 numTries = 20 # Number of IPC connection tries before
                               # giving up.",
            "foo.py",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 10.0,
                      "ploc": 7.0,
                      "lloc": 6.0,
                      "cloc": 4.0,
                      "blank": 1.0,
                      "sloc_average": 5.0,
                      "ploc_average": 3.5,
                      "lloc_average": 3.0,
                      "cloc_average": 2.0,
                      "blank_average": 0.5,
                      "sloc_min": 9.0,
                      "sloc_max": 9.0,
                      "cloc_min": 2.0,
                      "cloc_max": 2.0,
                      "ploc_min": 7.0,
                      "ploc_max": 7.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_no_blank() {
        // Checks that the blank metric is equal to 0 when there are no blank
        // lines and there are comments next to code lines.
        check_metrics::<PythonParser>(
            "def ConnectToUpdateServer():
                 pool = 4
                 updateServer = -42
                 isConnected = False
                 currTry = 0
                 numRetries = 10 # Number of IPC connection retries before
                                 # giving up.
                 numTries = 20 # Number of IPC connection tries before
                               # giving up.",
            "foo.py",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 9.0,
                      "ploc": 7.0,
                      "lloc": 6.0,
                      "cloc": 4.0,
                      "blank": 0.0,
                      "sloc_average": 4.5,
                      "ploc_average": 3.5,
                      "lloc_average": 3.0,
                      "cloc_average": 2.0,
                      "blank_average": 0.0,
                      "sloc_min": 8.0,
                      "sloc_max": 8.0,
                      "cloc_min": 2.0,
                      "cloc_max": 2.0,
                      "ploc_min": 7.0,
                      "ploc_max": 7.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_no_zero_blank_more_comments() {
        // Checks that the blank metric is not equal to 0 when there are more
        // comments next to code lines compared to the previous tests.
        check_metrics::<PythonParser>(
            "def ConnectToUpdateServer():
                 pool = 4

                 updateServer = -42
                 isConnected = False
                 currTry = 0 # Set this variable to 0
                 numRetries = 10 # Number of IPC connection retries before
                                 # giving up.
                 numTries = 20 # Number of IPC connection tries before
                               # giving up.",
            "foo.py",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 10.0,
                      "ploc": 7.0,
                      "lloc": 6.0,
                      "cloc": 5.0,
                      "blank": 1.0,
                      "sloc_average": 5.0,
                      "ploc_average": 3.5,
                      "lloc_average": 3.0,
                      "cloc_average": 2.5,
                      "blank_average": 0.5,
                      "sloc_min": 9.0,
                      "sloc_max": 9.0,
                      "cloc_min": 3.0,
                      "cloc_max": 3.0,
                      "ploc_min": 7.0,
                      "ploc_max": 7.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_no_zero_blank() {
        // Checks that the blank metric is not equal to 0 when there are some
        // comments next to code lines.
        check_metrics::<RustParser>(
            "fn ConnectToUpdateServer() {
              let pool = 0;

              let updateServer = -42;
              let isConnected = false;
              let currTry = 0;
              let numRetries = 10;  // Number of IPC connection retries before
                                    // giving up.
              let numTries = 20;    // Number of IPC connection tries before
                                    // giving up.
            }",
            "foo.rs",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 11.0,
                      "ploc": 8.0,
                      "lloc": 6.0,
                      "cloc": 4.0,
                      "blank": 1.0,
                      "sloc_average": 5.5,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 2.0,
                      "blank_average": 0.5,
                      "sloc_min": 11.0,
                      "sloc_max": 11.0,
                      "cloc_min": 4.0,
                      "cloc_max": 4.0,
                      "ploc_min": 8.0,
                      "ploc_max": 8.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn javascript_no_zero_blank() {
        // Checks that the blank metric is not equal to 0 when there are some
        // comments next to code lines.
        check_metrics::<JavascriptParser>(
            "function ConnectToUpdateServer() {
              var pool = 0;

              var updateServer = -42;
              var isConnected = false;
              var currTry = 0;
              var numRetries = 10;  // Number of IPC connection retries before
                                    // giving up.
              var numTries = 20;    // Number of IPC connection tries before
                                    // giving up.
            }",
            "foo.js",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 11.0,
                      "ploc": 8.0,
                      "lloc": 1.0,
                      "cloc": 4.0,
                      "blank": 1.0,
                      "sloc_average": 5.5,
                      "ploc_average": 4.0,
                      "lloc_average": 0.5,
                      "cloc_average": 2.0,
                      "blank_average": 0.5,
                      "sloc_min": 11.0,
                      "sloc_max": 11.0,
                      "cloc_min": 4.0,
                      "cloc_max": 4.0,
                      "ploc_min": 8.0,
                      "ploc_max": 8.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_no_zero_blank() {
        // Checks that the blank metric is not equal to 0 when there are some
        // comments next to code lines.
        check_metrics::<CppParser>(
            "void ConnectToUpdateServer() {
              int pool;

              int updateServer = -42;
              bool isConnected = false;
              int currTry = 0;
              const int numRetries = 10; // Number of IPC connection retries before
                                         // giving up.
              const int numTries = 20; // Number of IPC connection tries before
                                       // giving up.
            }",
            "foo.cpp",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 11.0,
                      "ploc": 8.0,
                      "lloc": 6.0,
                      "cloc": 4.0,
                      "blank": 1.0,
                      "sloc_average": 5.5,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 2.0,
                      "blank_average": 0.5,
                      "sloc_min": 11.0,
                      "sloc_max": 11.0,
                      "cloc_min": 4.0,
                      "cloc_max": 4.0,
                      "ploc_min": 8.0,
                      "ploc_max": 8.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_code_line_start_block_blank() {
        // Checks that the blank metric is equal to 1 when there are
        // block comments starting next to code lines.
        check_metrics::<CppParser>(
            "void ConnectToUpdateServer() {
              int pool;

              int updateServer = -42;
              bool isConnected = false;
              int currTry = 0;
              const int numRetries = 10; /* Number of IPC connection retries
              before
              giving up. */
              const int numTries = 20; // Number of IPC connection tries before
                                       // giving up.
            }",
            "foo.cpp",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 12.0,
                      "ploc": 8.0,
                      "lloc": 6.0,
                      "cloc": 5.0,
                      "blank": 1.0,
                      "sloc_average": 6.0,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 2.5,
                      "blank_average": 0.5,
                      "sloc_min": 12.0,
                      "sloc_max": 12.0,
                      "cloc_min": 5.0,
                      "cloc_max": 5.0,
                      "ploc_min": 8.0,
                      "ploc_max": 8.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_block_comment_blank() {
        // Checks that the blank metric is equal to 1 when there are
        // block comments on independent lines.
        check_metrics::<CppParser>(
            "void ConnectToUpdateServer() {
              int pool;

              int updateServer = -42;
              bool isConnected = false;
              int currTry = 0;
              /* Number of IPC connection retries
              before
              giving up. */
              const int numRetries = 10;
              const int numTries = 20; // Number of IPC connection tries before
                                       // giving up.
            }",
            "foo.cpp",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 13.0,
                      "ploc": 8.0,
                      "lloc": 6.0,
                      "cloc": 5.0,
                      "blank": 1.0,
                      "sloc_average": 6.5,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 2.5,
                      "blank_average": 0.5,
                      "sloc_min": 13.0,
                      "sloc_max": 13.0,
                      "cloc_min": 5.0,
                      "cloc_max": 5.0,
                      "ploc_min": 8.0,
                      "ploc_max": 8.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_code_line_block_one_line_blank() {
        // Checks that the blank metric is equal to 1 when there are
        // block comments before the same code line.
        check_metrics::<CppParser>(
            "void ConnectToUpdateServer() {
              int pool;

              int updateServer = -42;
              bool isConnected = false;
              int currTry = 0;
              /* Number of IPC connection retries before giving up. */ const int numRetries = 10;
              const int numTries = 20; // Number of IPC connection tries before
                                       // giving up.
            }",
            "foo.cpp",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 10.0,
                      "ploc": 8.0,
                      "lloc": 6.0,
                      "cloc": 3.0,
                      "blank": 1.0,
                      "sloc_average": 5.0,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 1.5,
                      "blank_average": 0.5,
                      "sloc_min": 10.0,
                      "sloc_max": 10.0,
                      "cloc_min": 3.0,
                      "cloc_max": 3.0,
                      "ploc_min": 8.0,
                      "ploc_max": 8.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_code_line_end_block_blank() {
        // Checks that the blank metric is equal to 1 when there are
        // block comments ending next to code lines.
        check_metrics::<CppParser>(
            "void ConnectToUpdateServer() {
              int pool;

              int updateServer = -42;
              bool isConnected = false;
              int currTry = 0;
              /* Number of IPC connection retries
              before
              giving up. */ const int numRetries = 10;
              const int numTries = 20; // Number of IPC connection tries before
                                       // giving up.
            }",
            "foo.cpp",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 12.0,
                      "ploc": 8.0,
                      "lloc": 6.0,
                      "cloc": 5.0,
                      "blank": 1.0,
                      "sloc_average": 6.0,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 2.5,
                      "blank_average": 0.5,
                      "sloc_min": 12.0,
                      "sloc_max": 12.0,
                      "cloc_min": 5.0,
                      "cloc_max": 5.0,
                      "ploc_min": 8.0,
                      "ploc_max": 8.0,
                      "lloc_min": 6.0,
                      "lloc_max": 6.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_cloc() {
        check_metrics::<PythonParser>(
            "\"\"\"Block comment
            Block comment
            \"\"\"
            # Line Comment
            a = 42 # Line Comment",
            "foo.py",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 5.0,
                      "ploc": 1.0,
                      "lloc": 2.0,
                      "cloc": 5.0,
                      "blank": 0.0,
                      "sloc_average": 5.0,
                      "ploc_average": 1.0,
                      "lloc_average": 2.0,
                      "cloc_average": 5.0,
                      "blank_average": 0.0,
                      "sloc_min": 5.0,
                      "sloc_max": 5.0,
                      "cloc_min": 5.0,
                      "cloc_max": 5.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_cloc() {
        check_metrics::<RustParser>(
            "/*Block comment
            Block Comment*/
            //Line Comment
            /*Block Comment*/ let a = 42; // Line Comment",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 4.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 5.0,
                      "blank": 0.0,
                      "sloc_average": 4.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 5.0,
                      "blank_average": 0.0,
                      "sloc_min": 4.0,
                      "sloc_max": 4.0,
                      "cloc_min": 5.0,
                      "cloc_max": 5.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_cloc() {
        check_metrics::<CppParser>(
            "/*Block comment
            Block Comment*/
            //Line Comment
            /*Block Comment*/ int a = 42; // Line Comment",
            "foo.c",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 4.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 5.0,
                      "blank": 0.0,
                      "sloc_average": 4.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 5.0,
                      "blank_average": 0.0,
                      "sloc_min": 4.0,
                      "sloc_max": 4.0,
                      "cloc_min": 5.0,
                      "cloc_max": 5.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_lloc() {
        check_metrics::<PythonParser>(
            "for x in range(0,42):
                if x % 2 == 0:
                    print(x)",
            "foo.py",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 3.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 3.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_lloc() {
        check_metrics::<RustParser>(
            "for x in 0..42 {
                if x % 2 == 0 {
                    println!(\"{}\", x);
                }
             }",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 5.0,
                      "ploc": 5.0,
                      "lloc": 3.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 5.0,
                      "ploc_average": 5.0,
                      "lloc_average": 3.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 5.0,
                      "sloc_max": 5.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 5.0,
                      "ploc_max": 5.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );

        // LLOC returns three because there is an empty Rust statement
        check_metrics::<RustParser>(
            "let a = 42;
             if true {
                42
             } else {
                43
             };",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 6.0,
                      "ploc": 6.0,
                      "lloc": 3.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 6.0,
                      "ploc_average": 6.0,
                      "lloc_average": 3.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 6.0,
                      "sloc_max": 6.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 6.0,
                      "ploc_max": 6.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn c_lloc() {
        check_metrics::<CppParser>(
            "for (;;)
                break;",
            "foo.c",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 2.0,
                      "ploc": 2.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 2.0,
                      "ploc_average": 2.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 2.0,
                      "sloc_max": 2.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_lloc() {
        check_metrics::<CppParser>(
            "nsTArray<xpcGCCallback> callbacks(extraGCCallbacks.Clone());
             for (uint32_t i = 0; i < callbacks.Length(); ++i) {
                 callbacks[i](status);
             }",
            "foo.cpp",
            |metric| {
                // Spaces: 1
                // lloc: nsTArray, for, callbacks
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 4.0,
                      "ploc": 4.0,
                      "lloc": 3.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 4.0,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 4.0,
                      "sloc_max": 4.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 4.0,
                      "ploc_max": 4.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_return_lloc() {
        check_metrics::<CppParser>(
            "uint8_t* pixel_data = frame.GetFrameDataAtPos(DesktopVector(x, y));
             return RgbaColor(pixel_data) == blank_pixel_;",
            "foo.cpp",
            |metric| {
                // Spaces: 1
                // lloc: pixel_data, return
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 2.0,
                      "ploc": 2.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 2.0,
                      "ploc_average": 2.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 2.0,
                      "sloc_max": 2.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_for_lloc() {
        check_metrics::<CppParser>(
            "for (; start != end; ++start) {
                 const unsigned char idx = *start;
                 if (idx > 127 || !kValidTokenMap[idx]) return false;
             }",
            "foo.cpp",
            |metric| {
                // Spaces: 1
                // lloc: for, idx, if, return
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 4.0,
                      "ploc": 4.0,
                      "lloc": 4.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 4.0,
                      "ploc_average": 4.0,
                      "lloc_average": 4.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 4.0,
                      "sloc_max": 4.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 4.0,
                      "ploc_max": 4.0,
                      "lloc_min": 4.0,
                      "lloc_max": 4.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_while_lloc() {
        check_metrics::<CppParser>(
            "while (sHeapAtoms) {
                 HttpHeapAtom* next = sHeapAtoms->next;
                 free(sHeapAtoms);
            }",
            "foo.cpp",
            |metric| {
                // Spaces: 1
                // lloc: while, next, free
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 4.0,
                      "ploc": 4.0,
                      "lloc": 3.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 4.0,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 4.0,
                      "sloc_max": 4.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 4.0,
                      "ploc_max": 4.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_string_on_new_line() {
        // More lines of the same instruction were counted as blank lines
        check_metrics::<PythonParser>(
            "capabilities[\"goog:chromeOptions\"][\"androidPackage\"] = \\
                \"org.chromium.weblayer.shell\"",
            "foo.py",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 2.0,
                      "ploc": 2.0,
                      "lloc": 1.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 2.0,
                      "ploc_average": 2.0,
                      "lloc_average": 1.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 2.0,
                      "sloc_max": 2.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_no_field_expression_lloc() {
        check_metrics::<RustParser>(
            "struct Foo {
                field: usize,
             }
             let foo = Foo { 42 };
             foo.field;",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 5.0,
                      "ploc": 5.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 5.0,
                      "ploc_average": 5.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 5.0,
                      "sloc_max": 5.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 5.0,
                      "ploc_max": 5.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_no_parenthesized_expression_lloc() {
        check_metrics::<RustParser>("let a = (42 + 0);", "foo.rs", |metric| {
            // Spaces: 1
            insta::assert_json_snapshot!(
                metric.loc,
                @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn rust_no_array_expression_lloc() {
        check_metrics::<RustParser>("let a = [0; 42];", "foo.rs", |metric| {
            // Spaces: 1
            insta::assert_json_snapshot!(
                metric.loc,
                @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn rust_no_tuple_expression_lloc() {
        check_metrics::<RustParser>("let a = (0, 42);", "foo.rs", |metric| {
            // Spaces: 1
            insta::assert_json_snapshot!(
                metric.loc,
                @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn rust_no_unit_expression_lloc() {
        check_metrics::<RustParser>("let a = ();", "foo.rs", |metric| {
            // Spaces: 1
            insta::assert_json_snapshot!(
                metric.loc,
                @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn rust_call_function_lloc() {
        check_metrics::<RustParser>(
            "let a = foo(); // +1
             foo(); // +1
             k!(foo()); // +1",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 3.0,
                      "cloc": 3.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 3.0,
                      "cloc_average": 3.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 3.0,
                      "cloc_max": 3.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_macro_invocation_lloc() {
        check_metrics::<RustParser>(
            "let a = foo!(); // +1
             foo!(); // +1
             k(foo!()); // +1",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 3.0,
                      "cloc": 3.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 3.0,
                      "cloc_average": 3.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 3.0,
                      "cloc_max": 3.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_function_in_loop_lloc() {
        check_metrics::<RustParser>(
            "for (a, b) in c.iter().enumerate() {} // +1
             while (a, b) in c.iter().enumerate() {} // +1
             while let Some(a) = c.strip_prefix(\"hi\") {} // +1",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 3.0,
                      "cloc": 3.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 3.0,
                      "cloc_average": 3.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 3.0,
                      "cloc_max": 3.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_function_in_if_lloc() {
        check_metrics::<RustParser>(
            "if foo() {} // +1
             if let Some(a) = foo() {} // +1",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 2.0,
                      "ploc": 2.0,
                      "lloc": 2.0,
                      "cloc": 2.0,
                      "blank": 0.0,
                      "sloc_average": 2.0,
                      "ploc_average": 2.0,
                      "lloc_average": 2.0,
                      "cloc_average": 2.0,
                      "blank_average": 0.0,
                      "sloc_min": 2.0,
                      "sloc_max": 2.0,
                      "cloc_min": 2.0,
                      "cloc_max": 2.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_function_in_return_lloc() {
        check_metrics::<RustParser>(
            "return foo();
             await foo();",
            "foo.rs",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 2.0,
                      "ploc": 2.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 2.0,
                      "ploc_average": 2.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 2.0,
                      "sloc_max": 2.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn rust_closure_expression_lloc() {
        check_metrics::<RustParser>(
            "let a = |i: i32| -> i32 { i + 1 }; // +1
             a(42); // +1
             k(b.iter().map(|n| n.parse.ok().unwrap_or(42))); // +1",
            "foo.rs",
            |metric| {
                // Spaces: 3
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 3.0,
                      "cloc": 3.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 1.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 0.0,
                      "lloc_max": 0.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_general_loc() {
        check_metrics::<PythonParser>(
            "def func(a,
                      b,
                      c):
                 print(a)
                 print(b)
                 print(c)",
            "foo.py",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 6.0,
                      "ploc": 6.0,
                      "lloc": 3.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 1.5,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 6.0,
                      "sloc_max": 6.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 6.0,
                      "ploc_max": 6.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn python_real_loc() {
        check_metrics::<PythonParser>(
            "def web_socket_transfer_data(request):
                while True:
                    line = request.ws_stream.receive_message()
                    if line is None:
                        return
                    code, reason = line.split(' ', 1)
                    if code is None or reason is None:
                        return
                    request.ws_stream.close_connection(int(code), reason)
                    # close_connection() initiates closing handshake. It validates code
                    # and reason. If you want to send a broken close frame for a test,
                    # following code will be useful.
                    # > data = struct.pack('!H', int(code)) + reason.encode('UTF-8')
                    # > request.connection.write(stream.create_close_frame(data))
                    # > # Suppress to re-respond client responding close frame.
                    # > raise Exception(\"customized server initiated closing handshake\")",
            "foo.py",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 16.0,
                      "ploc": 9.0,
                      "lloc": 8.0,
                      "cloc": 7.0,
                      "blank": 0.0,
                      "sloc_average": 8.0,
                      "ploc_average": 4.5,
                      "lloc_average": 4.0,
                      "cloc_average": 3.5,
                      "blank_average": 0.0,
                      "sloc_min": 16.0,
                      "sloc_max": 16.0,
                      "cloc_min": 7.0,
                      "cloc_max": 7.0,
                      "ploc_min": 9.0,
                      "ploc_max": 9.0,
                      "lloc_min": 8.0,
                      "lloc_max": 8.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn javascript_real_loc() {
        check_metrics::<JavascriptParser>(
            "assert.throws(Test262Error, function() {
               for (let { poisoned: x = ++initEvalCount } = poisonedProperty; ; ) {
                 return;
               }
             });",
            "foo.js",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 5.0,
                      "ploc": 5.0,
                      "lloc": 6.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 2.5,
                      "ploc_average": 2.5,
                      "lloc_average": 3.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 5.0,
                      "sloc_max": 5.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 5.0,
                      "ploc_max": 5.0,
                      "lloc_min": 5.0,
                      "lloc_max": 5.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn mozjs_real_loc() {
        check_metrics::<MozjsParser>(
            "assert.throws(Test262Error, function() {
               for (let { poisoned: x = ++initEvalCount } = poisonedProperty; ; ) {
                 return;
               }
             });",
            "foo.js",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 5.0,
                      "ploc": 5.0,
                      "lloc": 6.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 2.5,
                      "ploc_average": 2.5,
                      "lloc_average": 3.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 5.0,
                      "sloc_max": 5.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 5.0,
                      "ploc_max": 5.0,
                      "lloc_min": 5.0,
                      "lloc_max": 5.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn cpp_namespace_loc() {
        check_metrics::<CppParser>(
            "namespace mozilla::dom::quota {} // namespace mozilla::dom::quota",
            "foo.cpp",
            |metric| {
                // Spaces: 2
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 0.0,
                      "cloc": 1.0,
                      "blank": 0.0,
                      "sloc_average": 0.5,
                      "ploc_average": 0.5,
                      "lloc_average": 0.0,
                      "cloc_average": 0.5,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 0.0,
                      "lloc_max": 0.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_comments() {
        check_metrics::<JavaParser>(
            "for (int i = 0; i < 100; i++) { \
               // Print hello
               System.out.println(\"hello\"); \
               // Print world
               System.out.println(\"hello\"); \
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 3.0,
                      "cloc": 2.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 3.0,
                      "cloc_average": 2.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 2.0,
                      "cloc_max": 2.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_blank() {
        check_metrics::<JavaParser>(
            "int x = 1;


            int y = 2;",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 4.0,
                      "ploc": 2.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 2.0,
                      "sloc_average": 4.0,
                      "ploc_average": 2.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 2.0,
                      "sloc_min": 4.0,
                      "sloc_max": 4.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 2.0,
                      "ploc_max": 2.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 2.0,
                      "blank_max": 2.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_sloc() {
        check_metrics::<JavaParser>(
            "for (int i = 0; i < 100; i++) {
               System.out.println(i);
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_module_sloc() {
        check_metrics::<JavaParser>(
            "module helloworld{
              exports com.test;
            }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 0.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 0.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 0.0,
                      "lloc_max": 0.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_single_ploc() {
        check_metrics::<JavaParser>("int x = 1;", "foo.java", |metric| {
            // Spaces: 1
            insta::assert_json_snapshot!(
                metric.loc,
                @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn java_simple_ploc() {
        check_metrics::<JavaParser>(
            "for (int i = 0; i < 100; i = i++) {
               System.out.println(i);
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_multi_ploc() {
        check_metrics::<JavaParser>(
            "int x = 1;
            for (int i = 0; i < 100; i++) {
               System.out.println(i);
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 4.0,
                      "ploc": 4.0,
                      "lloc": 3.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 4.0,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 4.0,
                      "sloc_max": 4.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 4.0,
                      "ploc_max": 4.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_single_statement_lloc() {
        check_metrics::<JavaParser>("int max = 10;", "foo.java", |metric| {
            // Spaces: 1
            insta::assert_json_snapshot!(
                metric.loc,
                @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 1.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 1.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
            );
        });
    }

    #[test]
    fn java_for_lloc() {
        check_metrics::<JavaParser>(
            "for (int i = 0; i < 100; i++) { // + 1
               System.out.println(i); // + 1
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 3.0,
                      "ploc": 3.0,
                      "lloc": 2.0,
                      "cloc": 2.0,
                      "blank": 0.0,
                      "sloc_average": 3.0,
                      "ploc_average": 3.0,
                      "lloc_average": 2.0,
                      "cloc_average": 2.0,
                      "blank_average": 0.0,
                      "sloc_min": 3.0,
                      "sloc_max": 3.0,
                      "cloc_min": 2.0,
                      "cloc_max": 2.0,
                      "ploc_min": 3.0,
                      "ploc_max": 3.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_foreach_lloc() {
        check_metrics::<JavascriptParser>(
            "
            int arr[]={12,13,14,44}; // +1
            for (int i:arr) { // +1
               System.out.println(i); // +1
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 4.0,
                      "ploc": 4.0,
                      "lloc": 1.0,
                      "cloc": 3.0,
                      "blank": 0.0,
                      "sloc_average": 4.0,
                      "ploc_average": 4.0,
                      "lloc_average": 1.0,
                      "cloc_average": 3.0,
                      "blank_average": 0.0,
                      "sloc_min": 4.0,
                      "sloc_max": 4.0,
                      "cloc_min": 3.0,
                      "cloc_max": 3.0,
                      "ploc_min": 4.0,
                      "ploc_max": 4.0,
                      "lloc_min": 1.0,
                      "lloc_max": 1.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_while_lloc() {
        check_metrics::<JavaParser>(
            "
            int i=0; // +1
            while(i < 10) { // +1
                i++; // +1
                System.out.println(i); // +1
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 5.0,
                      "ploc": 5.0,
                      "lloc": 4.0,
                      "cloc": 4.0,
                      "blank": 0.0,
                      "sloc_average": 5.0,
                      "ploc_average": 5.0,
                      "lloc_average": 4.0,
                      "cloc_average": 4.0,
                      "blank_average": 0.0,
                      "sloc_min": 5.0,
                      "sloc_max": 5.0,
                      "cloc_min": 4.0,
                      "cloc_max": 4.0,
                      "ploc_min": 5.0,
                      "ploc_max": 5.0,
                      "lloc_min": 4.0,
                      "lloc_max": 4.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_do_while_lloc() {
        check_metrics::<JavaParser>(
            "
            int i=0; // +1
            do { // +1
                i++; // +1
                System.out.println(i); // +1
             } while(i < 10)",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 5.0,
                      "ploc": 5.0,
                      "lloc": 4.0,
                      "cloc": 4.0,
                      "blank": 0.0,
                      "sloc_average": 5.0,
                      "ploc_average": 5.0,
                      "lloc_average": 4.0,
                      "cloc_average": 4.0,
                      "blank_average": 0.0,
                      "sloc_min": 5.0,
                      "sloc_max": 5.0,
                      "cloc_min": 4.0,
                      "cloc_max": 4.0,
                      "ploc_min": 5.0,
                      "ploc_max": 5.0,
                      "lloc_min": 4.0,
                      "lloc_max": 4.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_switch_lloc() {
        check_metrics::<JavaParser>(
            "switch(grade) { // +1
                case 'A' :
                   System.out.println(\"Pass with distinction\"); // +1
                   break; // +1
                case 'B' :
                case 'C' :
                   System.out.println(\"Pass\"); // +1
                   break; // +1
                case 'D' :
                   System.out.println(\"At risk\"); // +1
                case 'F' :
                   System.out.println(\"Fail\"); // +1
                   break; // +1
                default :
                   System.out.println(\"Invalid grade\"); // +1
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 16.0,
                      "ploc": 16.0,
                      "lloc": 9.0,
                      "cloc": 9.0,
                      "blank": 0.0,
                      "sloc_average": 16.0,
                      "ploc_average": 16.0,
                      "lloc_average": 9.0,
                      "cloc_average": 9.0,
                      "blank_average": 0.0,
                      "sloc_min": 16.0,
                      "sloc_max": 16.0,
                      "cloc_min": 9.0,
                      "cloc_max": 9.0,
                      "ploc_min": 16.0,
                      "ploc_max": 16.0,
                      "lloc_min": 9.0,
                      "lloc_max": 9.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_continue_lloc() {
        check_metrics::<JavaParser>(
            "int max = 10; // +1

            for (int i = 0; i < max; i++) { // +1
                if(i % 2 == 0) { continue;} + 2
                System.out.println(i); // +1
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 6.0,
                      "ploc": 5.0,
                      "lloc": 5.0,
                      "cloc": 3.0,
                      "blank": 1.0,
                      "sloc_average": 6.0,
                      "ploc_average": 5.0,
                      "lloc_average": 5.0,
                      "cloc_average": 3.0,
                      "blank_average": 1.0,
                      "sloc_min": 6.0,
                      "sloc_max": 6.0,
                      "cloc_min": 3.0,
                      "cloc_max": 3.0,
                      "ploc_min": 5.0,
                      "ploc_max": 5.0,
                      "lloc_min": 5.0,
                      "lloc_max": 5.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_try_lloc() {
        check_metrics::<JavaParser>(
            "try { // +1
                int[] myNumbers = {1, 2, 3}; // +1
                System.out.println(myNumbers[10]); // +1
              } catch (Exception e) {
                System.out.println(e.getMessage()); // +1
                throw e; // +1
              }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 7.0,
                      "ploc": 7.0,
                      "lloc": 5.0,
                      "cloc": 5.0,
                      "blank": 0.0,
                      "sloc_average": 7.0,
                      "ploc_average": 7.0,
                      "lloc_average": 5.0,
                      "cloc_average": 5.0,
                      "blank_average": 0.0,
                      "sloc_min": 7.0,
                      "sloc_max": 7.0,
                      "cloc_min": 5.0,
                      "cloc_max": 5.0,
                      "ploc_min": 7.0,
                      "ploc_max": 7.0,
                      "lloc_min": 5.0,
                      "lloc_max": 5.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_class_loc() {
        check_metrics::<JavaParser>(
            "
            public class Person {
              private String name;
              public Person(String name){
                this.name = name; // +1
              }
              public String getName() {
                return name; // +1
              }
            }",
            "foo.java",
            |metric| {
                // Spaces: 4
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 9.0,
                      "ploc": 9.0,
                      "lloc": 2.0,
                      "cloc": 2.0,
                      "blank": 0.0,
                      "sloc_average": 2.25,
                      "ploc_average": 2.25,
                      "lloc_average": 0.5,
                      "cloc_average": 0.5,
                      "blank_average": 0.0,
                      "sloc_min": 9.0,
                      "sloc_max": 9.0,
                      "cloc_min": 2.0,
                      "cloc_max": 2.0,
                      "ploc_min": 9.0,
                      "ploc_max": 9.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_expressions_lloc() {
        check_metrics::<JavaParser>(
            "int x = 10;                                                            // +1 local var declaration
            x=+89;                                                                  // +1 expression statement
            int y = x * 2;                                                          // +1 local var declaration
            IntFunction double = (n) -> n*2;                                        // +1 local var declaration
            int y2 = double(x);                                                     // +1 local var declaration
            System.out.println(\"double \" + x + \" = \" + y2);                     // +1 expression statement
            String message = (x % 2) == 0 ? \"Evenly done.\" : \"Oddly done.\";     // +1 local var declaration
            Object done = (Runnable) () -> { System.out.println(\"Done!\"); };      // +2 local var declaration + expression statement
            String s = \"string\";                                                  // +1 local var declaration
            boolean isS = (s instanceof String);                                    // +1 local var declaration
            done.run();                                                             // +1 expression statement
            ",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 11.0,
                      "ploc": 11.0,
                      "lloc": 12.0,
                      "cloc": 11.0,
                      "blank": 0.0,
                      "sloc_average": 11.0,
                      "ploc_average": 11.0,
                      "lloc_average": 12.0,
                      "cloc_average": 11.0,
                      "blank_average": 0.0,
                      "sloc_min": 11.0,
                      "sloc_max": 11.0,
                      "cloc_min": 11.0,
                      "cloc_max": 11.0,
                      "ploc_min": 11.0,
                      "ploc_max": 11.0,
                      "lloc_min": 12.0,
                      "lloc_max": 12.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_statement_inline_loc() {
        check_metrics::<JavaParser>(
            "for (int i = 0; i < 100; i++) { System.out.println(\"hello\"); }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 1.0,
                      "ploc": 1.0,
                      "lloc": 2.0,
                      "cloc": 0.0,
                      "blank": 0.0,
                      "sloc_average": 1.0,
                      "ploc_average": 1.0,
                      "lloc_average": 2.0,
                      "cloc_average": 0.0,
                      "blank_average": 0.0,
                      "sloc_min": 1.0,
                      "sloc_max": 1.0,
                      "cloc_min": 0.0,
                      "cloc_max": 0.0,
                      "ploc_min": 1.0,
                      "ploc_max": 1.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_general_loc() {
        check_metrics::<JavaParser>(
            "int max = 100;

            /*
              Loop through and print
                from: 0
                to: max
            */
            for (int i = 0; i < max; i++) {
               // Print the value
               System.out.println(i);
             }",
            "foo.java",
            |metric| {
                // Spaces: 1
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 11.0,
                      "ploc": 4.0,
                      "lloc": 3.0,
                      "cloc": 6.0,
                      "blank": 1.0,
                      "sloc_average": 11.0,
                      "ploc_average": 4.0,
                      "lloc_average": 3.0,
                      "cloc_average": 6.0,
                      "blank_average": 1.0,
                      "sloc_min": 11.0,
                      "sloc_max": 11.0,
                      "cloc_min": 6.0,
                      "cloc_max": 6.0,
                      "ploc_min": 4.0,
                      "ploc_max": 4.0,
                      "lloc_min": 3.0,
                      "lloc_max": 3.0,
                      "blank_min": 1.0,
                      "blank_max": 1.0
                    }"###
                );
            },
        );
    }

    #[test]
    fn java_main_class_loc() {
        check_metrics::<JavaParser>(
            "package com.company;
             /**
             * The HelloWorldApp class implements an application that
             * simply prints \"Hello World!\" to standard output.
             */

            class HelloWorldApp {
              public void main(String[] args) {
                String message = args.length == 0 ? \"Hello empty world\" : \"Hello world\"; // +1 lloc : 1 var assignment
                System.out.println(message); // Display the string. +1 lloc
              }
            }",
            "foo.java",
            |metric| {
                // Spaces: 3
                insta::assert_json_snapshot!(
                    metric.loc,
                    @r###"
                    {
                      "sloc": 12.0,
                      "ploc": 7.0,
                      "lloc": 2.0,
                      "cloc": 6.0,
                      "blank": 1.0,
                      "sloc_average": 4.0,
                      "ploc_average": 2.3333333333333335,
                      "lloc_average": 0.6666666666666666,
                      "cloc_average": 2.0,
                      "blank_average": 0.3333333333333333,
                      "sloc_min": 6.0,
                      "sloc_max": 6.0,
                      "cloc_min": 2.0,
                      "cloc_max": 2.0,
                      "ploc_min": 6.0,
                      "ploc_max": 6.0,
                      "lloc_min": 2.0,
                      "lloc_max": 2.0,
                      "blank_min": 0.0,
                      "blank_max": 0.0
                    }"###
                );
            },
        );
    }
}
