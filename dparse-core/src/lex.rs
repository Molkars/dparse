use std::ops::Index;
use crate::span::{Location, Span};

#[inline]
fn make_span(start: Location, end: Location) -> Option<Span> {
    (start.index != end.index)
        .then(|| Span {
            start,
            len: end.index - start.index,
        })
}

pub enum LexResult<T> {
    Some(T),
    Err(LexError),
    None,
}

impl<T> LexResult<T> {
    pub fn is_some(&self) -> bool {
        matches!(self, LexResult::Some(_))
    }
    
    pub fn is_none(&self) -> bool {
        matches!(self, LexResult::None)
    }
    
    pub fn is_err(&self) -> bool {
        matches!(self, LexResult::Err(_))
    }
}

pub struct LexError {
    pub span: Span,
    pub kind: LexErrorKind,
}

pub enum LexErrorKind {

}

pub trait Lex: Sized {
    fn lex<L: Lexer>(lexer: &mut L) -> LexResult<Self>;
}

pub trait Lexer {
    fn source(&self) -> &str;
    fn location(&self) -> Location;
    fn set_location(&mut self, location: Location);
    
    fn span_contents(&self, span: Span) -> &str {
        &self.source()[span.range()]
    }

    fn cursor(&self) -> &str {
        self.source().index(self.location().index..)
    }

    fn peek(&self) -> Option<char> {
        self.cursor().chars().next()
    }

    fn take(&mut self) -> Option<char> {
        let mut location = self.location();
        let c = self.peek()?;
        location.index += c.len_utf8();
        if c == '\n' {
            location.line += 1;
            location.column = 0;
        } else {
            location.column += 1;
        }
        self.set_location(location);
        Some(c)
    }

    fn take_while(&mut self, f: impl Fn(char) -> bool) -> Option<Span> {
        let location = self.location();
        while self.peek().is_some_and(&f) {
            self.take();
        }
        make_span(location, self.location())
    }

    fn peek_char(&self, c: char) -> bool {
        self.peek().is_some_and(|x| x == c)
    }

    #[inline]
    fn peek_str(&self, s: &str) -> bool {
        self.cursor().get(..s.len()).is_some_and(|x| x == s)
    }

    fn take_char(&mut self, c: char) -> Option<Span> {
        self.peek_char(c)
            .then(|| {
                let start = self.location();
                self.take();
                make_span(start, self.location()).expect("take_char")
            })
    }

    fn take_str(&mut self, s: &str) -> Option<Span> {
        self.peek_str(s)
            .then(|| {
                let start = self.location();
                for _ in 0..s.len() {
                    self.take();
                }
                make_span(start, self.location()).expect("take_str")
            })
    }
}