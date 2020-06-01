#![recursion_limit = "128"]
#![allow(clippy::implicit_hasher)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
extern crate serde_cbor;
extern crate serde_json;
extern crate serde_yaml;
extern crate toml;

pub(crate) mod c_macro;

#[macro_use]
mod asttools;

#[macro_use]
mod macros;
pub use crate::macros::*;

pub mod node;
pub use crate::node::*;

mod metrics;
pub(crate) use metrics::*;

mod languages;
pub(crate) use languages::*;

mod checker;
pub(crate) use checker::*;

mod output;
pub use output::*;

pub mod spaces;
pub use crate::spaces::*;

pub mod getter;
pub use crate::getter::*;

pub mod find;
pub use crate::find::*;

pub mod function;
pub use crate::function::*;

mod alterator;
pub(crate) use crate::alterator::*;

mod ast;
pub use crate::ast::*;

pub mod count;
pub use crate::count::*;

pub mod preproc;
pub use crate::preproc::*;

mod langs;
pub use crate::langs::*;

mod tools;
pub use crate::tools::*;

mod traits;
pub use crate::traits::*;

mod ts_parser;
pub(crate) use crate::ts_parser::*;

mod comment_rm;
pub use crate::comment_rm::*;
