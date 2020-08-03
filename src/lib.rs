#![crate_type = "staticlib"]
#[macro_use]extern crate serde_derive;

pub mod api;
pub use api::*;