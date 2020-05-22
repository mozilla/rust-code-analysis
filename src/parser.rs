use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::Arc;
use tree_sitter::{Node, Parser as TSParser, Tree};

use crate::alterator::Alterator;
use crate::c_macro;
use crate::checker::*;
use crate::getter::Getter;
use crate::langs::*;
use crate::preproc::{get_macros, PreprocResults};
use crate::traits::*;

pub struct Parser<T: TSLanguage + Checker + Getter + Alterator + CodeMetricsT> {
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
fn get_fake_code<T: TSLanguage>(
    code: &[u8],
    path: &PathBuf,
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

impl<T: 'static + TSLanguage + Checker + Getter + Alterator + CodeMetricsT> ParserTrait
    for Parser<T>
{
    type Checker = T;
    type Getter = T;
    type Cyclomatic = T;
    type Halstead = T;
    type Loc = T;
    type Nom = T;
    type Mi = T;
    type NArgs = T;
    type Exit = T;

    fn new(code: Vec<u8>, path: &PathBuf, pr: Option<Arc<PreprocResults>>) -> Self {
        let mut parser = TSParser::new();
        parser.set_language(T::get_language()).unwrap();
        let fake_code = get_fake_code::<T>(&code, path, pr);
        /*let tree = if let Some(fake) = fake_code {
            parser.parse(&fake, None).unwrap()
        } else {
            parser.parse(&code, None).unwrap()
        };*/
        let code = if let Some(fake) = fake_code {
            //eprintln!("{}", String::from_utf8(fake.clone()).unwrap());
            fake
        } else {
            code
        };
        let tree = parser.parse(&code, None).unwrap();

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
        self.tree.root_node()
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
                "call" => res.push(Box::new(T::is_call)),
                "comment" => res.push(Box::new(T::is_comment)),
                "error" => res.push(Box::new(T::is_error)),
                "string" => res.push(Box::new(T::is_string)),
                "function" => res.push(Box::new(T::is_func)),
                _ => {
                    if let Ok(n) = f.parse::<u16>() {
                        res.push(Box::new(move |node: &Node| -> bool { node.kind_id() == n }));
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
// other => |node: &Node| -> bool { node.kind() == other },
