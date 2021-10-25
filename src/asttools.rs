use crate::node::Node;

#[allow(dead_code)]
pub fn get_parent<'a>(node: &'a Node<'a>, level: usize) -> Option<Node<'a>> {
    let mut level = level;
    let mut node = *node;
    while level != 0 {
        if let Some(parent) = node.object().parent() {
            node = Node::new(parent);
        } else {
            return None;
        }
        level -= 1;
    }

    Some(node)
}

macro_rules! has_ancestors {
    ($node:expr, $( $typs:pat_param )|*, $( $typ:pat_param ),+) => {{
        let mut res = false;
        loop {
            let mut node = *$node;
            $(
                if let Some(parent) = node.object().parent() {
                    match parent.kind_id().into() {
                        $typ => {
                            node = Node::new(parent);
                        },
                        _ => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            )*
            if let Some(parent) = node.object().parent() {
                match parent.kind_id().into() {
                    $( $typs )|+ => {
                        res = true;
                    },
                    _ => {
                        break;
                    }
                }
            } else {
                break;
            }
            break;
        }
        res
    }};
}

macro_rules! count_specific_ancestors {
    ($node:expr, $( $typs:pat_param )|*, $( $stops:pat_param )|*) => {{
        let mut count = 0;
        let mut node = *$node;
        while let Some(parent) = node.object().parent() {
            match parent.kind_id().into() {
                $( $typs )|* => {
                    if !Self::is_else_if(&Node::new(parent)) {
                        count += 1;
                    }
                },
                $( $stops )|* => break,
                _ => {}
            }
            node = Node::new(parent);
        }
        count
    }};
}
