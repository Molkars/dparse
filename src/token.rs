pub use ident::*;
pub use punct::*;

use crate::parse::Span;

mod punct;
mod ident;

pub trait Token {
    fn span(&self) -> Span;

    fn content(&self) -> &str;
}