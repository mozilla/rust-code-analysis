// After adding new fields for min and max in the json (server.rs line 630) this error arose:
// error: recursion limit reached while expanding `json_internal!`
// This solution was proposed as help by the compiler
// for the full error details check here :https://github.com/mozilla/rust-code-analysis/pull/793#discussion_r817610530
#![recursion_limit = "256"]
pub mod web;
pub use web::*;
