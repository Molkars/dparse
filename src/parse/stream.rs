use std::backtrace::Backtrace;
use std::collections::LinkedList;
use std::ops::Index;

use crate::parse::separated::Separated;
use crate::parse::{Parse, ParseError, Punctuated, Span, Spanner};

#[derive(Clone, Copy, Debug)]
pub struct ParseStream<'a> {
    pub(super) source: &'a str,
    pub(super) cursor: &'a str,
    pub(super) start_index: usize,
    pub(super) index: usize,
}

impl<'a> ParseStream<'a> {
    #[inline(always)]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: source,
            start_index: 0,
            index: 0,
        }
    }

    pub fn parse_punctuated<Item: Parse<'a>, Punct: Parse<'a>>(
        &mut self,
    ) -> Result<Punctuated<Item, Punct>, ParseError> {
        let mut items = Vec::new();
        let start = self.start_index;
        self.start_index = self.index;
        let mut trailing = None;

        loop {
            let Some(item) = self.try_parse::<Item>()? else {
                break;
            };

            let Some(punct) = self.try_parse::<Punct>()? else {
                trailing = Some(item);
                break;
            };

            items.push((item, punct));
        }

        if trailing.is_none() && items.is_empty() {
            return Err(self.mismatch());
        }

        self.start_index = start;
        Ok(Punctuated { items, trailing })
    }

    pub fn parse_separated<Item: Parse<'a>, Punct: Parse<'a>>(
        &mut self,
    ) -> Result<Separated<Item, Punct>, ParseError> {
        let mut item = Item::parse(self)?;
        let start = self.start_index;
        self.start_index = self.index;

        let mut items = Vec::new();
        let mut span = self.spanner();
        while let Some(punct) = self.try_parse::<Punct>()? {
            let Some(next) = self.try_parse::<Item>()? else {
                self.reset(span);
                break;
            };
            span = self.spanner();
            items.push((item, punct));
            item = next;
        }

        self.start_index = start;
        Ok(Separated {
            items,
            trailing: item,
        })
    }

    pub fn parse<T: Parse<'a>>(&mut self) -> Result<T, ParseError> {
        let start = self.start_index;
        self.start_index = self.index;

        let value = T::parse(self)?;

        self.start_index = start;
        Ok(value)
    }

    /// Attempts to parse a value from the stream. transposes the result for ease of the try trait.
    #[track_caller]
    pub fn try_parse<T: Parse<'a>>(&mut self) -> Result<Option<T>, ParseError> {
        let start = self.start_index;
        self.start_index = self.index;

        let spanner = self.spanner();
        match T::parse(self) {
            Ok(value) => {
                self.start_index = start;
                Ok(Some(value))
            }
            Err(e) if e.mismatch => {
                self.reset(spanner);
                self.start_index = start;
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    #[track_caller]
    pub fn did_parse<T: Parse<'a>>(&mut self) -> bool {
        let start = self.start_index;
        self.start_index = self.index;

        let spanner = self.spanner();
        match T::parse(self) {
            Ok(_) => {
                self.start_index = start;
                true
            }
            Err(e) if e.mismatch => {
                self.reset(spanner);
                self.start_index = start;
                false
            }
            Err(_) => {
                self.start_index = start;
                false
            }
        }
    }

    #[track_caller]
    #[inline(always)]
    pub fn require<T: Parse<'a>>(&mut self) -> Result<T, ParseError> {
        self.require_with::<T>(T::parse)
    }

    #[track_caller]
    pub fn require_with<T: Parse<'a>>(
        &mut self,
        f: impl FnOnce(&mut ParseStream<'a>) -> Result<T, ParseError>,
    ) -> Result<T, ParseError> {
        let start = self.start_index;
        self.start_index = self.index;
        let item = f(self);
        self.start_index = start;
        crate::required(self, item)
    }

    #[inline(always)]
    pub fn reset(&mut self, spanner: Spanner) {
        self.cursor = &self.source[spanner.index..];
        self.index = spanner.index;
        if self.index > self.start_index {
            self.start_index = self.index;
        }
    }

    #[inline(always)]
    pub fn spanner(&self) -> Spanner {
        Spanner { index: self.index }
    }

    #[inline(always)]
    pub fn span(&self, spanner: Spanner) -> Span {
        Span {
            index: spanner.index,
            length: self.index - spanner.index,
        }
    }

    #[inline(always)]
    #[cfg_attr(any(debug_assertions, feature = "track_caller"), track_caller)]
    pub fn mismatch(&self) -> ParseError {
        ParseError {
            mismatch: true,
            messages: LinkedList::from([String::from("mismatch")]),
            span: Span {
                index: self.start_index,
                length: self.index - self.start_index,
            },
            #[cfg(any(debug_assertions, feature = "track_caller"))]
            trace: Backtrace::capture(),
        }
    }

    #[inline(always)]
    #[cfg_attr(any(debug_assertions, feature = "track_caller"), track_caller)]
    pub fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError {
            mismatch: false,
            messages: LinkedList::from([message.into()]),
            span: Span {
                index: self.start_index,
                length: self.index - self.start_index,
            },
            #[cfg(any(debug_assertions, feature = "track_caller"))]
            trace: Backtrace::capture(),
        }
    }

    #[inline(always)]
    pub fn source_for_span(&self, span: Span) -> &'a str {
        self.source.index(span.index..).index(..span.length)
    }

    pub fn line_source_for_span(&self, span: Span) -> &'a str {
        let start = self.source[..span.index]
            .rfind('\n')
            .map(|i| i + 1)
            .unwrap_or(0);
        let end = self.source[span.index..]
            .find('\n')
            .map(|i| i + span.index)
            .unwrap_or(self.source.len());
        &self.source[start..end]
    }

    #[inline(always)]
    pub fn index(&self) -> usize {
        self.index
    }

    #[inline(always)]
    pub fn source(&self) -> &'a str {
        self.source
    }

    #[inline(always)]
    pub fn cursor(&self) -> &'a str {
        self.cursor
    }

    #[inline(always)]
    pub fn has_next(&self) -> bool {
        !self.cursor.is_empty()
    }

    pub fn find_char(&mut self, c: char) -> Option<Span> {
        let span = self.spanner();
        self.cursor.find(c).map(|index| {
            self.index += index;
            self.cursor = &self.cursor[index..];
            self.span(span)
        })
    }

    pub fn find_str(&mut self, s: &str) -> Option<Span> {
        let span = self.spanner();
        self.cursor.find(s).map(|index| {
            self.index += index;
            self.cursor = &self.cursor[index..];
            self.span(span)
        })
    }

    #[inline(always)]
    pub fn peek_char(&self) -> Option<char> {
        self.cursor.chars().next()
    }

    #[inline(always)]
    pub fn peek_str(&'a self, n: usize) -> Option<&'a str> {
        if self.cursor.len() < n {
            None
        } else {
            Some(&self.cursor[..n])
        }
    }

    pub fn expect(&mut self, content: impl AsRef<str>) -> Result<Span, ParseError> {
        let start = self.start_index;
        self.start_index = self.index;
        let content = content.as_ref();
        let spanner = self.spanner();
        if self.cursor.len() < content.len() {
            return Err(self.mismatch());
        }
        self.cursor
            .chars()
            .zip(content.chars())
            .all(|(a, b)| a == b)
            .then(|| {
                self.index += content.len();
                self.cursor = &self.cursor[content.len()..];
                self.span(spanner)
            })
            .ok_or_else(|| self.error(format!("expected `{}`", content)))
            .map(|span| {
                self.start_index = start;
                span
            })
    }

    pub fn parse_str(&mut self, content: impl AsRef<str>) -> Option<Span> {
        let content = content.as_ref();
        let spanner = self.spanner();
        if self.cursor.len() < content.len() {
            return None;
        }
        self.cursor
            .chars()
            .zip(content.chars())
            .all(|(a, b)| a == b)
            .then(|| {
                self.index += content.len();
                self.cursor = &self.cursor[content.len()..];
                self.span(spanner)
            })
    }


    pub fn match_char(&self, c: char) -> bool {
        self.cursor.chars().next() == Some(c)
    }

    pub fn match_str(&self, content: impl AsRef<str>) -> bool {
        let content = content.as_ref();
        self.cursor.len() >= content.len()
            && self
            .cursor
            .chars()
            .zip(content.chars())
            .all(|(a, b)| a == b)
    }

    pub fn take_char(&mut self, c: char) -> bool {
        if self.cursor.chars().next() == Some(c) {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn take_str(&mut self, content: impl AsRef<str>) -> bool {
        let content = content.as_ref();
        if self.cursor.len() >= content.len()
            && self
            .cursor
            .chars()
            .zip(content.chars())
            .all(|(a, b)| a == b)
        {
            self.index += content.len();
            self.cursor = &self.cursor[content.len()..];
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn advance(&mut self) -> Option<char> {
        let next = self.peek_char()?;
        self.index += next.len_utf8();
        self.cursor = &self.cursor[next.len_utf8()..];
        Some(next)
    }

    #[inline]
    pub fn take_while<F>(&mut self, f: F) -> &'a str
        where
            F: Fn(char) -> bool,
    {
        let mut chars = self.cursor.chars();
        let mut len = 0;
        while let Some(c) = chars.next() {
            if !f(c) {
                break;
            }
            len += c.len_utf8();
        }
        let result = &self.cursor[..len];
        self.cursor = &self.cursor[len..];
        self.index += len;
        result
    }
}
