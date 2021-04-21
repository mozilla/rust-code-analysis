use super::*;

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
}
