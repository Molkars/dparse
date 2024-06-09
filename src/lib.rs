use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::rc::Rc;

#[cfg(feature = "c_ident")]
pub mod c_ident;

pub type ParserFilter = Rc<dyn for<'b> Fn(&mut Parser<'b>)>;

#[derive(Clone)]
pub struct Parser<'a> {
    pub source: &'a str,
    pub location: Location,
    pub whitespace: Option<ParserFilter>,
    // comment: Option<Rc<dyn for<'b> Fn(&mut Parser<'b>)>>,
}

impl<'a> Parser<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            source: s,
            location: Location {
                index: 0,
                line: 1,
                column: 1,
            },
            whitespace: None,
        }
    }

    pub fn with_whitespace(mut self, whitespace: impl Fn(&mut Parser) + 'static) -> Self {
        self.whitespace = Some(Rc::new(whitespace));
        self
    }
}

#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub location: Location,
    pub length: usize,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "parse error occurred at {}:{} ({})", self.location.line, self.location.column, self.location.index)?;
        writeln!(f, "  {}", self.message)?;
        Ok(())
    }
}

impl Error for ParseError {}

impl ParseError {
    #[inline]
    pub fn new(message: impl Into<String>, location: Location) -> Self {
        Self {
            message: message.into(),
            location,
            length: 1,
        }
    }

    #[inline]
    pub fn new_spanned(message: impl Into<String>, location: Location, length: usize) -> Self {
        Self {
            message: message.into(),
            location,
            length,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn advance(&mut self, c: char) {
        self.index += c.len_utf8();
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

impl<'a> Parser<'a> {
    pub fn remaining(&self) -> &str {
        &self.source[self.location.index..]
    }

    /// parse while being mindful of whitespace
    pub fn atomic<T>(
        &mut self,
        parse_fn: impl FnOnce(&mut Parser<'a>) -> Result<T, ParseError>,
    ) -> Result<T, ParseError> {
        self.whitespace();
        let mut parser = self.clone();
        parser.whitespace = None;
        let value = parse_fn(&mut parser);
        self.location = parser.location;
        value
    }

    pub fn at_end(&mut self) -> bool {
        if let Some(whitespace) = self.whitespace.clone() {
            whitespace(self);
        }
        self.location.index >= self.source.len()
    }

    pub fn whitespace(&mut self) {
        if let Some(whitespace) = self.whitespace.clone() {
            let mut parser = self.clone();
            parser.whitespace = None;
            whitespace(&mut parser);
            self.location = parser.location;
        }
    }

    #[inline]
    pub fn peek<P: ParsePrimitive>(&mut self, filter: P) -> bool {
        self.whitespace();
        filter.peek(self)
    }

    #[inline]
    pub fn peek_unless<P: ParsePrimitive + Copy>(&mut self, filter: P) -> Option<char> {
        self.whitespace();
        if filter.peek(self) {
            None
        } else {
            self.peek_char()
        }
    }

    #[inline]
    pub fn peek_terminal<P: ParsePrimitive>(&mut self, filter: P) -> bool {
        self.whitespace();
        self.at_end() || filter.peek(self)
    }


    #[inline]
    pub fn take<P: ParsePrimitive>(&mut self, filter: P) -> bool {
        self.whitespace();
        filter.take(self)
    }

    #[inline]
    pub fn take_if<P: ParsePrimitive + Copy>(&mut self, filter: P) -> Option<char> {
        self.whitespace();
        if filter.peek(self) {
            self.take_char()
        } else {
            None
        }
    }

    #[inline]
    pub fn take_unless<P: ParsePrimitive + Copy>(&mut self, filter: P) -> Option<char> {
        self.whitespace();
        if filter.peek(self) {
            None
        } else {
            self.take_char()
        }
    }

    #[inline]
    pub fn expect<P: ParsePrimitive + Copy + Debug>(&mut self, filter: P) -> Result<(), ParseError> {
        self.whitespace();
        if filter.take(self) {
            Ok(())
        } else {
            Err(ParseError::new_spanned(format!("Expected {:?}", filter), self.location, filter.len()))
        }
    }

    #[inline]
    pub fn peek_char(&mut self) -> Option<char> {
        self.whitespace();
        self.remaining().chars().next()
    }

    #[inline]
    pub fn take_char(&mut self) -> Option<char> {
        self.peek_char()
            .inspect(|c| self.location.advance(*c))
    }
}

pub trait ParsePrimitive {
    fn peek(self, parser: &Parser) -> bool;
    fn take(self, parser: &mut Parser) -> bool;
    fn len(&self) -> usize;
}

impl ParsePrimitive for char {
    #[inline]
    fn peek(self, parser: &Parser) -> bool {
        let c = parser.source[parser.location.index..].chars().next();
        c == Some(self)
    }

    fn take(self, parser: &mut Parser) -> bool {
        if self.peek(parser) {
            parser.location.advance(self);
            true
        } else {
            false
        }
    }

    fn len(&self) -> usize {
        1
    }
}

impl ParsePrimitive for &'_ str {
    #[inline]
    fn peek(self, parser: &Parser) -> bool {
        parser.source[parser.location.index..].starts_with(self)
    }

    fn take(self, parser: &mut Parser) -> bool {
        let cursor = &parser.source[parser.location.index..];
        if self.len() > cursor.len() {
            return false;
        }

        let mut location = parser.location;
        for (a, b) in cursor.chars().zip(self.chars()) {
            if a != b {
                return false;
            }
            location.advance(a);
        }
        parser.location = location;

        true
    }

    fn len(&self) -> usize {
        str::len(self)
    }
}

impl<T: FnOnce(char) -> bool> ParsePrimitive for T {
    fn peek(self, parser: &Parser) -> bool {
        parser.source[parser.location.index..]
            .chars()
            .next()
            .is_some_and(self)
    }

    fn take(self, parser: &mut Parser) -> bool {
        parser.source[parser.location.index..]
            .chars()
            .next()
            .filter(|c| self(*c))
            .inspect(|c| parser.location.advance(*c))
            .is_some()
    }

    fn len(&self) -> usize {
        1
    }
}

#[cfg(test)]
mod lock_test {
    use super::{Parser, ParserFilter};

    struct WsLock<'a, 'parent> where 'a: 'parent {
        previous: Option<ParserFilter>,
        parser: &'parent mut MyParser<'a>
    }
    impl<'a, 'b> std::ops::Deref for WsLock<'a, 'b> {
        type Target = MyParser<'a>;
        fn deref(&self) -> &Self::Target { &self.parser }
    }
    impl<'a, 'b> std::ops::DerefMut for WsLock<'a, 'b> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.parser }
    }
    impl<'a, 'b> std::ops::Drop for WsLock<'a, 'b> {
        fn drop(&mut self) {
            use std::ops::DerefMut;
            self.parser.deref_mut().whitespace = self.previous.clone();
        }
    }

    struct MyParser<'a> {
        inner: Parser<'a>,
    }
    impl<'a> std::ops::Deref for MyParser<'a> {
        type Target = Parser<'a>;
        fn deref(&self) -> &Self::Target { &self.inner }
    }
    impl std::ops::DerefMut for MyParser<'_> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.inner }
    }

    impl<'a> MyParser<'a> {
        pub fn new(source: &'a str) -> Self {
            let inner = Parser::new(source)
                .with_whitespace(|parser| while parser.take(char::is_whitespace) {});
            Self { inner }
        }

        pub fn no_whitespace(&mut self) -> WsLock<'a, '_> {
            let previous = self.whitespace.clone();
            self.whitespace = None;
            WsLock {
                previous,
                parser: self,
            }
        }
    }

    #[test]
    fn lock_test() {
        const SRC: &str = r#" " hi there " "#;
        let mut parser = MyParser::new(SRC);

        assert!(parser.take('"'));
        let string = {
            let mut parser = parser.no_whitespace();
            let start = parser.location;
            while !parser.peek_terminal('"') {
                parser.take_char();
            }
            let end = parser.location;
            parser.source[start.index..end.index].to_owned()
        };
        assert_eq!(string, " hi there ");
        assert!(parser.take('"'));
    }
}

#[test]
fn whitespace_test() {
    const SRC: &str = " 1";
    let mut parser = Parser::new(SRC)
        .with_whitespace(|parser| while parser.take(char::is_whitespace) {});
    assert_eq!(Some('1'), parser.take_char());
}

#[test]
fn parser_demo() {
    let parse_no = |parser: &mut Parser| {
        // atomic disables the filter for the inner function
        parser.atomic(|parser| {
            let is_negative = parser.take('-');

            if !parser.peek(|c: char| c.is_ascii_digit()) {
                return Err(ParseError::new("expected digit", parser.location));
            }

            let mut out = 0i32;
            while let Some(c) = parser.take_if(|c: char| c.is_ascii_digit()) {
                out *= 10;
                out += c as i32 - '0' as i32;
            }
            if is_negative {
                out *= -1;
            }
            Ok(out)
        })
    };

    let parse_str = |parser: &mut Parser| {
        if !parser.take('\'') {
            return Ok(None);
        }

        Some(parser.atomic(|parser| {
            let mut content = String::new();
            while let Some(c) = parser.take_unless('\'') {
                content.push(c);
            }
            parser.expect('\'')?;
            Ok(content)
        })).transpose()
    };

    let mut parser = Parser::new("-12 'hey!' okay?")
        .with_whitespace(|p| while p.take(char::is_whitespace) {});
    assert!(matches!(parse_no(&mut parser), Ok(-12i32)));
    assert!(matches!(parse_str(&mut parser), Ok(Some(x)) if x == "hey!"));
    assert!(parser.expect("got it?").is_err());
    assert!(parser.expect("okay?").is_ok());
    assert!(parser.at_end())
}
