use super::*;

impl Exit for TypescriptCode {
    fn compute(node: &Node, stats: &mut Stats) {
        if let Typescript::ReturnStatement = node.object().kind_id().into() {
            stats.exit += 1;
        }
    }
}
