#![allow(dead_code)]

mod parse;
mod token;

#[cfg(feature = "basic-tokens")]
pub mod basic_tokens;

#[cfg(feature = "basic-parser")]
mod basic_parser;

pub use parse::{Parse, Parser, ParseError, ParseErrorWithContext, ParseResult};
pub use token::{Location, Span, ToStatic};

#[cfg(feature = "basic-tokens")]
pub use basic_parser::*;