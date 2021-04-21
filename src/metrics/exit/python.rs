use super::*;

impl Exit for PythonCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Python::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn python_no_exit() {
        check_metrics!(
            "a = 42",
            "foo.py",
            PythonParser,
            nexits,
            [(exit, 0, usize)],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }
    #[test]
    fn python_simple_function() {
        check_metrics!(
            "def f(a, b):
                 if a:
                     return a",
            "foo.py",
            PythonParser,
            nexits,
            [(exit, 1, usize)],
            [(exit_average, 1.0)] // 1 function
        );
    }

    #[test]
    fn python_more_functions() {
        check_metrics!(
            "def f(a, b):
                 if a:
                     return a
            def f(a, b):
                 if b:
                     return b",
            "foo.py",
            PythonParser,
            nexits,
            [(exit, 2, usize)],
            [(exit_average, 1.0)] // 2 functions
        );
    }

    #[test]
    fn python_nested_functions() {
        check_metrics!(
            "def f(a, b):
                 def foo(a):
                     if a:
                         return 1
                 bar = lambda a: lambda b: b or True or True
                 return bar(foo(a))(a)",
            "foo.py",
            PythonParser,
            nexits,
            [(exit, 2, usize)],
            [(exit_average, 0.5)] // 2 functions + 2 lambdas = 4
        );
    }
}
