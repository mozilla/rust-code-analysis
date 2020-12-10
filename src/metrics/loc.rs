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
    comment_lines: usize,
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
        self.comment_lines += other.comment_lines;
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
        self.comment_lines as f64
    }

    /// The `Blank` metric.
    ///
    /// Counts the number of blank lines in a scope
    #[inline(always)]
    pub fn blank(&self) -> f64 {
        // This metric counts the number of blank lines in a code
        // The if construct is needed because sometimes lloc and cloc
        // coincide on the same lines, in that case lloc + cloc could be greater
        // than the number of lines of a file.
        let blank = self.sloc() - self.ploc() - self.cloc();
        blank.max(0.0)
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

impl Loc for PythonCode {
    fn compute(node: &Node, stats: &mut Stats, is_func_space: bool, is_unit: bool) {
        use Python::*;

        let (start, end) = init(node, stats, is_func_space, is_unit);

        match node.object().kind_id().into() {
            DQUOTE | DQUOTE2 | Block | Module => {}
            Comment => {
                stats.comment_lines += (end - start) + 1;
            }
            String => {
                let parent = node.object().parent().unwrap();
                if let ExpressionStatement = parent.kind_id().into() {
                    stats.comment_lines += (end - start) + 1;
                } else if parent.start_position().row != start {
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
                stats.comment_lines += (end - start) + 1;
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            _ => {
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
                stats.comment_lines += (end - start) + 1;
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            _ => {
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
                stats.comment_lines += (end - start) + 1;
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            _ => {
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
                stats.comment_lines += (end - start) + 1;
            }
            ExpressionStatement | ExportStatement | ImportStatement | StatementBlock
            | IfStatement | SwitchStatement | ForStatement | ForInStatement | WhileStatement
            | DoStatement | TryStatement | WithStatement | BreakStatement | ContinueStatement
            | DebuggerStatement | ReturnStatement | ThrowStatement | EmptyStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }
            _ => {
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
                stats.comment_lines += (end - start) + 1;
            }
            Statement
            | EmptyStatement
            | ExpressionStatement
            | LetDeclaration
            | AssignmentExpression
            | CompoundAssignmentExpr
            | ReturnExpression
            | ArrayExpression
            | TupleExpression
            | UnitExpression
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
                    CallExpression | MacroInvocation | ClosureExpression | LetDeclaration,
                    SourceFile | FunctionItem
                ) == 0
                {
                    stats.logical_lines += 1;
                }
            }
            _ => {
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
                stats.comment_lines += (end - start) + 1;
            }
            WhileStatement | SwitchStatement | CaseStatement | IfStatement | ForStatement
            | ReturnStatement | BreakStatement | ContinueStatement | GotoStatement
            | ThrowStatement | TryStatement | ExpressionStatement | LabeledStatement
            | StatementIdentifier => {
                stats.logical_lines += 1;
            }

            _ => {
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
}
