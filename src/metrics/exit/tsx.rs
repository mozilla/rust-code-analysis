use super::*;

impl Exit for TsxCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Tsx::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}
