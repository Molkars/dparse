
# dparse - [D]umb [Parse]r

A simple parsing library for rust

```rust
use dparse::{Parser, ParseError};

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
```

### Configurable Ignorance

- ignore what you don't want to parse

```rs

fn ignore_whitespace(parser: &mut Parser) {
    let current_filter = parser.whitespace.clone();
    parser.whitespace = Some(Rc::new(|parser| {
        // NOTE: this parser has `.whitespace = None` so we can use parser methods without infinite recursion
        // If you need a whitespace handler, clone this parser and give the clone a handler
        while parser.take(char::is_whitespace) {
            // nothing
        }
    }));
}

```

or if you want a temporarily locked parser for some reason

```rs
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
```

I think you're better off writing wrapped methods:

```rs
impl<'a> MyParser<'a> {
    pub fn atomic<T>(
        &mut self,
        parse_fn: impl FnOnce(&mut MyParser<'a>) -> Result<T, ParseError>,
    ) -> Result<T, ParseError> {
        self.whitespace();
        let mut parser = MyParser(self.0.clone()); // or whatever
        parser.0.whitespace = None;
        let value = parse_fn(&mut parser);
        self.location = parser.location;
        value
    }
}
```
