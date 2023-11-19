use std::borrow::Cow;
use crate::{Span, token, ToStatic};

token! {
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Tilde("~");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Tick("`");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Bang("!");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct At("@");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Pound("#");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Dollar("$");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Percent("%");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Caret("^");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Ampersand("&");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Star("*");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct OpenParen("(");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct CloseParen(")");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Minus("-");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Underscore("_");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Plus("+");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Equals("=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct OpenBracket("[");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct CloseBracket("]");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct OpenBrace("{");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct CloseBrace("}");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Pipe("|");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Backslash("\\");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Colon(":");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct SemiColon(";");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Quote("\"");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Apostrophe("'");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Comma(",");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Period(".");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct LeftAngle("<");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct RightAngle(">");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Question("?");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Slash("/");

    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct TildeEquals("~=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct BangEquals("!=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct PercentEquals("%=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleCaret("^^");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct CaretEquals("^=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleAmpersand("&&");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct AmpersandEquals("&=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleStar("**");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct StarEquals("*=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Parens("()");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleMinus("--");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct MinusEquals("-=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoublePlus("++");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct PlusEquals("+=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct TripleEquals("===");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleEquals("==");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct Brackets("[]");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoublePipe("||");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct PipeEquals("|=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleColon("::");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct ColonEquals(":=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleLeftAngle("<<");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct LeftAngleEquals("<=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleRightAngle(">>");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct RightAngleEquals(">=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleSlash("//");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct SlashEquals("/=");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct DoubleQuestion("??");
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
	pub struct QuestionEquals("?=");
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct CIdent<'src> {
    ident: Cow<'src, str>,
    span: Span,
}

impl<'src> ToStatic for CIdent<'src> {
    type Output = CIdent<'static>;

    fn to_static(&self) -> Self::Output {
        CIdent {
            ident: self.ident.to_static(),
            span: self.span,
        }
    }
}
