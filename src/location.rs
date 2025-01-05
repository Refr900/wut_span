use crate::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub line: u32,
    pub column: u32,
    pub len: u32,
}

impl Location {
    #[inline]
    pub const fn new(line: u32, column: u32, len: u32) -> Self {
        Self { line, column, len }
    }
}

impl Location {
    /// # Panics
    /// Panics if source does not contain span.
    #[inline]
    #[track_caller]
    pub fn from_span(source: &str, span: Span) -> Self {
        match Self::try_from_span(source, span) {
            Some(location) => location,
            None => panic!("Location::from_span(source, span): source does not contain span!"),
        }
    }

    #[inline]
    pub fn try_from_span(source: &str, span: Span) -> Option<Self> {
        if span.end() > source.len() {
            return None;
        }
        // SAFETY: we are check that span included in source
        Some(unsafe { Self::from_span_unchecked(source, span) })
    }

    /// # Unsafe
    /// Unsafe if source does not contain span.
    pub unsafe fn from_span_unchecked(source: &str, span: Span) -> Self {
        debug_assert!(
            span.end() <= source.len(),
            "Location::from_span_unchecked(source, span): source does not contain span!"
        );
        let mut line = 1;
        let mut column = 1;

        let before_span = &source[..span.start()];
        for char in before_span.chars() {
            if char == '\n' {
                line += 1;
                column = 0;
            }
            column += 1;
        }

        let lexeme = &source[span.as_range()];
        let len = lexeme.chars().count();
        Self::new(line, column, len as u32)
    }
}

impl Location {
    #[inline(always)]
    pub fn try_to_span(&self, source: &str) -> Option<Span> {
        Span::try_from_location(source, *self)
    }

    /// # Panics
    /// Panics if source does not contain location.
    #[inline(always)]
    #[track_caller]
    pub fn to_span(&self, source: &str) -> Span {
        Span::from_location(source, *self)
    }
}

impl core::fmt::Display for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)?;
        if f.alternate() {
            write!(f, ":{}", self.len)?;
        }
        Ok(())
    }
}
