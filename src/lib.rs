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

pub mod node;
pub use crate::node::*;

mod rca_metrics;
pub(crate) use rca_metrics::*;

mod rca_languages;
pub(crate) use rca_languages::*;

pub mod web;

#[macro_use]
pub mod asttools;
pub use crate::asttools::*;

pub mod metrics;
pub use crate::metrics::*;

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
