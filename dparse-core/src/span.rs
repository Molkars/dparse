use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub struct Location {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Location,
    pub len: usize,
}

impl Span {
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start.index..self.start.index + self.len
    }
}