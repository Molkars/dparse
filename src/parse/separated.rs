use std::fmt::Debug;

use crate::parse::{Parse, ParseError, ParseStream};

/// A list of items separated by punctuation without a trailing separator.
///
/// # Examples
/// ```
/// use parser::punct;
/// use parser::parse::{Parse, ParseError, ParseStream, Separated};
///
/// punct!(
///     pub struct Comma(",");
///     pub struct A("a");
/// );
///
/// fn parse<'a, T: Parse<'a>>(source: &'a str) -> Result<T, ParseError> {
///   T::parse(&mut ParseStream::new(source))
/// }
///
/// assert!(parse::<Separated<A, Comma>>("a, a, a").is_ok()); // Ok(_)
/// assert!(parse::<Separated<A, Comma>>("a, a, a,").is_err()); // Err()
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Separated<Item, Punct> {
    pub items: Vec<(Item, Punct)>,
    pub trailing: Item,
}

impl<'a, Item: Parse<'a>, Punct: Parse<'a>> Parse<'a> for Separated<Item, Punct> {
    #[inline(always)]
    fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        input.parse_separated()
    }
}

impl<Item, Punct> Separated<Item, Punct> {
    pub fn new(items: Vec<(Item, Punct)>, trailing: Item) -> Self {
        Self { items, trailing }
    }

    pub fn count(&self) -> usize {
        self.items.len() + 1
    }

    pub fn iter(&self) -> impl Iterator<Item=(&Item, Option<&Punct>)> {
        self.items
            .iter()
            .map(|(item, punct)| (item, Some(punct)))
            .chain(std::iter::once((&self.trailing, None)))
    }

    pub fn items(&self) -> impl Iterator<Item=&Item> {
        self.items
            .iter()
            .map(|(item, _)| item)
            .chain(std::iter::once(&self.trailing))
    }

    pub fn puncts(&self) -> impl Iterator<Item=&Punct> {
        self.items.iter().map(|(_, punct)| punct)
    }
}
