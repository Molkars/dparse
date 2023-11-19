mod location;
mod to_static;

pub use location::{Location, Span};
pub use to_static::ToStatic;

#[macro_export]
macro_rules! token {
    (
        $(
            $(#[$attr:meta])*
            $v:vis struct $name:ident($lit:literal)
        );* $(;)?
    ) => {
        $(
            $(#[$attr])*
            $v struct $name {
                pub span: $crate::Span,
            }

            impl $name {
                pub const LITERAL: &'static str = $lit;

                pub fn new(span: $crate::Span) -> Self {
                    Self {
                        span,
                    }
                }
            }

            impl<'src> $crate::Parse<'src> for $name {
                fn parse<P: $crate::Parser<'src> + ?Sized>(parser: &mut P) -> $crate::ParseResult<Self> {
                    parser.consume_str(Self::LITERAL)
                        .map(Self::new)
                        .ok_or_else(|| parser.mismatch(format!("expected {}", Self::LITERAL)))
                }
            }
        )*
    }
}