use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
    pub index: usize,
}

impl Debug for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location({}, {}:{})", self.index, self.line, self.column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self {
            line: 1,
            column: 1,
            index: 0,
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

impl Location {
    pub fn new(line: usize, column: usize, index: usize) -> Self {
        Self {
            line,
            column,
            index,
        }
    }

    pub fn span(&self, other: Self) -> Result<Span, &'static str> {
        Span::new(*self, other)
    }
}

#[derive(Clone, Copy, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Span {
    pub start: Location,
    pub end: Location,
}

impl Debug for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Span({:?} through {:?})", self.start, self.end)
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl Span {
    pub fn new(start: Location, end: Location) -> Result<Self, &'static str> {
        if start.line < end.line {
            return Err("start line is less than end line");
        }

        if start.line == end.line && start.column >= end.column {
            return Err("start column is greater than or equal to end column");
        }

        if start.index >= end.index {
            return Err("start index is greater than or equal to end index");
        }

        Ok(Span { start, end })
    }

    pub fn len(&self) -> usize {
        self.end.index - self.start.index
    }

    pub fn is_empty(&self) -> bool {
        self.start.index == self.end.index
    }

    pub fn join(&self, other: Self) -> Self {
        let start = if self.start.index < other.start.index {
            self.start
        } else {
            other.start
        };

        let end = if self.end.index > other.end.index {
            self.end
        } else {
            other.end
        };

        Self { start, end }
    }
}
