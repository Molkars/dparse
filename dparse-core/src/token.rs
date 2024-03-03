use crate::Span;

#[derive(Debug, Clone, Copy)]
pub struct Token<Kind> {
    pub span: Span,
    pub kind: Kind,
}