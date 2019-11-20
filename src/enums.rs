use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NodeKind {
    Unknown,
    Function,
    Class,
    Struct,
    Trait,
    Impl,
    Unit,
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            NodeKind::Unknown => "unknown",
            NodeKind::Function => "function",
            NodeKind::Class => "class",
            NodeKind::Struct => "struct",
            NodeKind::Trait => "trait",
            NodeKind::Impl => "impl",
            NodeKind::Unit => "unit",
        };
        write!(f, "{}", s)
    }
}
