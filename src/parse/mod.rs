mod error;

use std::any::type_name;
pub use error::{ParseError, ParseErrorWithContext};
use crate::{Location, Span};

pub type ParseResult<T> = Result<T, ParseError>;

pub trait Parser<'src> {
    type Lock;

    fn source(&self) -> &'src str;
    fn source_for_span(&self, span: Span) -> Option<&'src str> {
        let start = span.start.index;
        let end = span.end.index;
        self.source().get(start..end)
    }
    fn fork(&self) -> Self where Self: Sized;

    fn preserve_whitespace(&mut self) -> Self::Lock;
    fn ignore_whitespace(&mut self) -> Self::Lock;
    fn allows_whitespace(&self) -> bool;

    fn location(&self) -> Location;
    fn span(&self) -> Span;
    fn set_location(&mut self, loc: Location) -> Result<(), Location>;


    fn more(&self) -> bool {
        self.location().index < self.source().len()
    }

    #[inline]
    fn peek(&self) -> Option<char> {
        self.peek_by_n(0)
    }
    fn peek_by_n(&self, off: usize) -> Option<char>;
    #[inline]
    fn peek_n(&self, n: usize) -> Option<&'src str> {
        self.peek_n_by_n(n, 0)
    }
    fn peek_n_by_n(&self, n: usize, off: usize) -> Option<&'src str>;

    fn consume(&mut self) -> Option<char>;
    fn consume_n(&mut self, n: usize) -> Option<&'src str> {
        let start = self.location();
        for _ in 0..n {
            self.consume()?;
        }
        let span = start.span(self.location())
            .expect("Parser::consume_n: span failed");
        self.source_for_span(span)
    }
    fn consume_str(&mut self, str: &str) -> Option<Span> {
        if !self.match_str(str) {
            return None;
        }
        let start = self.location();
        for _ in 0..str.len() {
            self.consume()?;
        }
        let span = start.span(self.location())
            .expect("Parser::consume_str: span failed");
        Some(span)
    }
    #[inline]
    fn consume_char(&mut self, c: char) -> Option<Span> {
        if !self.match_char(c) {
            return None;
        }
        let start = self.location();
        self.consume()?;
        let span = start.span(self.location())
            .expect("Parser::consume_char: span failed");
        Some(span)
    }

    #[inline]
    fn take_str(&mut self, str: &str) -> bool {
        if self.match_str(str) {
            self.consume_n(str.len());
            true
        } else {
            false
        }
    }
    #[inline]
    fn take_char(&mut self, c: char) -> bool {
        if self.match_char(c) {
            self.consume();
            true
        } else {
            false
        }
    }

    fn take_while(&mut self, f: impl Fn(char) -> bool) -> Option<&'src str> {
        let start = self.location();
        while let Some(c) = self.peek() {
            if !f(c) {
                break;
            }
            self.consume();
        }
        if self.location() == start {
            return None;
        }
        let span = start.span(self.location())
            .expect("Parser::take_while: span failed");
        let source = self.source_for_span(span)
            .expect("Parser::take_while: source_for_span failed");
        Some(source)
    }

    fn match_char(&self, c: char) -> bool {
        self.peek() == Some(c)
    }
    fn match_str(&self, str: &str) -> bool {
        self.peek_n(str.len()) == Some(str)
    }

    #[track_caller]
    fn parse<T: Parse<'src>>(&mut self) -> ParseResult<T> {
        T::parse(self)
    }

    #[track_caller]
    fn try_parse<T: Parse<'src>>(&mut self) -> ParseResult<Option<T>> {
        let start = self.location();
        let result = T::parse(self);
        match result {
            Ok(t) => Ok(Some(t)),
            Err(e) if e.is_mismatch() => {
                self.set_location(start)
                    .expect("Parser::try_parse: set_location failed");
                Ok(None)
            }
            Err(e) => Err(e),
        }
    }

    fn fork_parse<T: Parse<'src>>(&mut self) -> ParseResult<T> where Self: Sized {
        self.fork().parse()
    }

    #[track_caller]
    fn require<T: Parse<'src>>(&mut self) -> Result<T, ParseError> {
        let start = self.location();
        match T::parse(self) {
            Ok(item) => Ok(item),
            Err(mut e) if e.is_mismatch() => {
                let span = start.span(self.location())
                    .expect("Parser::require: span failed");
                e.mismatch = false;
                Err(e.context(span, format!("required: {}", type_name::<T>())))
            }
            Err(e) => Err(e),
        }
    }

    #[track_caller]
    fn mismatch(&self, msg: impl Into<String>) -> ParseError {
        let loc = self.span();
        ParseError::mismatch(loc, msg.into())
    }

    #[track_caller]
    fn error(&self, msg: impl Into<String>) -> ParseError {
        let loc = self.span();
        ParseError::new(loc, msg.into())
    }
}

pub trait Parse<'src>: Sized {
    fn parse<P: Parser<'src> + ?Sized>(parser: &mut P) -> Result<Self, ParseError>;
}

// impls

impl<'src, T: Parse<'src>> Parse<'src> for Box<T> {
    fn parse<P: Parser<'src> + ?Sized>(parser: &mut P) -> Result<Self, ParseError> {
        parser.parse().map(Box::new)
    }
}

impl<'src, T: Parse<'src>> Parse<'src> for Option<T> {
    fn parse<P: Parser<'src> + ?Sized>(parser: &mut P) -> Result<Self, ParseError> {
        parser.try_parse()
    }
}

impl<'src, T: Parse<'src>> Parse<'src> for Vec<T> {
    fn parse<P: Parser<'src> + ?Sized>(parser: &mut P) -> Result<Self, ParseError> {
        let mut vec = Vec::new();
        while let Some(item) = parser.try_parse()? {
            vec.push(item);
        }
        Ok(vec)
    }
}

macro_rules! tuple_impl {
    ($n:literal; $first:ident, $($name:ident),*) => {
        impl<'src, $first: Parse<'src> $(, $name: Parse<'src>)*> Parse<'src> for ($first, $($name,)*) {
            #[allow(non_snake_case)]
            fn parse<P: Parser<'src> + ?Sized>(parser: &mut P) -> Result<Self, ParseError> {
                let $first = parser.parse()?;
                $(
                    let $name = parser.require::<$name>()
                        .context(parser.span(), format!("expected {} in {}", type_name::<$name>(), type_name::<Self>()))?;
                )*
                Ok(($first, $($name,)*))
            }
        }
    };
}

tuple_impl!(1; A,);
tuple_impl!(2; A, B);
tuple_impl!(3; A, B, C);
