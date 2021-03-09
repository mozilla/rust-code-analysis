use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::fmt;

use crate::checker::Checker;
use crate::*;

// TODO: Find a way to increment the cognitive complexity value
// for recursive code. For some kind of langauges, such as C++, it is pretty
// hard to detect, just parsing the code, if a determined function is recursive
// because the call graph of a function is solved at runtime.
// So a possible solution could be searching for a crate which implements
// a light language interpreter, computing the call graph, and then detecting
// if there are cycles. At this point, it is possible to figure out if a
// function is recursive or not.

/// The `Cognitive Complexity` metric.
#[derive(Debug, Clone)]
pub struct Stats {
    structural: usize,
    nesting: usize,
    total_space_functions: usize,
    boolean_seq: BoolSequence,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            structural: 0,
            nesting: 0,
            total_space_functions: 1,
            boolean_seq: BoolSequence::default(),
        }
    }
}

impl Serialize for Stats {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut st = serializer.serialize_struct("cognitive", 2)?;
        st.serialize_field("sum", &self.cognitive())?;
        st.serialize_field("average", &self.cognitive_average())?;
        st.end()
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "sum: {}, average: {}",
            self.cognitive(),
            self.cognitive_average()
        )
    }
}

impl Stats {
    /// Merges a second `Cognitive Complexity` metric into the first one
    pub fn merge(&mut self, other: &Stats) {
        self.structural += other.structural;
    }

    /// Returns the `Cognitive Complexity` metric value
    pub fn cognitive(&self) -> f64 {
        self.structural as f64
    }

    /// Returns the `Cognitive Complexity` metric average value
    ///
    /// This value is computed dividing the `Cognitive Complexity` value
    /// for the total number of functions/closures in a space.
    ///
    /// If there are no functions in a code, its value is `NAN`.
    pub fn cognitive_average(&self) -> f64 {
        self.cognitive() / self.total_space_functions as f64
    }

    pub(crate) fn finalize(&mut self, total_space_functions: usize) {
        self.total_space_functions = total_space_functions;
    }
}

#[doc(hidden)]
pub trait Cognitive
where
    Self: Checker,
{
    fn compute(_node: &Node, _stats: &mut Stats) {}
}

macro_rules! compute_booleans {
    ($node: ident, $stats: ident, $( $typs: pat )|*) => {
        let mut cursor = $node.object().walk();
        for child in $node.object().children(&mut cursor) {
            if let $( $typs )|* = child.kind_id().into() {
                $stats.structural = $stats
                    .boolean_seq
                    .eval_based_on_prev(child.kind_id(), $stats.structural);
            }
        }
    };
}

macro_rules! nesting_levels {
    ($node: ident, $stats: ident, [$nest_func: pat => $nest_func_stop: pat],
     [$lambdas: pat => $( $lambdas_stop: pat )|*],
     [$( $nest_level: pat )|* => $( $nest_level_stop: pat )|*]) => {
        // Find the depth of a function (the most external function is
        // not considered)
        $stats.nesting = count_specific_ancestors!($node, $nest_func, $nest_func_stop).max(1) - 1;

        // Find the depth of a lambda
        let lambda_depth = count_specific_ancestors!($node, $lambdas, $( $lambdas_stop )|*);

        // Find the nesting operator level
        $stats.nesting += lambda_depth
            + count_specific_ancestors!(
                $node,
                $( $nest_level )|*,
                $( $nest_level_stop)|*
            );

        // Reset the boolean sequence
        $stats.boolean_seq.reset();

        increment($stats);
    };
    ($node: ident, $stats: ident,
     [$lambdas: pat => $( $lambdas_stop: pat )|*],
     [$( $nest_level: pat )|* => $( $nest_level_stop: pat )|*]) => {
        // Find the depth of a lambda
        let lambda_depth = count_specific_ancestors!($node, $lambdas, $( $lambdas_stop )|*);

        // Find the nesting operator level
        $stats.nesting = lambda_depth
            + count_specific_ancestors!(
                $node,
                $( $nest_level )|*,
                $( $nest_level_stop)|*
            );

        // Reset the boolean sequence
        $stats.boolean_seq.reset();

        increment($stats);
    };
}

#[derive(Debug, Default, Clone)]
struct BoolSequence {
    boolean_op: Option<u16>,
    first_boolean: bool,
}

impl BoolSequence {
    fn reset(&mut self) {
        self.boolean_op = None;
    }

    fn not_operator(&mut self, not_id: u16) {
        self.boolean_op = Some(not_id);
    }

    fn eval_based_on_prev(&mut self, bool_id: u16, structural: usize) -> usize {
        if let Some(prev) = self.boolean_op {
            if prev != bool_id {
                // The boolean operator is different from the previous one, so
                // the counter is incremented.
                structural + 1
            } else {
                // The boolean operator is equal to the previous one, so
                // the counter is not incremented.
                structural
            }
        } else {
            // Save the first boolean operator in a sequence of
            // logical operators and increment the counter.
            self.boolean_op = Some(bool_id);
            structural + 1
        }
    }
}

#[inline(always)]
fn increment(stats: &mut Stats) {
    stats.structural += stats.nesting + 1;
}

#[inline(always)]
fn increment_by_one(stats: &mut Stats) {
    stats.structural += 1;
}

impl Cognitive for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Python::*;

        match node.object().kind_id().into() {
            IfStatement | ForStatement | WhileStatement | ConditionalExpression => {
                nesting_levels!(
                    node, stats,
                    [FunctionDefinition => Module],
                    [Lambda => FunctionDefinition | Module],
                    [IfStatement | ForStatement | WhileStatement | ExceptClause => FunctionDefinition]
                );
            }
            ElifClause => {
                // No nesting increment for them because their cost has already
                // been paid by the if construct
                increment_by_one(stats);
                // Reset the boolean sequence
                stats.boolean_seq.reset();
            }
            ElseClause | FinallyClause => {
                // No nesting increment for them because their cost has already
                // been paid by the if construct
                increment_by_one(stats);
            }
            ExceptClause => {
                increment(stats);
            }
            ExpressionList | ExpressionStatement | Tuple => {
                stats.boolean_seq.reset();
            }
            NotOperator => {
                stats.boolean_seq.not_operator(node.object().kind_id());
            }
            BooleanOperator => {
                if count_specific_ancestors!(node, BooleanOperator, Lambda) == 0 {
                    stats.structural += count_specific_ancestors!(
                        node,
                        Lambda,
                        ExpressionList | IfStatement | ForStatement | WhileStatement
                    );
                }
                compute_booleans!(node, stats, And | Or);
            }
            _ => {}
        }
    }
}

impl Cognitive for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Rust::*;

        //TODO: Implement macros

        match node.object().kind_id().into() {
            IfExpression => {
                // Check if a node is not an else-if
                if !Self::is_else_if(&node) {
                    nesting_levels!(
                        node, stats,
                        [FunctionItem => SourceFile],
                        [ClosureExpression => SourceFile],
                        [IfExpression | ForExpression | WhileExpression | MatchExpression => FunctionItem]
                    );
                }
            }
            ForExpression | WhileExpression | MatchExpression => {
                nesting_levels!(
                    node, stats,
                    [FunctionItem => SourceFile],
                    [ClosureExpression => SourceFile],
                    [IfExpression | ForExpression | WhileExpression | MatchExpression => FunctionItem]
                );
            }
            Else /*else-if also */ => {
                increment_by_one(stats);
            }
            BreakExpression | ContinueExpression => {
                if let Some(label_child) = node.object().child(1) {
                    if let LoopLabel = label_child.kind_id().into() {
                        increment_by_one(stats);
                    }
                }
            }
            UnaryExpression => {
                stats.boolean_seq.not_operator(node.object().kind_id());
            }
            BinaryExpression => {
                compute_booleans!(node, stats, AMPAMP | PIPEPIPE);
            }
            _ => {}
        }
    }
}

impl Cognitive for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        use Cpp::*;

        //TODO: Implement macros

        match node.object().kind_id().into() {
            IfStatement => {
                if !Self::is_else_if(&node) {
                    nesting_levels!(
                        node, stats,
                        [LambdaExpression => TranslationUnit],
                        [IfStatement
                            | ForStatement
                            | WhileStatement
                            | DoStatement
                            | SwitchStatement
                            | CatchClause => FunctionDefinition]
                    );
                }
            }
            ForStatement | WhileStatement | DoStatement | SwitchStatement | CatchClause => {
                nesting_levels!(
                    node, stats,
                    [LambdaExpression => TranslationUnit],
                    [IfStatement
                        | ForStatement
                        | WhileStatement
                        | DoStatement
                        | SwitchStatement
                        | CatchClause => FunctionDefinition]
                );
            }
            GotoStatement | Else /* else-if also */ => {
                increment_by_one(stats);
            }
            UnaryExpression2 => {
                stats.boolean_seq.not_operator(node.object().kind_id());
            }
            BinaryExpression2 => {
                compute_booleans!(node, stats, AMPAMP | PIPEPIPE);
            }
            _ => {}
        }
    }
}

macro_rules! js_cognitive {
    ($lang:ident) => {
        fn compute(node: &Node, stats: &mut Stats) {
            use $lang::*;

            match node.object().kind_id().into() {
                IfStatement => {
                    if !Self::is_else_if(&node) {
                        nesting_levels!(
                            node, stats,
                            [FunctionDeclaration => Program],
                            [ArrowFunction => FunctionDeclaration | Program],
                            [IfStatement
                                | ForStatement
                                | ForInStatement
                                | WhileStatement
                                | DoStatement
                                | SwitchStatement
                                | CatchClause
                                | TernaryExpression => FunctionDeclaration]
                        );
                    }
                }
                ForStatement | ForInStatement | WhileStatement | DoStatement | SwitchStatement | CatchClause | TernaryExpression => {
                    nesting_levels!(
                        node, stats,
                        [FunctionDeclaration => Program],
                        [ArrowFunction => FunctionDeclaration | Program],
                        [IfStatement
                            | ForStatement
                            | ForInStatement
                            | WhileStatement
                            | DoStatement
                            | SwitchStatement
                            | CatchClause
                            | TernaryExpression => FunctionDeclaration]
                    );
                }
                Else /* else-if also */ => {
                    increment_by_one(stats);
                }
                ExpressionStatement => {
                    // Reset the boolean sequence
                    stats.boolean_seq.reset();
                }
                UnaryExpression => {
                    stats.boolean_seq.not_operator(node.object().kind_id());
                }
                BinaryExpression => {
                    compute_booleans!(node, stats, AMPAMP | PIPEPIPE);
                }
                _ => {}
            }
        }
    };
}

impl Cognitive for MozjsCode {
    js_cognitive!(Mozjs);
}

impl Cognitive for JavascriptCode {
    js_cognitive!(Javascript);
}

impl Cognitive for TypescriptCode {
    js_cognitive!(Typescript);
}

impl Cognitive for TsxCode {
    js_cognitive!(Tsx);
}

impl Cognitive for PreprocCode {}
impl Cognitive for CcommentCode {}
impl Cognitive for JavaCode {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn python_no_cognitive() {
        check_metrics!(
            "a = 42",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 0, usize)],
            [(cognitive_average, f64::NAN)]
        );
    }

    #[test]
    fn rust_no_cognitive() {
        check_metrics!(
            "let a = 42;",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 0, usize)],
            [(cognitive_average, f64::NAN)]
        );
    }

    #[test]
    fn c_no_cognitive() {
        check_metrics!(
            "int a = 42;",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 0, usize)],
            [(cognitive_average, f64::NAN)]
        );
    }

    #[test]
    fn mozjs_no_cognitive() {
        check_metrics!(
            "var a = 42;",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 0, usize)],
            [(cognitive_average, f64::NAN)]
        );
    }

    #[test]
    fn python_simple_function() {
        check_metrics!(
            "def f(a, b):
                if a and b:  # +2 (+1 and)
                   return 1
                if c and d: # +2 (+1 and)
                   return 1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn python_expression_statement() {
        // Boolean expressions containing `And` and `Or` operators were not
        // considered in assignments
        check_metrics!(
            "def f(a, b):
                c = True and True",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 1, usize)],
            [(cognitive_average, 1.0)]
        );
    }

    #[test]
    fn python_tuple() {
        // Boolean expressions containing `And` and `Or` operators were not
        // considered inside tuples
        check_metrics!(
            "def f(a, b):
                return \"%s%s\" % (a and \"Get\" or \"Set\", b)",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );
    }

    #[test]
    fn python_elif_function() {
        // Boolean expressions containing `And` and `Or` operators were not
        // considered in `elif` statements
        check_metrics!(
            "def f(a, b):
                if a and b:  # +2 (+1 and)
                   return 1
                elif c and d: # +2 (+1 and)
                   return 1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn python_more_elifs_function() {
        // Boolean expressions containing `And` and `Or` operators were not
        // considered when there were more `elif` statements
        check_metrics!(
            "def f(a, b):
                if a and b:  # +2 (+1 and)
                   return 1
                elif c and d: # +2 (+1 and)
                   return 1
                elif e and f: # +2 (+1 and)
                   return 1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 6, usize)],
            [(cognitive_average, 6.0)]
        );
    }

    #[test]
    fn rust_simple_function() {
        check_metrics!(
            "fn f() {
                 if a && b { // +2 (+1 &&)
                     println!(\"test\");
                 }
                 if c && d { // +2 (+1 &&)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn c_simple_function() {
        check_metrics!(
            "void f() {
                 if (a && b) { // +2 (+1 &&)
                     printf(\"test\");
                 }
                 if (c && d) { // +2 (+1 &&)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn mozjs_simple_function() {
        check_metrics!(
            "function f() {
                 if (a && b) { // +2 (+1 &&)
                     window.print(\"test\");
                 }
                 if (c && d) { // +2 (+1 &&)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn python_sequence_same_booleans() {
        check_metrics!(
            "def f(a, b):
                if a and b and True:  # +2 (+1 sequence of and)
                   return 1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );
    }

    #[test]
    fn rust_sequence_same_booleans() {
        check_metrics!(
            "fn f() {
                 if a && b && true { // +2 (+1 sequence of &&)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );

        check_metrics!(
            "fn f() {
                 if a || b || c || d { // +2 (+1 sequence of ||)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );
    }

    #[test]
    fn c_sequence_same_booleans() {
        check_metrics!(
            "void f() {
                 if (a && b && 1 == 1) { // +2 (+1 sequence of &&)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );

        check_metrics!(
            "void f() {
                 if (a || b || c || d) { // +2 (+1 sequence of ||)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );
    }

    #[test]
    fn mozjs_sequence_same_booleans() {
        check_metrics!(
            "function f() {
                 if (a && b && 1 == 1) { // +2 (+1 sequence of &&)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );

        check_metrics!(
            "function f() {
                 if (a || b || c || d) { // +2 (+1 sequence of ||)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );
    }

    #[test]
    fn rust_not_booleans() {
        check_metrics!(
            "fn f() {
                 if !a && !b { // +2 (+1 &&)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 2, usize)],
            [(cognitive_average, 2.0)]
        );

        check_metrics!(
            "fn f() {
                 if a && !(b && c) { // +3 (+1 &&, +1 &&)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );

        check_metrics!(
            "fn f() {
                 if !(a || b) && !(c || d) { // +4 (+1 ||, +1 &&, +1 ||)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn c_not_booleans() {
        check_metrics!(
            "void f() {
                 if (a && !(b && c)) { // +3 (+1 &&, +1 &&)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );

        check_metrics!(
            "void f() {
                 if (!(a || b) && !(c || d)) { // +4 (+1 ||, +1 &&, +1 ||)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn mozjs_not_booleans() {
        check_metrics!(
            "function f() {
                 if (a && !(b && c)) { // +3 (+1 &&, +1 &&)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );

        check_metrics!(
            "function f() {
                 if (!(a || b) && !(c || d)) { // +4 (+1 ||, +1 &&, +1 ||)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn python_sequence_different_booleans() {
        check_metrics!(
            "def f(a, b):
                if a and b or True:  # +3 (+1 and, +1 or)
                   return 1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );
    }

    #[test]
    fn rust_sequence_different_booleans() {
        check_metrics!(
            "fn f() {
                 if a && b || true { // +3 (+1 &&, +1 ||)
                     println!(\"test\");
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );
    }

    #[test]
    fn c_sequence_different_booleans() {
        check_metrics!(
            "void f() {
                 if (a && b || 1 == 1) { // +3 (+1 &&, +1 ||)
                     printf(\"test\");
                 }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );
    }

    #[test]
    fn mozjs_sequence_different_booleans() {
        check_metrics!(
            "function f() {
                 if (a && b || 1 == 1) { // +3 (+1 &&, +1 ||)
                     window.print(\"test\");
                 }
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );
    }

    #[test]
    fn python_formatted_sequence_different_booleans() {
        check_metrics!(
            "def f(a, b):
                if (  # +1
                    a and b and  # +1
                    (c or d)  # +1
                ):
                   return 1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );
    }

    #[test]
    fn python_1_level_nesting() {
        check_metrics!(
            "def f(a, b):
                if a:  # +1
                    for i in range(b):  # +2
                        return 1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );
    }

    #[test]
    fn rust_1_level_nesting() {
        check_metrics!(
            "fn f() {
                 if true { // +1
                     if true { // +2 (nesting = 1)
                         println!(\"test\");
                     } else if 1 == 1 { // +1
                         if true { // +3 (nesting = 2)
                             println!(\"test\");
                         }
                     } else { // +1
                         if true { // +3 (nesting = 2)
                             println!(\"test\");
                         }
                     }
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 11, usize)],
            [(cognitive_average, 11.0)]
        );

        check_metrics!(
            "fn f() {
                 if true { // +1
                     match true { // +2 (nesting = 1)
                         true => println!(\"test\"),
                         false => println!(\"test\"),
                     }
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );
    }

    #[test]
    fn c_1_level_nesting() {
        check_metrics!(
            "void f() {
                 if (1 == 1) { // +1
                     if (1 == 1) { // +2 (nesting = 1)
                         printf(\"test\");
                     } else if (1 == 1) { // +1
                         if (1 == 1) { // +3 (nesting = 2)
                             printf(\"test\");
                         }
                     } else { // +1
                         if (1 == 1) { // +3 (nesting = 2)
                             printf(\"test\");
                         }
                     }
                 }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 11, usize)],
            [(cognitive_average, 11.0)]
        );
    }

    #[test]
    fn mozjs_1_level_nesting() {
        check_metrics!(
            "function f() {
                 if (1 == 1) { // +1
                     if (1 == 1) { // +2 (nesting = 1)
                         window.print(\"test\");
                     } else if (1 == 1) { // +1
                         if (1 == 1) { // +3 (nesting = 2)
                             window.print(\"test\");
                         }
                     } else { // +1
                         if (1 == 1) { // +3 (nesting = 2)
                             window.print(\"test\");
                         }
                     }
                 }
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 11, usize)],
            [(cognitive_average, 11.0)]
        );
    }

    #[test]
    fn python_2_level_nesting() {
        check_metrics!(
            "def f(a, b):
                if a:  # +1
                    for i in range(b):  # +2
                        if b:  # +3
                            return 1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 6, usize)],
            [(cognitive_average, 6.0)]
        );
    }

    #[test]
    fn rust_2_level_nesting() {
        check_metrics!(
            "fn f() {
                 if true { // +1
                     for i in 0..4 { // +2 (nesting = 1)
                         match true { // +3 (nesting = 2)
                             true => println!(\"test\"),
                             false => println!(\"test\"),
                         }
                     }
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 6, usize)],
            [(cognitive_average, 6.0)]
        );
    }

    #[test]
    fn python_try_construct() {
        check_metrics!(
            "def f(a, b):
                try:
                    for foo in bar:  # +1
                        return a
                except Exception:  # +1
                    if a < 0:  # +2
                        return a",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn mozjs_try_construct() {
        check_metrics!(
            "function asyncOnChannelRedirect(oldChannel, newChannel, flags, callback) {
                 for (const collector of this.collectors) {
                     try {
                         collector._onChannelRedirect(oldChannel, newChannel, flags);
                     } catch (ex) {
                         console.error(
                             \"StackTraceCollector.onChannelRedirect threw an exception\",
                              ex
                         );
                     }
                 }
                 callback.onRedirectVerifyCallback(Cr.NS_OK);
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 3, usize)],
            [(cognitive_average, 3.0)]
        );
    }

    #[test]
    fn rust_break_continue() {
        // Only labeled break and continue statements are considered
        check_metrics!(
            "fn f() {
                 'tens: for ten in 0..3 { // +1
                     '_units: for unit in 0..=9 { // +2 (nesting = 1)
                         if unit % 2 == 0 { // +3 (nesting = 2)
                             continue;
                         } else if unit == 5 { // +1
                             continue 'tens; // +1
                         } else if unit == 6 { // +1
                             break;
                         } else { // +1
                             break 'tens; // +1
                         }
                     }
                 }
             }",
            "foo.rs",
            RustParser,
            cognitive,
            [(cognitive, 11, usize)],
            [(cognitive_average, 11.0)]
        );
    }

    #[test]
    fn c_goto() {
        check_metrics!(
            "void f() {
             OUT: for (int i = 1; i <= max; ++i) { // +1
                      for (int j = 2; j < i; ++j) { // +2 (nesting = 1)
                          if (i % j == 0) { // +3 (nesting = 2)
                              goto OUT; // +1
                          }
                      }
                  }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 7, usize)],
            [(cognitive_average, 7.0)]
        );
    }

    #[test]
    fn c_switch() {
        check_metrics!(
            "void f() {
                 switch (1) { // +1
                     case 1:
                         printf(\"one\");
                         break;
                     case 2:
                         printf(\"two\");
                         break;
                     case 3:
                         printf(\"three\");
                         break;
                     default:
                         printf(\"all\");
                         break;
                 }
             }",
            "foo.c",
            CppParser,
            cognitive,
            [(cognitive, 1, usize)],
            [(cognitive_average, 1.0)]
        );
    }

    #[test]
    fn mozjs_switch() {
        check_metrics!(
            "function f() {
                 switch (1) { // +1
                     case 1:
                         window.print(\"one\");
                         break;
                     case 2:
                         window.print(\"two\");
                         break;
                     case 3:
                         window.print(\"three\");
                         break;
                     default:
                         window.print(\"all\");
                         break;
                 }
             }",
            "foo.js",
            MozjsParser,
            cognitive,
            [(cognitive, 1, usize)],
            [(cognitive_average, 1.0)]
        );
    }

    #[test]
    fn python_ternary_operator() {
        check_metrics!(
            "def f(a, b):
                 if a % 2:  # +1
                     return 'c' if a else 'd'  # +2
                 return 'a' if a else 'b'  # +1",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 4, usize)],
            [(cognitive_average, 4.0)]
        );
    }

    #[test]
    fn python_nested_functions_lambdas() {
        check_metrics!(
            "def f(a, b):
                 def foo(a):
                     if a:  # +2 (+1 nesting)
                         return 1
                 # +3 (+1 for boolean sequence +2 for lambda nesting)
                 bar = lambda a: lambda b: b or True or True
                 return bar(foo(a))(a)",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 5, usize)],
            [(cognitive_average, 1.25)] // 2 functions + 2 lamdas = 4
        );
    }

    #[test]
    fn python_real_function() {
        check_metrics!(
            "def process_raw_constant(constant, min_word_length):
                 processed_words = []
                 raw_camelcase_words = []
                 for raw_word in re.findall(r'[a-z]+', constant):  # +1
                     word = raw_word.strip()
                         if (  # +2 (+1 if and +1 nesting)
                             len(word) >= min_word_length
                             and not (word.startswith('-') or word.endswith('-')) # +2 operators
                         ):
                             if is_camel_case_word(word):  # +3 (+1 if and +2 nesting)
                                 raw_camelcase_words.append(word)
                             else: # +1 else
                                 processed_words.append(word.lower())
                 return processed_words, raw_camelcase_words",
            "foo.py",
            PythonParser,
            cognitive,
            [(cognitive, 9, usize)],
            [(cognitive_average, 9.0)]
        );
    }
}
