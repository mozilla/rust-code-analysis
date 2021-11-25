//! rust-code-analysis is a library to analyze and extract information
//! from source codes written in many different programming languages.
//!
//! You can find the source code of this software on
//! <a href="https://github.com/mozilla/rust-code-analysis/" target="_blank">GitHub</a>,
//! while issues and feature requests can be posted on the respective
//! <a href="https://github.com/mozilla/rust-code-analysis/issues/" target="_blank">GitHub Issue Tracker</a>.
//!
//! ## Supported Languages
//!
//! - C++
//! - C#
//! - CSS
//! - Go
//! - HTML
//! - Java
//! - JavaScript
//! - The JavaScript used in Firefox internal
//! - Python
//! - Rust
//! - Typescript
//!
//! ## Supported Metrics
//!
//! - CC: it calculates the code complexity examining the
//!   control flow of a program.
//! - SLOC: it counts the number of lines in a source file.
//! - PLOC: it counts the number of physical lines (instructions)
//!   contained in a source file.
//! - LLOC: it counts the number of logical lines (statements)
//!   contained in a source file.
//! - CLOC: it counts the number of comments in a source file.
//! - BLANK: it counts the number of blank lines in a source file.
//! - HALSTEAD: it is a suite that provides a series of information,
//!   such as the effort required to maintain the analyzed code,
//!   the size in bits to store the program, the difficulty to understand
//!   the code, an estimate of the number of bugs present in the codebase,
//!   and an estimate of the time needed to implement the software.
//! - MI: it is a suite that allows to evaluate the maintainability
//!   of a software.
//! - NOM: it counts the number of functions and closures
//!   in a file/trait/class.
//! - NEXITS: it counts the number of possible exit points
//!   from a method/function.
//! - NARGS: it counts the number of arguments of a function/method.

#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate lazy_static;
extern crate num;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate serde;

#[macro_use]
mod asttools;
mod c_langs_macros;
mod c_macro;
#[macro_use]
mod macros;
mod getter;

mod alterator;
pub(crate) use alterator::*;

mod node;
pub use crate::node::*;

mod metrics;
pub use metrics::*;

mod languages;
pub(crate) use languages::*;

mod checker;
pub(crate) use checker::*;

mod output;
pub use output::*;

mod spaces;
pub use crate::spaces::*;

mod ops;
pub use crate::ops::*;

mod find;
pub use crate::find::*;

mod function;
pub use crate::function::*;

mod ast;
pub use crate::ast::*;

mod count;
pub use crate::count::*;

mod preproc;
pub use crate::preproc::*;

mod langs;
pub use crate::langs::*;

mod tools;
pub use crate::tools::*;

mod concurrent_files;
pub use crate::concurrent_files::*;

mod traits;
pub use crate::traits::*;

mod parser;
pub use crate::parser::*;

mod comment_rm;
pub use crate::comment_rm::*;
