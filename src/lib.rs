#![recursion_limit = "128"]
#![allow(clippy::implicit_hasher)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_cbor;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

#[macro_use]
mod macros;
pub use crate::macros::*;

pub mod node;
pub use crate::node::*;

mod metrics;
pub(crate) use metrics::*;

mod languages;
pub(crate) use languages::*;

mod output;
pub use output::*;

pub mod web;

#[macro_use]
pub mod asttools;
pub use crate::asttools::*;

pub mod spaces;
pub use crate::spaces::*;

pub mod getter;
pub use crate::getter::*;

pub mod find;
pub use crate::find::*;

pub mod function;
pub use crate::function::*;

pub mod count;
pub use crate::count::*;

pub mod c_macro;

pub mod preproc;
pub use crate::preproc::*;

mod langs;
pub use crate::langs::*;

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
