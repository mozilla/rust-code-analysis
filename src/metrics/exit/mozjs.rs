use super::*;

impl Exit for MozjsCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Mozjs::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}
