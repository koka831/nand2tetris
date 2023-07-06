#![forbid(unsafe_code)]
#![feature(box_patterns)]

pub mod compiler;
pub mod diagnosis;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod symbol;
pub mod token;

use std::path::Path;

pub use crate::error::*;

pub fn is_jack_file<P: AsRef<Path>>(path: &P) -> bool {
    path.as_ref()
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("jack"))
}
