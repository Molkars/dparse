use std::iter::once;
use dparse_core::{Lex, Lexer, LexResult, Span};

pub struct Ident(String);

impl Lex for Ident {
    fn lex<L: Lexer>(lexer: &mut L) -> LexResult<Self> {
        if lexer.peek().is_some_and(|c| c.is_ascii_alphabetic() || c == '_') {
            let span = lexer.take_while(|c| c.is_ascii_alphanumeric() || c == '_')
                .expect("take_while returned None");
            let content = lexer.slice(span).to_string();
            LexResult::Some(Ident(content))
        } else {
            LexResult::None
        }
    }
}

pub enum Number {
    Integer(u64),
    Decimal(f64),
}

impl Lex for Number {
    fn lex<L: Lexer>(lexer: &mut L) -> LexResult<Self> {
        if !lexer.peek().is_some_and(|c| c.is_ascii_digit()) {
            return LexResult::None;
        }

        let start = lexer.location();
        lexer.take_while(|c| c.is_ascii_digit() || c == '_');

        let mid = lexer.location();
        LexResult::Some(if lexer.take_char('.').is_some() && lexer.peek().is_some_and(|c| c.is_ascii_digit()) {
            lexer.take_while(|c| c.is_ascii_digit() || c == '_');
            
            let span = Span::new(start, lexer.location());
            let content = lexer.slice(span).replace('_', "");
            let value = content.parse().expect("parse failed");
            Number::Decimal(value)
        } else {
            lexer.set_location(mid);
            let span = Span::new(start, lexer.location());
            let content = lexer.slice(span).replace('_', "");
            let value = content.parse().expect("parse failed");
            Number::Integer(value)
        })
    }
}

pub struct LitString {
    
}

#[derive(Token)]
pub enum Tokens {
    Ident(Ident),
    Number(Number),
    String(LitString),
}

fn main() {}
