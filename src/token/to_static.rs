use std::borrow::Cow;

pub trait ToStatic {
    type Output;

    fn to_static(&self) -> Self::Output;
}

impl<'a> ToStatic for Cow<'a, str> {
    type Output = Cow<'static, str>;

    fn to_static(&self) -> Self::Output {
        Cow::Owned(self.to_string())
    }
}

impl<'a> ToStatic for &'a str {
    type Output = Cow<'static, str>;

    fn to_static(&self) -> Self::Output {
        Cow::Owned(self.to_string())
    }
}

impl ToStatic for String {
    type Output = Cow<'static, str>;

    fn to_static(&self) -> Self::Output {
        Cow::Owned(self.clone())
    }
}

