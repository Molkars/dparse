// #![feature(result_option_inspect)]
#![allow(dead_code)]

use std::any::type_name;
use std::process::abort;

use crate::parse::{Parse, ParseError, ParseStream};

pub mod parse;
pub mod token;
#[cfg(feature = "basic")]
pub mod basic;

#[inline]
pub fn parse<'a, T: Parse<'a>>(input: &'a str) -> Result<T, ParseError> {
    let mut input = ParseStream::new(input);
    T::parse(&mut input)
}

pub fn unwrap<T>(result: Result<T, ParseError>) -> T {
    match result {
        Ok(t) => t,
        Err(e) => {
            eprintln!("parse error[mismatch={}]", e.mismatch);
            for message in &e.messages {
                eprintln!("  {}", message);
            }
            if cfg!(any(debug_assertions, feature = "track_caller")) {
                eprintln!(" TRACE:");
                eprintln!("{:?}", e.backtrace());
            }
            abort();
        }
    }
}

#[inline]
#[track_caller]
pub fn required<'a, T: Parse<'a>>(
    input: &ParseStream<'a>,
    mut result: Result<T, ParseError>,
) -> Result<T, ParseError> {
    if let Err(e) = &mut result {
        e.mismatch = false;
        e.messages.push_front(match input.peek_char() {
            Some(c) => format!(
                "expected {}, instead found unexpected character '{}' at {}",
                type_name::<T>(),
                c,
                input.span(input.spanner()).display(input)
            ),
            None => {
                format!("expected {}, instead found end of input", type_name::<T>())
            }
        });
    }
    result
}
