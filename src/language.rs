use enum_iterator::IntoEnumIterator;
use tree_sitter::Parser;

use crate::*;

pub fn guess(source: impl AsRef<[u8]>) -> Option<LANG> {
    for lang in LANG::into_enum_iter() {
        let mut parser = Parser::new();
        let language = get_language(&lang);
        parser.set_language(language).unwrap();
        if let Some(tree) = parser.parse(&source, None) {
            if !tree.root_node().has_error() {
                return Some(lang);
            }
        }
    }
    None
}
