#[macro_export]
macro_rules! punct {
    ($(pub struct $name:ident($lex:literal));* $(;)?) => {
        $(
            #[derive(Clone, Copy, PartialEq, Eq, Hash)]
            pub struct $name {
                span: $crate::parse::Span,
            }

            impl<'a> $crate::parse::Parse<'a> for $name {
                #[inline(always)]
                fn parse(input: &mut $crate::parse::ParseStream<'a>) -> Result<Self, $crate::parse::ParseError> {
                    input.take_while(|c| c.is_whitespace());
                    let spanner = input.spanner();
                    if input.take_str($lex) {
                        Ok(Self {
                            span: input.span(spanner)
                        })
                    } else {
                        Err(input.mismatch())
                    }
                }
            }

            impl $crate::token::Token for $name {
                #[inline(always)]
                fn span(&self) -> $crate::parse::Span {
                    self.span
                }

                #[inline(always)]
                fn content(&self) -> &'static str {
                    $lex
                }
            }

            impl ::std::fmt::Debug for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    write!(f, concat!(stringify!($name), "({:?} @ {:?})"), $lex, self.span)
                }
            }
        
            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                    f.write_str($lex)
                }
            }
        )*
    }
}
