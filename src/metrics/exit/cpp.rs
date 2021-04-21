use super::*;

impl Exit for CppCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Cpp::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn c_no_exit() {
        check_metrics!(
            "int a = 42;",
            "foo.c",
            CppParser,
            nexits,
            [(exit, 0, usize)],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }
}
