#![recursion_limit = "128"]
#![allow(clippy::implicit_hasher)]
//#![warn(unused_extern_crates)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[cfg_attr(test, macro_use)]
extern crate serde_json;

#[macro_use]
mod macros;
pub use crate::macros::*;

pub mod web;

pub mod find;
pub use crate::find::*;

pub mod count;
pub use crate::count::*;

pub mod c_macro;

pub mod preproc;
pub use crate::preproc::*;

mod language;
pub use crate::language::*;

mod languages;
pub use crate::languages::*;

mod tools;
pub use crate::tools::*;

mod traits;
pub use crate::traits::*;

mod ts_parser;
pub use crate::ts_parser::*;

mod checker;
pub use crate::checker::*;

mod comment_rm;
pub use crate::comment_rm::*;

mod dump;
pub use crate::dump::*;

mod language_ccomment;
pub use crate::language_ccomment::*;

mod language_c;
pub use crate::language_c::*;

mod language_cpp;
pub use crate::language_cpp::*;

mod language_csharp;
pub use crate::language_csharp::*;

mod language_css;
pub use crate::language_css::*;

mod language_go;
pub use crate::language_go::*;

mod language_html;
pub use crate::language_html::*;

mod language_java;
pub use crate::language_java::*;

mod language_javascript;
pub use crate::language_javascript::*;

mod language_python;
pub use crate::language_python::*;

mod language_rust;
pub use crate::language_rust::*;

mod language_tsx;
pub use crate::language_tsx::*;

mod language_typescript;
pub use crate::language_typescript::*;

mod language_preproc;
pub use crate::language_preproc::*;
