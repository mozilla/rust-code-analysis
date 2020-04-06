use std::path::PathBuf;
use std::sync::Arc;
use tree_sitter::{Language, Node};

use crate::checker::Checker;
use crate::cyclomatic::Cyclomatic;
use crate::exit::Exit;
use crate::fn_args::NArgs;
use crate::getter::Getter;
use crate::halstead::Halstead;
use crate::languages::*;
use crate::loc::Loc;
use crate::mi::Mi;
use crate::nom::Nom;
use crate::preproc::PreprocResults;
use crate::ts_parser::Filter;
use crate::web::alterator::Alterator;

pub trait TSLanguage {
    type BaseLang;

    fn get_lang() -> LANG;
    fn get_language() -> Language;
    fn get_lang_name() -> &'static str;
}

pub trait TSParserTrait {
    type Checker: Alterator + Checker;
    type Getter: Getter;
    type Cyclomatic: Cyclomatic;
    type Halstead: Halstead;
    type Loc: Loc;
    type Nom: Nom;
    type Mi: Mi;
    type NArgs: NArgs;
    type Exit: Exit;

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

pub trait Search<'a> {
    fn first_occurence(&self, pred: fn(u16) -> bool) -> Option<Node<'a>>;
    fn act_on_node(&self, pred: &mut dyn FnMut(&Node<'a>));
    fn first_child(&self, pred: fn(u16) -> bool) -> Option<Node<'a>>;
    fn act_on_child(&self, action: &mut dyn FnMut(&Node<'a>));
}
