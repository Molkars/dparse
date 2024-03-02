use std::fmt::Debug;
use crate::{Lexer, LexResult};
use crate::span::Span;

#[derive(Debug, Clone)]
pub struct Token<Set> {
    pub span: Span,
    pub kind: TokenKind<Set>,
}

impl<Set> Token<Set> {
    pub fn punct(span: Span, lit: &'static str) -> Self {
        Token {
            span,
            kind: TokenKind::Punct(lit),
        }
    }

    pub fn custom(span: Span, kind: Set) -> Self {
        Token {
            span,
            kind: TokenKind::Custom(kind),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind<Set> {
    Punct(&'static str),
    Custom(Set),
}

pub trait TypedToken<Set>
    where
        Self: Debug + Clone,
        Self: Into<Token<Set>>,
        Self: TryFrom<Token<Set>, Error=Token<Set>>,
{
    fn span(&self) -> Span;
}

pub trait TokenSet: Sized + Clone + Debug {
    fn lex<L: Lexer>(lexer: &mut L) -> LexResult<Token<Self>>;
}

#[macro_export]
macro_rules! token_set {
    (
        $(#[$meta:meta])*
        $v:vis enum $n:ident {
            $($token_name:ident($token_type:ty)),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        $v enum $n {
            $($token_name,)+
        }

        impl dparse_core::TokenSet for $n {
            fn lex<L: dparse_core::Lexer>(lexer: &mut L) -> dparse_core::LexResult<dparse_core::Token<Self>> {
                $(
                    let item = <$token_type as dparse_core::Lex>::lex(lexer);
                    match item {
                        dparse_core::LexResult::Some(item) => {
                            let span = dparse_core::TypedToken::span(&item);
                            let token = dparse_core::Token::custom(span, $n::$token_name);
                            return dparse_core::LexResult::Some(token);
                        }
                        dparse_core::LexResult::Err(err) => {
                            return dparse_core::LexResult::Err(err);
                        }
                        dparse_core::LexResult::None => {}
                    };
                )+
                dparse_core::LexResult::None
            }
        }
    };
}
