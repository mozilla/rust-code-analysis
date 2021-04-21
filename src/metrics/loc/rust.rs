use super::*;

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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

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
                (cloc, 4, usize),  // The number of comments is 4
                (blank, 1, usize)  // The number of blank lines is 1
            ]
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
}
