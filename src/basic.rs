use crate::{ident, keywords, punct};

ident! {
    pub struct CIdent<'a>(
        |c| c.is_alphabetic() || c == '_';
        |c| c.is_alphanumeric() || c == '_'
    )
}

keywords! {
    for CIdent;
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