use std::fmt::{Debug, Formatter};
use crate::parse::{Parse, ParseError, ParseStream};

pub struct OnLine<T>(pub T);

impl<'a, T: Parse<'a>> Parse<'a> for OnLine<T> {
    fn parse(input: &mut ParseStream<'a>) -> Result<Self, ParseError> {
        let span = input.spanner();

        input.take_while(|c| c.is_whitespace() && c != '\n');
        if let Some('\n') = input.peek_char() {
            input.reset(span);
            return Err(input.mismatch());
        }

        Ok(Self(input.parse()?))
    }
}

impl<T: Debug> Debug for OnLine<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        T::fmt(&self.0, f)
    }
}