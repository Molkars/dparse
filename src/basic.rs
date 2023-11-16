use std::str::FromStr;
use crate::punct;
use crate::parse::{Parse, ParseError, ParseStream, Span};

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct CIdent {
    value: String,
    span: Span,
}

impl CIdent {
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn content(&self) -> &str {
        &self.value
    }
}

impl Parse<'_> for CIdent {
    fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
        input.take_while(|c| c.is_whitespace());
        if !input.peek_char().is_some_and(|c| c.is_ascii_digit()) {
            return Err(input.mismatch());
        }
        let spanner = input.spanner();
        let content = input.take_while(|c| c.is_ascii_alphanumeric() || c == '_');

        Ok(CIdent { value: content.to_string(), span: input.span(spanner) })
    }
}

punct! {
    pub struct Tick("`");
    pub struct Tilde("~");
    pub struct Bang("!");
    pub struct At("@");
    pub struct Hash("#");
    pub struct Dollar("$");
    pub struct Percent("%");
    pub struct Caret("^");
    pub struct Ampersand("&");
    pub struct Star("*");
    pub struct OpenParen("(");
    pub struct CloseParen(")");
    pub struct Dash("-");
    pub struct Underscore("_");
    pub struct Plus("+");
    pub struct Equals("=");
    pub struct OpenBracket("[");
    pub struct OpenBrace("{");
    pub struct CloseBracket("]");
    pub struct CloseBrace("}");
    pub struct Pipe("|");
    pub struct Backslash("\\");
    pub struct Colon(":");
    pub struct SemiColon(";");
    pub struct Apostrophe("'");
    pub struct Quote("\"");
    pub struct LeftArrow("<");
    pub struct Comma(",");
    pub struct RightArrow(">");
    pub struct Dot(".");
    pub struct Question("?");
    pub struct Slash("/");
    pub struct TildeEquals("~=");
    pub struct BangEquals("!=");
    pub struct PercentEquals("%=");
    pub struct CaretEquals("^=");
    pub struct AmpersandEquals("&=");
    pub struct StarEquals("*=");
    pub struct Parens("()");
    pub struct DashEquals("-=");
    pub struct DashArrow("->");
    pub struct PlusEquals("+=");
    pub struct DoubleEquals("==");
    pub struct TripleEquals("===");
    pub struct EqualArrow("=>");
    pub struct Brackets("[]");
    pub struct Braces("{}");
    pub struct BarEquals("|=");
    pub struct DoubleColon("::");
    pub struct ColonEquals(":=");
    pub struct DoubleLeftArrow("<<");
    pub struct TripleLeftArrow("<<<");
    pub struct DoubleRightArrow(">>");
    pub struct TripleRightArrow(">>>");
    pub struct SlashEquals("/=");
    pub struct DoubleSlash("//");
    pub struct DoubleQuestion("??");
    pub struct DoubleDot("..");
    pub struct TripleDot("...");
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LitBool {
    value: bool,
    span: Span,
}

impl LitBool {
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn content(&self) -> bool {
        self.value
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LitInt {
    value: u64,
    span: Span,
}

impl LitInt {
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn content(&self) -> u64 {
        self.value
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct LitDecimal {
    value: f64,
    span: Span,
}

impl LitDecimal {
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn content(&self) -> f64 {
        self.value
    }
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct LitCStr {
    value: String,
    span: Span,
}

impl LitCStr {
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline]
    pub fn content(&self) -> &str {
        &self.value
    }
}

impl Parse<'_> for LitBool {
    fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
        input.take_while(|c| c.is_whitespace());
        let spanner = input.spanner();
        let value = input.take_while(|c| c.is_alphabetic());
        match value {
            "true" => Ok(LitBool { value: true, span: input.span(spanner) }),
            "false" => Ok(LitBool { value: false, span: input.span(spanner) }),
            _ => {
                input.reset(spanner);
                Err(input.error("expected boolean literal"))
            }
        }
    }
}

impl Parse<'_> for LitInt {
    fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
        input.take_while(|c| c.is_whitespace());
        let spanner = input.spanner();
        let radix = if input.take_str("0x") {
            16
        } else if input.take_str("0o") {
            8
        } else if input.take_str("0b") {
            2
        } else {
            10
        };

        let content = input.take_while(|c| c.is_digit(radix) || c == '_');
        if content.is_empty() {
            if radix == 10 {
                input.reset(spanner);
                return Err(input.error("expected integer literal"));
            }
            return Err(input.error(format!("expected integer literal with radix: {}", radix)));
        }

        let value = u64::from_str_radix(content, radix)
            .map_err(|e| input.error(format!("invalid integer literal: {}", e)))?;
        Ok(LitInt { value, span: input.span(spanner) })
    }
}

impl Parse<'_> for LitDecimal {
    fn parse(input: &mut ParseStream<'_>) -> Result<Self, ParseError> {
        input.take_while(|c| c.is_whitespace());
        let spanner = input.spanner();
        let content = input.take_while(|c| c.is_digit(10) || c == '_');
        if content.is_empty() {
            return Err(input.mismatch());
        }

        if input.take_char('e') || input.take_char('E') {
            let _ = input.take_char('+') || input.take_char('-');
            let exponent = input.take_while(|c| c.is_digit(10) || c == '_');
            if exponent.is_empty() {
                input.reset(spanner);
                return Err(input.error("invalid exponential literal! <float>e[+-]<int>"));
            }
        } else if !input.take_char('.') {
            input.reset(spanner);
            return Err(input.mismatch()); // this is kinda shaky but for integer compat we'll go with it
        } else {
            let content = input.take_while(|c| c.is_digit(10) || c == '_');
            if content.is_empty() {
                input.reset(spanner);
                return Err(input.error("invalid decimal literal! <int>.<int>"));
            }
        }

        let content = input.source_for_span(input.span(spanner));
        let value = f64::from_str(content)
            .map_err(|e| input.error(format!("invalid decimal literal: {}", e)))?;
        Ok(LitDecimal { value, span: input.span(spanner) })
    }
}

impl Parse<'_> for LitCStr {
    fn parse(input: &mut ParseStream) -> Result<Self, ParseError> {
        input.take_while(|c| c.is_whitespace());
        let span = input.spanner();

        if !input.take_char('"') {
            return Err(input.mismatch());
        }

        let mut content = String::new();
        while let Some(c) = input.peek_char() {
            if c == '\r' || c == '\n' || c == '"' {
                break;
            }

            if c != '\\' {
                content.push(c);
                input.advance();
                continue;
            }

            input.advance();
            let Some(c) = input.advance() else {
                break;
            };

            match c {
                'n' => content.push('\n'),
                'r' => content.push('\r'),
                't' => content.push('\t'),
                '\\' => content.push('\\'),
                '"' => content.push('"'),
                'u' => {
                    if !input.take_char('{') {
                        return Err(input.error("invalid escape sequence"));
                    }
                    let span = input.spanner();
                    let _ = std::iter::from_fn(|| input.advance())
                        .take(4)
                        .filter(char::is_ascii_hexdigit)
                        .count();
                    let span = input.span(span);
                    if !input.take_char('}') {
                        return Err(input.error("invalid escape sequence -- missing closing `}`"));
                    }
                    let substring = input.source_for_span(span);
                    let Ok(codepoint) = u32::from_str_radix(substring, 16) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    let Some(codepoint) = char::from_u32(codepoint) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    content.push(codepoint);
                }
                'U' => {
                    if !input.take_char('{') {
                        return Err(input.error("invalid escape sequence"));
                    }

                    let span = input.spanner();
                    let _ = std::iter::from_fn(|| input.advance())
                        .take(8)
                        .filter(char::is_ascii_hexdigit)
                        .count();
                    let span = input.span(span);

                    if !input.take_char('}') {
                        return Err(input.error("invalid escape sequence -- missing closing `}`"));
                    }
                    let substring = input.source_for_span(span);
                    let Ok(codepoint) = u32::from_str_radix(substring, 16) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    let Some(codepoint) = char::from_u32(codepoint) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    content.push(codepoint);
                }
                'x' => {
                    let span = input.spanner();
                    let _ = std::iter::from_fn(|| input.advance())
                        .take(2)
                        .filter(char::is_ascii_hexdigit)
                        .count();
                    let span = input.span(span);
                    let substring = input.source_for_span(span);
                    let Ok(codepoint) = u32::from_str_radix(substring, 16) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    let Some(codepoint) = char::from_u32(codepoint) else {
                        return Err(input.error("invalid escape sequence -- invalid codepoint"));
                    };
                    content.push(codepoint);
                }
                _ => return Err(input.error("invalid escape sequence")),
            }
        }
        if !input.take_char('"') {
            return Err(input.error("unterminated string literal"));
        }

        Ok(Self {
            value: content,
            span: input.span(span),
        })
    }
}
