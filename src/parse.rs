use std::cmp::Ordering;
use std::collections::LinkedList;
use std::fmt::{Debug, Display, Formatter};

pub use on_line::OnLine;
pub use punctuated::Punctuated;
pub use separated::Separated;
pub use stream::ParseStream;

mod on_line;
mod punctuated;
mod separated;
mod stream;

pub trait Parse<'a>
    where
        Self: Sized,
{
    fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError>;
}

#[non_exhaustive]
pub struct ParseError {
    pub(crate) mismatch: bool,
    pub(crate) messages: LinkedList<String>,
    pub(crate) span: Span,
    #[cfg(any(debug_assertions, feature = "track_caller"))]
    pub(crate) trace: std::backtrace::Backtrace,
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ParseError")
            .field("mismatch", &self.mismatch)
            .field("messages", &self.messages)
            .field("span", &self.span)
            .finish()
    }
}

impl std::error::Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "parse error at {}:", self.span)?;
        for (i, message) in self.messages.iter().enumerate() {
            writeln!(f, "  {: <4}: {}", i + 1, message)?;
        }
        #[cfg(any(debug_assertions, feature = "track_caller"))]
        writeln!(f, "  {: <4}: {}", self.messages.len() + 1, self.trace)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Span {
    index: usize,
    length: usize,
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({}:+{})", self.index, self.length)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:+..{}", self.index, self.length)
    }
}

impl Span {
    pub fn display<'a>(self, stream: &'a ParseStream<'_>) -> impl Display + 'a {
        DisplaySpan(self, stream)
    }

    pub fn highlight<'a>(&self, src: &'a str) -> impl Display + 'a {
        struct Highlight<'a> {
            span: Span,
            src: &'a str,
        }

        impl Display for Highlight<'_> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "your code here lol")?;
                writeln!(f, "^^^^^^^^^^^^^^^^^^")?;
                Ok(())
            }
        }

        Highlight { span: *self, src }
    }

    pub fn through(self, other: Span) -> Result<Span, ()> {
        if self.index > other.index {
            return Err(());
        }

        Ok(Self {
            index: self.index,
            length: other.end() - self.start(),
        })
    }

    pub fn start(self) -> usize {
        self.index
    }

    pub fn end(self) -> usize {
        self.index + self.length
    }

    pub fn len(self) -> usize {
        self.length
    }
}

struct DisplaySpan<'a>(Span, &'a ParseStream<'a>);

impl<'a> Display for DisplaySpan<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self(span, stream) = self;

        match span.index.cmp(&stream.source.len()) {
            Ordering::Equal => write!(f, "end of input"),
            Ordering::Greater => write!(f, "(invalid span)"),
            Ordering::Less => {
                let (line, column) =
                    stream.source[..span.index]
                        .chars()
                        .fold((1, 1), |(line, col), c| match c {
                            '\n' => (line + 1, 1),
                            _ => (line, col + 1),
                        });
                write!(f, "{}:{}", line, column)
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Spanner {
    index: usize,
}

impl ParseError {
    pub fn messages(&self) -> impl Iterator<Item=&str> + '_ {
        self.messages.iter().map(|s| s.as_str())
    }

    pub fn is_mismatch(&self) -> bool {
        self.mismatch
    }

    pub fn span(&self) -> Span {
        self.span
    }

    #[cfg(any(debug_assertions, feature = "track_caller"))]
    pub fn backtrace(&self) -> &std::backtrace::Backtrace {
        &self.trace
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let mut vec = Vec::new();
        while let Some(item) = input.try_parse::<T>()? {
            vec.push(item);
        }
        if vec.is_empty() {
            Err(input.mismatch())
        } else {
            Ok(vec)
        }
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Box<T> {
    fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        T::parse(input).map(Box::new)
    }
}

impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        input.try_parse()
    }
}

macro_rules! tuple_impl {
    ($($name:ident),*) => {
        impl<'a, $($name: Parse<'a>),*> Parse<'a> for ($($name,)*) {
            fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError> {
                Ok(($(input.parse::<$name>()?,)*))
            }
        }
    }
}

tuple_impl!(T0);
tuple_impl!(T0, T1);
tuple_impl!(T0, T1, T2);
tuple_impl!(T0, T1, T2, T3);
tuple_impl!(T0, T1, T2, T3, T4);
tuple_impl!(T0, T1, T2, T3, T4, T5);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
tuple_impl!(T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);


pub trait WithMessage {
    type Output;

    fn with_message(self, message: impl Into<String>) -> Self::Output;
}

impl<T> WithMessage for Result<T, ParseError> {
    type Output = Self;

    fn with_message(self, message: impl Into<String>) -> Self::Output {
        self.map_err(|mut e| {
            e.messages.push_front(message.into());
            e
        })
    }
}