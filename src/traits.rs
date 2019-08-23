use std::path::PathBuf;
use std::sync::Arc;
use tree_sitter::{Language, Node};

use crate::checker::Checker;
use crate::languages::*;
use crate::preproc::PreprocResults;
use crate::ts_parser::Filter;
use crate::web::alterator::Alterator;

pub trait TSLanguage {
    fn get_lang() -> LANG;
    fn get_language() -> Language;
    fn get_lang_name() -> &'static str;
}

pub trait TSParserTrait {
    type Checker: Alterator + Checker;

    fn new(code: Vec<u8>, path: &PathBuf, pr: Option<Arc<PreprocResults>>) -> Self;
    fn get_language(&self) -> LANG;
    fn get_root(&self) -> Node;
    fn get_code(&self) -> &[u8];
    fn get_filters(&self, filters: &[String]) -> Filter;
}

pub trait Callback {
    type Res;
    type Cfg;

    fn call<T: TSParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res;
}
