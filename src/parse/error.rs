use std::collections::LinkedList;
use std::fmt::{Display, Formatter};
use crate::token::Span;

#[derive(Debug)]
pub struct ParseError {
    pub(super) mismatch: bool,
    pub(super) span: Span,
    pub(super) message: String,
    pub(super) context: LinkedList<(Span, String)>,
    #[cfg(feature = "backtrace")]
    pub(super) backtrace: std::backtrace::Backtrace,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "A parse error occurred.")?;
        writeln!(f, "  at {} : {}", self.span, self.message)?;
        for (span, message) in &self.context {
            writeln!(f, "{}: {}", span, message)?;
        }
        #[cfg(feature = "backtrace")]
        writeln!(f, "Backtrace:\n{}", self.backtrace)?;
        Ok(())
    }
}

impl ParseError {
    #[track_caller]
    pub fn mismatch(span: Span, message: impl Into<String>) -> Self {
        Self {
            mismatch: true,
            span,
            message: message.into(),
            context: LinkedList::new(),
            #[cfg(feature = "backtrace")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }

    #[track_caller]
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            mismatch: false,
            span,
            message: message.into(),
            context: LinkedList::new(),
            #[cfg(feature = "backtrace")]
            backtrace: std::backtrace::Backtrace::capture(),
        }
    }

    pub fn is_mismatch(&self) -> bool {
        self.mismatch
    }

    pub fn context(mut self, span: Span, message: impl Into<String>) -> Self {
        self.context.push_back((span, message.into()));
        self
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

pub trait ParseErrorWithContext {
    fn context(self, span: Span, message: impl Into<String>) -> Self;
}

impl<T> ParseErrorWithContext for Result<T, ParseError> {
    #[track_caller]
    fn context(self, span: Span, message: impl Into<String>) -> Self {
        self.map_err(|e| e.context(span, message))
    }
}