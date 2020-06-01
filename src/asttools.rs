use tree_sitter::Node;

#[allow(dead_code)]
pub fn get_parent<'a>(node: &'a Node<'a>, level: usize) -> Option<Node<'a>> {
    let mut level = level;
    let mut node = *node;
    while level != 0 {
        if let Some(parent) = node.parent() {
            node = parent;
        } else {
            return None;
        }
        level -= 1;
    }

    Some(node)
}

#[doc(hidden)]
#[macro_export]
macro_rules! has_ancestors {
    ($node:expr, $( $typs:pat )|*, $( $typ:pat ),+) => {{
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
