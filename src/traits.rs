use std::path::PathBuf;
use std::sync::Arc;
use tree_sitter::Language;

use crate::alterator::Alterator;
use crate::checker::Checker;
use crate::cyclomatic::Cyclomatic;
use crate::exit::Exit;
use crate::fn_args::NArgs;
use crate::getter::Getter;
use crate::halstead::Halstead;
use crate::langs::*;
use crate::loc::Loc;
use crate::mi::Mi;
use crate::node::Node;
use crate::nom::Nom;
use crate::parser::Filter;
use crate::preproc::PreprocResults;

/// A trait for callback functions.
///
/// Allows to call a private library function, getting as result
/// its output value.
pub trait Callback {
    /// The output type returned by the callee
    type Res;
    /// The input type used by the caller to pass the arguments to the callee
    type Cfg;

    /// Calls a function inside the library and returns its value
    fn call<T: ParserTrait>(cfg: Self::Cfg, parser: &T) -> Self::Res;
}

#[doc(hidden)]
pub trait CodeMetricsT: Cyclomatic + Exit + Halstead + NArgs + Loc + Nom + Mi {}

#[doc(hidden)]
pub trait TSLanguage {
    type BaseLang;

    fn get_lang() -> LANG;
    fn get_language() -> Language;
    fn get_lang_name() -> &'static str;
}

#[doc(hidden)]
pub trait ParserTrait {
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

pub(crate) trait Search<'a> {
    fn first_occurence(&self, pred: fn(u16) -> bool) -> Option<Node<'a>>;
    fn act_on_node(&self, pred: &mut dyn FnMut(&Node<'a>));
    fn first_child(&self, pred: fn(u16) -> bool) -> Option<Node<'a>>;
    fn act_on_child(&self, action: &mut dyn FnMut(&Node<'a>));
}
