use super::*;

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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

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
                (cloc, 5, usize),  // The number of comments is 5
                (blank, 1, usize)  // The number of blank lines is 1
            ]
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
}
