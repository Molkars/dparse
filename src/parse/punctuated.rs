use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use crate::parse::{Parse, ParseError, ParseStream};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Punctuated<Item, Punct> {
    pub items: Vec<(Item, Punct)>,
    pub trailing: Option<Item>,
}

impl<Item, Punct> Punctuated<Item, Punct> {
    pub fn new(items: Vec<(Item, Punct)>, trailing: Option<Item>) -> Self {
        Self { items, trailing }
    }

    pub fn len(&self) -> usize {
        self.items.len() + usize::from(self.trailing.is_some())
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.trailing.is_none()
    }

    pub fn iter(&self) -> impl Iterator<Item=(&Item, Option<&Punct>)> {
        self.items
            .iter()
            .map(|(item, punct)| (item, Some(punct)))
            .chain(self.trailing.iter().map(|item| (item, None)))
    }

    pub fn items(&self) -> impl Iterator<Item=&Item> {
        self.items
            .iter()
            .map(|(item, _)| item)
            .chain(self.trailing.iter())
    }

    pub fn items_mut(&mut self) -> impl Iterator<Item=&mut Item> {
        self.items.iter_mut().map(|(item, _)| item)
    }
}

impl<'a, Item: Parse<'a>, Punct: Parse<'a>> Parse<'a> for Punctuated<Item, Punct> {
    #[inline(always)]
    fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        input.parse_punctuated()
    }
}

pub struct Items<'a, Item, Punct> {
    parent: &'a Punctuated<Item, Punct>,
    index: usize,
}

impl<'a, Item, Punct> Iterator for Items<'a, Item, Punct> {
    type Item = &'a Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index.cmp(&self.parent.items.len()) {
            Ordering::Less => {
                let item = &self.parent.items[self.index].0;
                self.index += 1;
                Some(item)
            }
            Ordering::Equal => {
                self.index += 1;
                self.parent.trailing.as_ref()
            }
            Ordering::Greater => None,
        }
    }
}

impl<'a, Item, Punct> IntoIterator for &'a Punctuated<Item, Punct> {
    type Item = &'a Item;
    type IntoIter = Items<'a, Item, Punct>;

    fn into_iter(self) -> Self::IntoIter {
        Items {
            parent: self,
            index: 0,
        }
    }
}

impl<Item: Display, Punct: Display> Display for Punctuated<Item, Punct> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (item, punct) in &self.items {
            write!(f, "{}{}", item, punct)?;
        }
        if let Some(item) = &self.trailing {
            write!(f, "{}", item)?;
        }
        Ok(())
    }
}
