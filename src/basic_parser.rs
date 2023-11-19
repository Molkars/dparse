use std::cell::Cell;
use std::rc::Rc;
use crate::{Location, Parser, Span};

#[derive(Clone)]
pub struct BasicParser<'src> {
    source: &'src str,
    cursor: Cell<(&'src str, Location)>,
    preserve_whitespace: Rc<Cell<bool>>,
}

pub struct BasicWhitespaceLock {
    preserve_whitespace: Rc<Cell<bool>>,
    value: bool,
}

impl Drop for BasicWhitespaceLock {
    fn drop(&mut self) {
        self.preserve_whitespace.set(self.value);
    }
}

impl<'src> BasicParser<'src> {
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            cursor: Cell::new((source, Location::default())),
            preserve_whitespace: Rc::new(Cell::new(false)),
        }
    }

    #[inline]
    fn cursor(&self) -> &'src str {
        self.cursor.get().0
    }

    #[inline]
    fn take_whitespace(&self) -> bool {
        if self.preserve_whitespace.get() {
            return false;
        }

        let (mut cursor, mut location) = self.cursor.get();
        let start_idx = location.index;

        let mut chars = cursor.chars()
            .take_while(|c| c.is_whitespace());
        while let Some(c) = chars.next() {
            Self::advance_char(c, &mut location);
        }
        if start_idx != location.index {
            cursor = &cursor[location.index - start_idx..];
            self.cursor.set((cursor, location));
            true
        } else {
            false
        }
    }

    fn advance_char(c: char, loc: &mut Location) {
        loc.index += 1;
        match c {
            '\n' => {
                loc.line += 1;
                loc.column = 0;
            }
            _ => loc.column += 1,
        }
    }
}

impl<'src> Parser<'src> for BasicParser<'src> {
    type Lock = BasicWhitespaceLock;
    fn source(&self) -> &'src str {
        self.source
    }

    fn fork(&self) -> Self where Self: Sized {
        self.clone()
    }

    #[inline]
    fn preserve_whitespace(&mut self) -> Self::Lock {
        self.preserve_whitespace.set(true);
        BasicWhitespaceLock {
            preserve_whitespace: self.preserve_whitespace.clone(),
            value: false,
        }
    }

    #[inline]
    fn ignore_whitespace(&mut self) -> Self::Lock {
        self.preserve_whitespace.set(false);
        BasicWhitespaceLock {
            preserve_whitespace: self.preserve_whitespace.clone(),
            value: true,
        }
    }

    #[inline]
    fn allows_whitespace(&self) -> bool {
        self.preserve_whitespace.get()
    }

    fn location(&self) -> Location {
        self.take_whitespace(); // this is a bit tricky but in order to do try-parse, we are optimizing
        self.cursor.get().1
    }

    #[inline]
    fn span(&self) -> Span {
        let loc = self.location();
        let mut end = loc;
        self.peek()
            .map(|c| {
                Self::advance_char(c, &mut end);
            })
            .unwrap_or_else(|| {
                end.index += 1;
                end.column += 1;
            });
        Span::new(loc, end)
            .expect("BasicParser::span: span failed")
    }

    fn set_location(&mut self, loc: Location) -> Result<(), Location> {
        let current_loc = self.location();

        #[cfg(debug_assertions)]
        {
            let mut temp_loc = Location::default();
            for c in self.source[..loc.index].chars() {
                Self::advance_char(c, &mut temp_loc);
            };
            debug_assert_eq!(temp_loc.line, loc.line, "line mismatch: {} != {}", temp_loc.line, loc.line);
            debug_assert_eq!(temp_loc.column, loc.column, "column mismatch: {} != {}", temp_loc.column, loc.column);
            debug_assert_eq!(temp_loc.index, loc.index, "index mismatch: {} != {}", temp_loc.index, loc.index);
        }

        if loc.index > self.source.len() {
            return Err(current_loc);
        }

        let cursor = &self.source[loc.index..];
        self.cursor.set((cursor, loc));
        Ok(())
    }

    #[inline]
    fn peek(&self) -> Option<char> {
        self.take_whitespace();
        self.cursor().chars().next()
    }

    #[inline]
    fn peek_by_n(&self, off: usize) -> Option<char> {
        self.take_whitespace();
        self.cursor().chars().nth(off)
    }

    #[inline]
    fn peek_n(&self, n: usize) -> Option<&'src str> {
        self.take_whitespace();

        let cursor = self.cursor();
        let (count, off) = cursor.chars()
            .take(n)
            .fold((0, 0), |(count, off), c| (count + 1, off + c.len_utf8()));
        if count != n {
            return None;
        }

        Some(&cursor[..off])
    }

    #[inline]
    fn peek_n_by_n(&self, n: usize, off: usize) -> Option<&'src str> {
        self.take_whitespace();

        let cursor = self.cursor();
        let (count, off) = cursor.chars()
            .skip(off)
            .take(n)
            .fold((0, 0), |(count, off), c| (count + 1, off + c.len_utf8()));
        if count != n {
            return None;
        }

        Some(&cursor[..off])
    }

    #[inline]
    fn consume(&mut self) -> Option<char> {
        let c = self.peek()?;
        let (mut cursor, mut location) = self.cursor.get();
        Self::advance_char(c, &mut location);
        cursor = &cursor[c.len_utf8()..];
        self.cursor.set((cursor, location));
        Some(c)
    }

    #[inline]
    fn consume_n(&mut self, n: usize) -> Option<&'src str> {
        self.peek_n(n).map(|s| {
            let (mut cursor, mut location) = self.cursor.get();
            cursor = &cursor[s.len()..];
            for c in s.chars() {
                Self::advance_char(c, &mut location);
            }
            self.cursor.set((cursor, location));
            s
        })
    }
}
