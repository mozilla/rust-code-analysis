use crate::node::Node;

macro_rules! has_ancestors {
    ($node:expr, $( $typs:pat_param )|*, $( $typ:pat_param ),+) => {{
        let mut res = false;
        loop {
            let mut node = *$node;
            $(
                if let Some(parent) = node.parent() {
                    match parent.kind_id().into() {
                        $typ => {
                            node = parent;
                        },
                        _ => {
                            break;
                        }
                    }
                } else {
                    break;
                }
            )*
            if let Some(parent) = node.parent() {
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
