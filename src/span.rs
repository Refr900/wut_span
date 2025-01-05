use std::ops::Range;

use super::Location;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub const ZERO: Self = Self::dot(0);

    /// # Panics
    /// Panics if the start is greater than the end
    #[inline]
    #[track_caller]
    pub const fn new(start: usize, end: usize) -> Self {
        assert!(
            start <= end,
            "Span::new(start, end): start must be less than or equal to end"
        );
        Self { start, end }
    }

    #[inline]
    pub const fn with_len(start: usize, len: usize) -> Self {
        Self {
            start,
            end: start + len,
        }
    }

    #[inline]
    pub const fn dot(place: usize) -> Self {
        Self {
            start: place,
            end: place,
        }
    }
}

impl Span {
    /// # Panics
    /// Panics if the source not include the location
    #[inline]
    #[track_caller]
    pub fn from_location(source: &str, location: Location) -> Self {
        match Self::try_from_location(source, location) {
            Some(span) => span,
            None => panic!(
                "Span::from_location(source, location): the source not include the location!"
            ),
        }
    }

    pub fn try_from_location(source: &str, location: Location) -> Option<Self> {
        let mut start = 0;
        // skip lines
        let mut lines = source.lines();
        if location.line > 1 {
            for _ in 0..location.line - 1 {
                match lines.next() {
                    Some(line) => {
                        start += line.len();
                        // add `\n`
                        start += 1;
                    }
                    None => return None,
                }
            }
            // we added one unnecessary newline, needed remove
            start -= 1;
        }

        let mut chars = source[start..].chars();
        // skip until start
        for _ in 0..location.column {
            match chars.next() {
                Some(char) => start += char.len_utf8(),
                None => return None,
            }
        }

        // calculate len
        let mut len = 0;
        if location.len != 0 {
            for _ in 0..location.len {
                match chars.next() {
                    Some(char) => len += char.len_utf8(),
                    None => return None,
                }
            }
        }

        Some(Self::with_len(start, len))
    }
}

impl Span {
    #[inline]
    pub const fn is_contained_in(&self, source: &str) -> bool {
        self.end <= source.len()
    }
}

impl Span {
    #[inline]
    pub const fn as_range(&self) -> Range<usize> {
        self.start..self.end
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }
}

impl Span {
    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    #[inline]
    pub const fn end(&self) -> usize {
        self.end
    }
}

impl Span {
    /// # Panics
    /// Panics if source does not contain span.
    #[inline]
    #[track_caller]
    pub fn to_location(self, source: &str) -> Location {
        assert!(
            self.end <= source.len(),
            "Span::to_location(source): source does not contain span!"
        );
        // SAFETY: we are check that span included in source
        unsafe { Location::from_span_unchecked(source, self) }
    }
}
