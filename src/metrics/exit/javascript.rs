use super::*;

impl Exit for JavascriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Javascript::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn javascript_no_exit() {
        check_metrics!(
            "var a = 42;",
            "foo.js",
            JavascriptParser,
            nexits,
            [(exit, 0, usize)],
            [(exit_average, f64::NAN)] // 0 functions
        );
    }
}
