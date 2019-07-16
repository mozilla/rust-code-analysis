use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::Arc;
use tree_sitter::{Node, Parser, Tree};

use crate::c_macro;
use crate::checker::*;
use crate::languages::*;
use crate::preproc::{get_macros, PreprocResults};
use crate::traits::*;

pub struct TSParser<T: TSLanguage + Checker> {
    code: Vec<u8>,
    tree: Tree,
    phantom: PhantomData<T>,
}

type FilterFn = fn(&Node) -> bool;

pub struct Filter {
    filters: Vec<FilterFn>,
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
            LANG::C | LANG::Cpp => {
                let macros = get_macros(path, &pr.files);
                c_macro::replace(code, &macros)
            }
            _ => None,
        }
    } else {
        None
    }
}

impl<T: TSLanguage + Checker> TSParserTrait for TSParser<T> {
    type Checker = T;

    fn new(code: Vec<u8>, path: &PathBuf, pr: Option<Arc<PreprocResults>>) -> Self {
        let mut parser = Parser::new();
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
        let mut res = Vec::new();
        for f in filters.iter() {
            let f = f.as_str();
            res.push(match f {
                "call" => T::is_call,
                "comment" => T::is_comment,
                "error" => T::is_error,
                "string" => T::is_string,
                _ => |_: &Node| -> bool { true },
            });
        }
        if res.is_empty() {
            res.push(|_: &Node| -> bool { true })
        }

        Filter { filters: res }
    }
}
// other => |node: &Node| -> bool { node.kind() == other },
