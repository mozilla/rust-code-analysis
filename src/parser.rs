use std::marker::PhantomData;
use std::path::Path;
use std::sync::Arc;

use crate::abc::Abc;
use crate::checker::Checker;
use crate::cognitive::Cognitive;
use crate::cyclomatic::Cyclomatic;
use crate::exit::Exit;
use crate::halstead::Halstead;
use crate::loc::Loc;
use crate::mi::Mi;
use crate::nargs::NArgs;
use crate::nom::Nom;
use crate::npa::Npa;
use crate::npm::Npm;
use crate::wmc::Wmc;

use crate::alterator::Alterator;
use crate::getter::Getter;

use crate::c_macro;
use crate::langs::*;
use crate::node::{Node, Tree};
use crate::preproc::{get_macros, PreprocResults};
use crate::traits::*;

#[derive(Debug)]
pub struct Parser<
    T: LanguageInfo
        + Alterator
        + Checker
        + Getter
        + Abc
        + Cognitive
        + Cyclomatic
        + Exit
        + Halstead
        + Loc
        + Mi
        + NArgs
        + Nom
        + Npa
        + Npm
        + Wmc,
> {
    code: Vec<u8>,
    tree: Tree,
    phantom: PhantomData<T>,
}

type FilterFn = dyn Fn(&Node) -> bool;

pub struct Filter {
    filters: Vec<Box<FilterFn>>,
}

impl Filter {
    pub fn any(&self, node: &Node) -> bool {
        for f in self.filters.iter() {
            if f(node) {
                return true;
            }
        }
        false
    }

    pub fn all(&self, node: &Node) -> bool {
        for f in self.filters.iter() {
            if !f(node) {
                return false;
            }
        }
        true
    }
}

#[inline(always)]
fn get_fake_code<T: LanguageInfo>(
    code: &[u8],
    path: &Path,
    pr: Option<Arc<PreprocResults>>,
) -> Option<Vec<u8>> {
    if let Some(pr) = pr {
        match T::get_lang() {
            LANG::Cpp => {
                let macros = get_macros(path, &pr.files);
                c_macro::replace(code, &macros)
            }
            _ => None,
        }
    } else {
        None
    }
}

impl<
        T: 'static
            + LanguageInfo
            + Alterator
            + Checker
            + Getter
            + Abc
            + Cognitive
            + Cyclomatic
            + Exit
            + Halstead
            + Loc
            + Mi
            + NArgs
            + Nom
            + Npa
            + Npm
            + Wmc,
    > ParserTrait for Parser<T>
{
    type Checker = T;
    type Getter = T;
    type Cognitive = T;
    type Cyclomatic = T;
    type Halstead = T;
    type Loc = T;
    type Nom = T;
    type Mi = T;
    type NArgs = T;
    type Exit = T;
    type Wmc = T;
    type Abc = T;
    type Npm = T;
    type Npa = T;

    fn new(code: Vec<u8>, path: &Path, pr: Option<Arc<PreprocResults>>) -> Self {
        let fake_code = get_fake_code::<T>(&code, path, pr);
        let code = if let Some(fake) = fake_code {
            fake
        } else {
            code
        };

        let tree = Tree::new::<T>(&code);

        Self {
            code,
            tree,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    fn get_language(&self) -> LANG {
        T::get_lang()
    }

    #[inline(always)]
    fn get_root(&self) -> Node {
        self.tree.get_root()
    }

    #[inline(always)]
    fn get_code(&self) -> &[u8] {
        &self.code
    }

    fn get_filters(&self, filters: &[String]) -> Filter {
        let mut res: Vec<Box<FilterFn>> = Vec::new();
        for f in filters.iter() {
            let f = f.as_str();
            match f {
                "all" => res.push(Box::new(|_: &Node| -> bool { true })),
                "call" => res.push(Box::new(T::is_call)),
                "comment" => res.push(Box::new(T::is_comment)),
                "error" => res.push(Box::new(T::is_error)),
                "string" => res.push(Box::new(T::is_string)),
                "function" => res.push(Box::new(T::is_func)),
                _ => {
                    if let Ok(n) = f.parse::<u16>() {
                        res.push(Box::new(move |node: &Node| -> bool { node.kind_id() == n }));
                    } else {
                        let f = f.to_owned();
                        res.push(Box::new(move |node: &Node| -> bool {
                            node.kind().contains(&f)
                        }));
                    }
                }
            }
        }
        if res.is_empty() {
            res.push(Box::new(|_: &Node| -> bool { true }))
        }

        Filter { filters: res }
    }
}
