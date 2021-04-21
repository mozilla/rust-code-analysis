use super::*;

impl Exit for RustCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Rust::ReturnExpression = node.object().kind_id().into() {
            stats.exit += 1;
        } else if Self::is_func(node) && node.object().child_by_field_name("return_type").is_some()
        {
            stats.exit += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn rust_no_exit() {
        check_metrics!(
            "let a = 42;",
            "foo.rs",
            RustParser,
            nexits,
            [(exit, 0, usize)],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }
}
