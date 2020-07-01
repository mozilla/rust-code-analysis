use crate::checker::Checker;
use fxhash::FxHashSet;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;
use tree_sitter::Node;

use crate::*;

/// The `Loc` metric suite.
#[derive(Debug, Default)]
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

    /// The `Ploc` metric.
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
    let start = node.start_position().row;
    let end = node.end_position().row;

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

        match node.kind_id().into() {
            DQUOTE | DQUOTE2 | Block | Module => {}
            Comment => {
                stats.comment_lines += (end - start) + 1;
            }
            String => {
                let parent = node.parent().unwrap();
                if let ExpressionStatement = parent.kind_id().into() {
                    stats.comment_lines += (end - start) + 1;
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

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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

        match node.kind_id().into() {
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
            | CallExpression
            | ArrayExpression
            | ParenthesizedExpression
            | TupleExpression
            | UnitExpression
            | IfExpression
            | IfLetExpression
            | WhileExpression
            | WhileLetExpression
            | LoopExpression
            | ForExpression
            | ClosureExpression
            | BreakExpression
            | ContinueExpression
            | IndexExpression
            | AwaitExpression
            | FieldExpression
            | MacroInvocation => {
                stats.logical_lines += 1;
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

        match node.kind_id().into() {
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
impl Loc for CSharpCode {}
impl Loc for JavaCode {}
impl Loc for GoCode {}
impl Loc for CssCode {}
impl Loc for HtmlCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_blank() {
        check_metrics!(
            "\na = 42\n\n",
            "foo.py",
            PythonParser,
            loc,
            [(blank, 1, usize)]
        );

        check_metrics!(
            "\nlet a = 42;\n\n",
            "foo.rs",
            RustParser,
            loc,
            [(blank, 1, usize)]
        );

        check_metrics!(
            "fn func() { /* comment */ }\n",
            "foo.rs",
            RustParser,
            loc,
            [(blank, 0, usize)]
        );

        check_metrics!(
            "\nint a = 42;\n\n",
            "foo.c",
            CppParser,
            loc,
            [(blank, 1, usize)]
        );
    }

    #[test]
    fn test_cloc() {
        check_metrics!(
            "\"\"\"Block comment\nBlock comment\n\"\"\"\n
            # Line Comment\na = 42 # Line Comment\n",
            "foo.py",
            PythonParser,
            loc,
            [(cloc, 5, usize)]
        );

        check_metrics!(
            "/*Block comment\nBlock Comment*/\n//Line Comment\n/*Block Comment*/ let a = 42; // Line Comment\n",
            "foo.rs",
            RustParser,
            loc,
            [(cloc, 5, usize)]
        );

        check_metrics!(
            "/*Block comment\nBlock Comment*/\n//Line Comment\n/*Block Comment*/ int a = 42; // Line Comment\n",
            "foo.c",
            CppParser,
            loc,
            [(cloc, 5, usize)]
        );
    }

    #[test]
    fn test_lloc() {
        check_metrics!(
            "for x in range(0,42):\n
                if x % 2 == 0:\n
                    print(x)\n",
            "foo.py",
            PythonParser,
            loc,
            [(lloc, 3, usize)]
        );

        check_metrics!(
            "for x in 0..42 {\n
                if x % 2 == 0 {\n
                    println!(\"{}\", x);\n
                }\n
             }\n",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 3, usize)]
        );

        // LLOC returns three because there is an empty Rust statement
        check_metrics!(
            "let a = 42;\n
             if true {\n
                42\n
             } else {\n
                43\n
             };\n",
            "foo.rs",
            RustParser,
            loc,
            [(lloc, 3, usize)]
        );

        check_metrics!(
            "for (;;)\n
                break;\n",
            "foo.c",
            CppParser,
            loc,
            [(lloc, 2, usize)]
        );
    }
}
