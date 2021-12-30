use crate::checker::Checker;
use fxhash::FxHashSet;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::*;

/// The `Loc` metric suite.
#[derive(Debug, Clone, Default)]
pub struct Stats {
    start: usize,
    end: usize,
    unit: bool,
    lines: FxHashSet<usize>,
    logical_lines: usize,
    only_comment_lines: usize,
    code_comment_lines: usize,
    comment_line_end: Option<usize>,
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("loc", 5)?;
        st.serialize_field("sloc", &self.sloc())?;
        st.serialize_field("ploc", &self.ploc())?;
        st.serialize_field("lloc", &self.lloc())?;
        st.serialize_field("cloc", &self.cloc())?;
        st.serialize_field("blank", &self.blank())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sloc: {}, ploc: {}, lloc: {}, cloc: {}, blank: {}",
            self.sloc(),
            self.ploc(),
            self.lloc(),
            self.cloc(),
            self.blank(),
        )
    }
}

impl Stats {
    /// Merges a second `Loc` metric suite into the first one
    pub fn merge(&mut self, other: &Stats) {
        // Merge ploc lines
        for l in other.lines.iter() {
            self.lines.insert(*l);
        }

        // Merge lloc lines
        self.logical_lines += other.logical_lines;

        // Merge cloc lines
        self.only_comment_lines += other.only_comment_lines;
        self.code_comment_lines += other.code_comment_lines;
    }

    /// The `Sloc` metric.
    ///
    /// Counts the number of lines in a scope
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

    /// The `Ploc` metric.
    ///
    /// Counts the number of instruction lines in a scope
    #[inline(always)]
    pub fn ploc(&self) -> f64 {
        // This metric counts the number of instruction lines in a code
        // https://en.wikipedia.org/wiki/Source_lines_of_code
        self.lines.len() as f64
    }

    /// The `Lloc` metric.
    ///
    /// Counts the number of statements in a scope
    #[inline(always)]
    pub fn lloc(&self) -> f64 {
        // This metric counts the number of statements in a code
        // https://en.wikipedia.org/wiki/Source_lines_of_code
        self.logical_lines as f64
    }

    /// The `Cloc` metric.
    ///
    /// Counts the number of comments in a scope
    #[inline(always)]
    pub fn cloc(&self) -> f64 {
        // Comments are counted regardless of their placement
        // https://en.wikipedia.org/wiki/Source_lines_of_code
        (self.only_comment_lines + self.code_comment_lines) as f64
    }

    /// The `Blank` metric.
    ///
    /// Counts the number of blank lines in a scope
    #[inline(always)]
    pub fn blank(&self) -> f64 {
        self.sloc() - self.ploc() - self.only_comment_lines as f64
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
    let start = node.object().start_position().row;
    let end = node.object().end_position().row;

    if is_func_space {
        stats.start = start;
        stats.end = end;
        stats.unit = is_unit;
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
    let is_comment_after_code_line = stats.lines.contains(&start);
    if is_comment_after_code_line && comment_diff == 0 {
        // A comment is *entirely* next to a code line
        stats.code_comment_lines += 1;
    } else if is_comment_after_code_line && comment_diff > 0 {
        // A block comment that starts next to a code line and ends on
        // independent lines.
        stats.code_comment_lines += 1;
        stats.only_comment_lines += comment_diff;
    } else {
        // A comment on an independent line AND
        // a block comment on independent lines OR
        // a comment *before* a code line
        stats.only_comment_lines += (end - start) + 1;
        // Save line end of a comment to check whether
        // a comment *before* a code line is considered
        stats.comment_line_end = Some(end);
    }
}

#[inline(always)]
// Detects the comments that are on a code line but *before* the code part.
// This difference is necessary in order to avoid having
// a wrong count for the blank metric.
fn check_comment_ends_on_code_line(stats: &mut Stats, start_code_line: usize) {
    if let Some(end) = stats.comment_line_end {
        if end == start_code_line && !stats.lines.contains(&start_code_line) {
            // Comment entirely *before* a code line
            stats.only_comment_lines -= 1;
            stats.code_comment_lines += 1;
        }
    }
}

impl Loc for PythonCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Python::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.object().kind_id().into() {
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
                    stats.lines.insert(start);
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
                stats.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Mozjs::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.object().kind_id().into() {
            String | DQUOTE | Program => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Javascript::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.object().kind_id().into() {
            String | DQUOTE | Program => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Typescript::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.object().kind_id().into() {
            String | DQUOTE | Program => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for TsxCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Tsx::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.object().kind_id().into() {
            String | DQUOTE | Program => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for RustCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Rust::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.object().kind_id().into() {
            StringLiteral | RawStringLiteral | Block | SourceFile => {}
            LineComment | BlockComment => {
                add_cloc_lines(stats, start, end);
            }
            Statement
            | EmptyStatement
            | ExpressionStatement
            | LetDeclaration
            | AssignmentExpression
            | CompoundAssignmentExpr
            | ReturnExpression
            | IfExpression
            | IfLetExpression
            | WhileExpression
            | WhileLetExpression
            | LoopExpression
            | ForExpression
            | BreakExpression
            | ContinueExpression
            | AwaitExpression => {
                stats.logical_lines += 1;
            }
            CallExpression | MacroInvocation | ClosureExpression => {
                if count_specific_ancestors!(
                    node,
                    CallExpression
                        | MacroInvocation
                        | ClosureExpression
                        | LetDeclaration
                        | WhileExpression
                        | WhileLetExpression
                        | ForExpression
                        | IfExpression
                        | IfLetExpression
                        | ReturnExpression
                        | AwaitExpression,
                    Block
                ) == 0
                {
                    stats.logical_lines += 1;
                }
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for CppCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Cpp::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.object().kind_id().into() {
            RawStringLiteral | StringLiteral | DeclarationList | FieldDeclarationList
            | TranslationUnit => {}
            Comment => {
                add_cloc_lines(stats, start, end);
            }
            WhileStatement | SwitchStatement | CaseStatement | IfStatement | ForStatement
            | ReturnStatement | BreakStatement | ContinueStatement | GotoStatement
            | ThrowStatement | TryStatement | ExpressionStatement | LabeledStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            Declaration => {
                if count_specific_ancestors!(
                    node,
                    WhileStatement | ForStatement | IfStatement,
                    CompoundStatement
                ) == 0
                {
                    stats.logical_lines += 1;
                }
            }
            _ => {
                check_comment_ends_on_code_line(stats, start);
                stats.lines.insert(start);
            }
        }
    }
}

impl Loc for PreprocCode {}
impl Loc for CcommentCode {}
impl Loc for JavaCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn python_sloc() {
        check_metrics!(
            "

            a = 42

            ",
            "foo.py",
            PythonParser,
            loc,
            [(sloc, 1, usize)]
        );
    }

    #[test]
    fn python_blank() {
        check_metrics!(
            "
            a = 42

            b = 43

            ",
            "foo.py",
            PythonParser,
            loc,
            [(blank, 1, usize)]
        );
    }

    #[test]
    fn rust_blank() {
        check_metrics!(
            "

            let a = 42;

            let b = 43;

            ",
            "foo.rs",
            RustParser,
            loc,
            [(blank, 1, usize)]
        );

        check_metrics!(
            "fn func() { /* comment */ }",
            "foo.rs",
            RustParser,
            loc,
            [(blank, 0, usize)]
        );
    }

    #[test]
    fn c_blank() {
        check_metrics!(
            "

            int a = 42;

            int b = 43;

            ",
            "foo.c",
            CppParser,
            loc,
            [(blank, 1, usize)]
        );
    }

    #[test]
    fn python_no_zero_blank() {
        // Checks that the blank metric is not equal to 0 when there are some
        // comments next to code lines.
        check_metrics!(
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
            PythonParser,
            loc,
            [
                (sloc, 10, usize), // The number of lines is 10
                (ploc, 7, usize),  // The number of code lines is 7
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 4, usize),  // The number of comments is 4
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn python_no_blank() {
        // Checks that the blank metric is equal to 0 when there are no blank
        // lines and there are comments next to code lines.
        check_metrics!(
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
            PythonParser,
            loc,
            [
                (sloc, 9, usize),  // The number of lines is 9
                (ploc, 7, usize),  // The number of code lines is 7
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 4, usize),  // The number of comments is 4
                (blank, 0, usize)  // The number of blank lines is 0
            ]
        );
    }

    #[test]
    fn python_no_zero_blank_more_comments() {
        // Checks that the blank metric is not equal to 0 when there are more
        // comments next to code lines compared to the previous tests.
        check_metrics!(
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
            PythonParser,
            loc,
            [
                (sloc, 10, usize), // The number of lines is 10
                (ploc, 7, usize),  // The number of code lines is 7
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 5, usize),  // The number of comments is 5
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn rust_no_zero_blank() {
        // Checks that the blank metric is not equal to 0 when there are some
        // comments next to code lines.
        check_metrics!(
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
            RustParser,
            loc,
            [
                (sloc, 11, usize), // The number of lines is 11
                (ploc, 8, usize),  // The number of code lines is 8
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 4, usize),  // The number of comments is 4
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn javascript_no_zero_blank() {
        // Checks that the blank metric is not equal to 0 when there are some
        // comments next to code lines.
        check_metrics!(
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
            JavascriptParser,
            loc,
            [
                (sloc, 11, usize), // The number of lines is 11
                (ploc, 8, usize),  // The number of code lines is 8
                (lloc, 1, usize),  // The number of statements is 1
                (cloc, 4, usize),  // The number of comments is 4
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn cpp_no_zero_blank() {
        // Checks that the blank metric is not equal to 0 when there are some
        // comments next to code lines.
        check_metrics!(
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
            CppParser,
            loc,
            [
                (sloc, 11, usize), // The number of lines is 11
                (ploc, 8, usize),  // The number of code lines is 8
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 4, usize),  // The number of comments is 4
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn cpp_code_line_start_block_blank() {
        // Checks that the blank metric is equal to 1 when there are
        // block comments starting next to code lines.
        check_metrics!(
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
            CppParser,
            loc,
            [
                (sloc, 12, usize), // The number of lines is 12
                (ploc, 8, usize),  // The number of code lines is 8
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 5, usize),  // The number of comments is 5
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn cpp_block_comment_blank() {
        // Checks that the blank metric is equal to 1 when there are
        // block comments on independent lines.
        check_metrics!(
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
            CppParser,
            loc,
            [
                (sloc, 13, usize), // The number of lines is 13
                (ploc, 8, usize),  // The number of code lines is 8
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 5, usize),  // The number of comments is 5
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn cpp_code_line_block_one_line_blank() {
        // Checks that the blank metric is equal to 1 when there are
        // block comments before the same code line.
        check_metrics!(
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
            CppParser,
            loc,
            [
                (sloc, 10, usize), // The number of lines is 10
                (ploc, 8, usize),  // The number of code lines is 8
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 3, usize),  // The number of comments is 3
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn cpp_code_line_end_block_blank() {
        // Checks that the blank metric is equal to 1 when there are
        // block comments ending next to code lines.
        check_metrics!(
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
            CppParser,
            loc,
            [
                (sloc, 12, usize), // The number of lines is 12
                (ploc, 8, usize),  // The number of code lines is 8
                (lloc, 6, usize),  // The number of statements is 6
                (cloc, 5, usize),  // The number of comments is 5
                (blank, 1, usize)  // The number of blank lines is 1
            ]
        );
    }

    #[test]
    fn python_cloc() {
        check_metrics!(
            "\"\"\"Block comment
            Block comment
            \"\"\"
            # Line Comment
            a = 42 # Line Comment",
            "foo.py",
            PythonParser,
            loc,
            [(cloc, 5, usize)]
        );
    }

    #[test]
    fn rust_cloc() {
        check_metrics!(
            "/*Block comment
            Block Comment*/
            //Line Comment
            /*Block Comment*/ let a = 42; // Line Comment",
            "foo.rs",
            RustParser,
            loc,
            [(cloc, 5, usize)]
        );
    }

    #[test]
    fn c_cloc() {
        check_metrics!(
            "/*Block comment
            Block Comment*/
            //Line Comment
            /*Block Comment*/ int a = 42; // Line Comment",
            "foo.c",
            CppParser,
            loc,
            [(cloc, 5, usize)]
        );
    }

    #[test]
    fn python_lloc() {
        check_metrics!(
            "for x in range(0,42):
                if x % 2 == 0:
                    print(x)",
            "foo.py",
            PythonParser,
            loc,
            [(lloc, 3, usize)]
        );
    }

    #[test]
    fn rust_lloc() {
        check_metrics!(
            "for x in 0..42 {
                if x % 2 == 0 {
                    println!(\"{}\", x);
                }
             }",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 3, usize)]
        );

        // LLOC returns three because there is an empty Rust statement
        check_metrics!(
            "let a = 42;
             if true {
                42
             } else {
                43
             };",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 3, usize)]
        );
    }

    #[test]
    fn c_lloc() {
        check_metrics!(
            "for (;;)
                break;",
            "foo.c",
            CppParser,
            loc,
            [(lloc, 2, usize)]
        );
    }

    #[test]
    fn cpp_lloc() {
        check_metrics!(
            "nsTArray<xpcGCCallback> callbacks(extraGCCallbacks.Clone());
             for (uint32_t i = 0; i < callbacks.Length(); ++i) {
                 callbacks[i](status);
             }",
            "foo.cpp",
            CppParser,
            loc,
            [(lloc, 3, usize)] // nsTArray, for, callbacks
        );
    }

    #[test]
    fn cpp_return_lloc() {
        check_metrics!(
            "uint8_t* pixel_data = frame.GetFrameDataAtPos(DesktopVector(x, y));
             return RgbaColor(pixel_data) == blank_pixel_;",
            "foo.cpp",
            CppParser,
            loc,
            [(lloc, 2, usize)] // pixel_data, return
        );
    }

    #[test]
    fn cpp_for_lloc() {
        check_metrics!(
            "for (; start != end; ++start) {
                 const unsigned char idx = *start;
                 if (idx > 127 || !kValidTokenMap[idx]) return false;
             }",
            "foo.cpp",
            CppParser,
            loc,
            [(lloc, 4, usize)] // for, idx, if, return
        );
    }

    #[test]
    fn cpp_while_lloc() {
        check_metrics!(
            "while (sHeapAtoms) {
                 HttpHeapAtom* next = sHeapAtoms->next;
                 free(sHeapAtoms);
            }",
            "foo.cpp",
            CppParser,
            loc,
            [(lloc, 3, usize)] // while, next, free,
        );
    }

    #[test]
    fn python_string_on_new_line() {
        // More lines of the same instruction were counted as blank lines
        check_metrics!(
            "capabilities[\"goog:chromeOptions\"][\"androidPackage\"] = \\
                \"org.chromium.weblayer.shell\"",
            "foo.py",
            PythonParser,
            loc,
            [
                (sloc, 2, usize),
                (ploc, 2, usize),
                (lloc, 1, usize),
                (cloc, 0, usize),
                (blank, 0, usize)
            ]
        );
    }

    #[test]
    fn rust_no_field_expression_lloc() {
        check_metrics!(
            "struct Foo {
                field: usize,
             }
             let foo = Foo { 42 };
             foo.field",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 1, usize)]
        );
    }

    #[test]
    fn rust_no_parenthesized_expression_lloc() {
        check_metrics!(
            "let a = (42 + 0);",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 1, usize)]
        );
    }

    #[test]
    fn rust_no_array_expression_lloc() {
        check_metrics!(
            "let a = [0; 42];",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 1, usize)]
        );
    }

    #[test]
    fn rust_no_tuple_expression_lloc() {
        check_metrics!(
            "let a = (0, 42);",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 1, usize)]
        );
    }

    #[test]
    fn rust_no_unit_expression_lloc() {
        check_metrics!("let a = ();", "foo.rs", RustParser, loc, [(lloc, 1, usize)]);
    }

    #[test]
    fn rust_call_function_lloc() {
        check_metrics!(
            "let a = foo(); // +1
             foo(); // +1
             k!(foo()); // +1",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 3, usize)]
        );
    }

    #[test]
    fn rust_macro_invocation_lloc() {
        check_metrics!(
            "let a = foo!(); // +1
             foo!(); // +1
             k(foo!()); // +1",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 3, usize)]
        );
    }

    #[test]
    fn rust_function_in_loop_lloc() {
        check_metrics!(
            "for (a, b) in c.iter().enumerate() {} // +1
             while (a, b) in c.iter().enumerate() {} // +1
             while let Some(a) = c.strip_prefix(\"hi\") {} // +1",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 3, usize)]
        );
    }

    #[test]
    fn rust_function_in_if_lloc() {
        check_metrics!(
            "if foo() {} // +1
             if let Some(a) = foo() {} // +1",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 2, usize)]
        );
    }

    #[test]
    fn rust_function_in_return_lloc() {
        check_metrics!(
            "return foo();
             await foo();",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 2, usize)]
        );
    }

    #[test]
    fn rust_closure_expression_lloc() {
        check_metrics!(
            "let a = |i: i32| -> i32 { i + 1 }; // +1
             a(42); // +1
             k(b.iter().map(|n| n.parse.ok().unwrap_or(42))); // +1",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 3, usize)]
        );
    }

    #[test]
    fn python_general_loc() {
        check_metrics!(
            "def func(a,
                      b,
                      c):
                 print(a)
                 print(b)
                 print(c)",
            "foo.py",
            PythonParser,
            loc,
            [
                (sloc, 6, usize),  // The number of lines is 6
                (ploc, 6, usize),  // The number of code lines is 6
                (lloc, 3, usize),  // The number of statements is 3 (print)
                (cloc, 0, usize),  // The number of comments is 0
                (blank, 0, usize)  // The number of blank lines is 0
            ]
        );
    }

    #[test]
    fn python_real_loc() {
        check_metrics!(
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
            PythonParser,
            loc,
            [
                (sloc, 16, usize), // The number of lines is 16
                (ploc, 9, usize),  // The number of code lines is 9
                (lloc, 8, usize),  // The number of statements is 8
                (cloc, 7, usize),  // The number of comments is 7
                (blank, 0, usize)  // The number of blank lines is 0
            ]
        );
    }

    #[test]
    fn javascript_real_loc() {
        check_metrics!(
            "assert.throws(Test262Error, function() {
               for (let { poisoned: x = ++initEvalCount } = poisonedProperty; ; ) {
                 return;
               }
             });",
            "foo.js",
            JavascriptParser,
            loc,
            [
                (sloc, 5, usize),  // The number of lines is 5
                (ploc, 5, usize),  // The number of code lines is 5
                (lloc, 7, usize),  // The number of statements is 7
                (cloc, 0, usize),  // The number of comments is 0
                (blank, 0, usize)  // The number of blank lines is 0
            ]
        );
    }

    #[test]
    fn mozjs_real_loc() {
        check_metrics!(
            "assert.throws(Test262Error, function() {
               for (let { poisoned: x = ++initEvalCount } = poisonedProperty; ; ) {
                 return;
               }
             });",
            "foo.js",
            MozjsParser,
            loc,
            [
                (sloc, 5, usize),  // The number of lines is 5
                (ploc, 5, usize),  // The number of code lines is 5
                (lloc, 7, usize),  // The number of statements is 7
                (cloc, 0, usize),  // The number of comments is 0
                (blank, 0, usize)  // The number of blank lines is 0
            ]
        );
    }
}
