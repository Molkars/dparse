#[macro_export]
macro_rules! ident {
    (pub struct $name:ident<$l:lifetime>($is_first:expr; $is_ident:expr)) => {

        #[derive(Debug, Clone)]
        pub struct $name<$l> {
            content: &$l str,
            span: $crate::parse::Span,
        }

        impl<$l> $name<$l> {
            pub fn is_valid_first_char(c: &char) -> bool {
                fn is<F: std::ops::Fn(char) -> bool>(f: F) -> F { f }
                let f = is($is_first);
                f(*c)
            }

            pub fn is_valid_char(c: &char) -> bool {
                fn is<F: std::ops::Fn(char) -> bool>(f: F) -> F { f }
                let f = is($is_ident);
                f(*c)
            }
        }

        impl<$l> $crate::parse::Parse<$l> for $name<$l> {
            fn parse(input: &mut $crate::parse::ParseStream<$l>) -> Result<Self, $crate::parse::ParseError> {
                input.take_while(|c| c.is_whitespace());

                if input.peek_char().filter(|c| Self::is_valid_first_char(c)).is_none() {
                    return Err(input.mismatch());
                }

                let spanner = input.spanner();
                let content = input.take_while(|c| Self::is_valid_char(&c));

                if $name::KEYWORDS.contains(&content) {
                    return Err(input.mismatch());
                }

                Ok(Self {
                    content,
                    span: input.span(spanner),
                })
            }
        }

        impl<$l> $crate::token::Token for $name<$l> {
            fn span(&self) -> $crate::parse::Span {
                self.span
            }

            fn content(&self) -> &$l str {
                self.content
            }
        }

        impl ::std::fmt::Display for $name<'_> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(self.content)
            }
        }
    };
    (pub struct $name:ident<$l:lifetime> for $parent:ident where $f:expr) => {
        pub struct $name<$l> {
            inner: $parent<$l>,
        }

        impl $name<'_> {
            pub fn check_str(input: impl core::convert::AsRef<str>) -> bool {
                fn is<F: core::ops::Fn(&str) -> bool>(f: F) -> F { f }
                let f = is($f);
                f(input.as_ref())
            }
        }

        impl<$l> core::ops::Deref for $name<$l> {
            type Target = $parent<$l>;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl<$l> core::ops::DerefMut for $name<$l> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl<$l> core::convert::TryFrom<$parent<$l>> for $name<$l> {
            type Error = $parent<$l>;

            fn try_from(inner: $parent<$l>) -> Result<Self, Self::Error> {
                if Self::check_str(inner.content) {
                    Ok(Self { inner })
                } else {
                    Err(inner)
                }
            }
        }

        impl<$l> $crate::parse::Parse<$l> for $name<$l> {
            fn parse(input: &mut $crate::parse::ParseStream<$l>) -> Result<Self, $crate::parse::ParseError> {
                let inner: $parent = <$parent as $crate::parse::Parse>::parse(input)?;
                <Self as core::convert::TryFrom<$parent>>::try_from(inner).map_err(|_| input.mismatch())
            }
        }

        impl $crate::token::Token for $name<'_> {
            fn content(&self) -> &str {
                self.inner.content
            }

            fn span(&self) -> $crate::parse::Span {
                self.inner.span
            }
        }
    }
}

#[macro_export]
macro_rules! keywords {
    (ident $ident:ident; $(pub struct $name:ident($lex:literal));* $(;)?) => {
        impl $ident<'static> {
            pub const KEYWORDS: &'static [&'static str] = &[
                $($lex),*
            ];
        }

        $(
            #[derive(Debug, Clone, Copy)]
            pub struct $name {
                span: $crate::parse::Span,
            }

            impl $crate::parse::Parse<'_> for $name {
                #[inline]
                fn parse(input: &mut $crate::parse::ParseStream<'_>) -> Result<$name, $crate::parse::ParseError> {
                    input.take_while(|c| c.is_whitespace());

                    if input.peek_char().filter(|c| $ident::is_valid_first_char(c)).is_none() {
                        return Err(input.mismatch());
                    }

                    let spanner = input.spanner();
                    let content = input.take_while(|c| $ident::is_valid_char(&c));
                    if content == $lex {
                        Ok(Self { span: input.span(spanner) })
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
        )*
    };
}

#[test]
fn test_ident() {
    ident! {
        pub struct Ident<'a>(
            |c| c.is_alphabetic() || c == '_';
            |c| c.is_alphanumeric() || c == '_'
        )
    }

    ident! {
        pub struct Underscore<'a> for Ident where |s| s == "_"
    }

    keywords! {
        ident Ident;

        pub struct Use("use");
        pub struct Let("let");

        pub struct Fn("fn");
        pub struct Struct("struct");
        pub struct Enum("enum");

        pub struct Mut("mut");
        pub struct Const("const");

        pub struct Return("return");
        pub struct Expect("expect");
        pub struct Unless("unless");
        pub struct Match("match");
        pub struct Else("else");
        pub struct If("if");
        pub struct Elif("elif");
        pub struct Loop("loop");
        pub struct For("for");
        pub struct While("while");
    }

    fn parse<'a, T: crate::parse::Parse<'a>>(input: &'a str) -> Option<T> {
        let mut input = crate::parse::ParseStream::<'a>::new(input);
        T::parse(&mut input).ok()
    }

    assert!(parse::<Ident>("abc")
        .filter(|c| c.content == "abc")
        .is_some());
    assert!(parse::<Ident>("_abc")
        .filter(|c| c.content == "_abc")
        .is_some());
    assert!(parse::<Ident>("_1abc")
        .filter(|c| c.content == "_1abc")
        .is_some());
    assert!(parse::<Ident>("_").filter(|c| c.content == "_").is_some());
    assert!(parse::<Underscore>("_")
        .filter(|c| c.content == "_")
        .is_some());
    assert!(parse::<Ident>("3_1abc").is_none());

    assert!(parse::<Ident>(" \t \n \r abc")
        .filter(|c| c.content == "abc")
        .is_some());
    assert!(parse::<Ident>(" \t \n \r _abc")
        .filter(|c| c.content == "_abc")
        .is_some());
    assert!(parse::<Ident>(" \t \n \r _1abc")
        .filter(|c| c.content == "_1abc")
        .is_some());
    assert!(parse::<Ident>(" \t \n \r _")
        .filter(|c| c.content == "_")
        .is_some());
    assert!(parse::<Underscore>(" \t \n \r _")
        .filter(|c| c.content == "_")
        .is_some());
    assert!(parse::<Ident>(" \t \n \r 3_1abc").is_none());
}
