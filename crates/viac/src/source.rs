use std::{ops::Range, slice::SliceIndex};

#[derive(Debug, Clone)]
pub struct SourceBuf {
    name: String,
    inner: String,
}

impl<'a> SourceBuf {
    pub fn new(name: impl Into<String>, code: impl Into<String>) -> Self {
        let name = name.into();
        let inner = code.into();

        // This should never happen as a 4 GiB file is total madness, but we still check just in case
        assert!(inner.len() < u32::MAX as usize, "File too large");

        Self { name, inner }
    }

    pub fn name(&'a self) -> &'a str {
        self.name.as_str()
    }

    pub fn as_str(&'a self) -> &'a str {
        self.inner.as_str()
    }

    pub fn get<I>(&'a self, i: I) -> &'a str
    where
        I: SliceIndex<str, Output = str>,
    {
        self.inner
            .get(i)
            .expect("SourceBuf: attempt to read out-of-range slice index")
    }

    pub fn read_span(&'a self, span: &SourceSpan) -> &'a str {
        self.get(span.begin as usize..span.end as usize)
    }

    pub fn get_line_col(&self, offset: u32) -> (u32, u32) {
        assert!(offset <= self.inner.len() as u32, "offset out of bounds");

        let mut line = 1;
        let mut col = 1;

        for (i, ch) in self.inner.char_indices() {
            if i as u32 >= offset {
                break;
            }

            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }

        (line, col)
    }

    pub fn get_line(&self, line_number: u32) -> Option<&str> {
        if line_number == 0 {
            return None;
        }

        let mut current_line = 1;
        let mut start = 0;

        for (i, ch) in self.inner.char_indices() {
            if ch == '\n' {
                if current_line == line_number {
                    return Some(&self.inner[start..i]);
                }

                current_line += 1;
                start = i + 1;
            }
        }

        (current_line == line_number).then_some(&self.inner[start..])
    }

    pub fn get_line_count(&self) -> u32 {
        if self.inner.is_empty() {
            return 0;
        }

        let count = self.inner.chars().filter(|&c| c == '\n').count() as u32;

        if self.inner.ends_with('\n') {
            count
        } else {
            count + 1
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    pub begin: u32,
    pub end: u32,
}

impl SourceSpan {
    pub fn new(begin: u32, end: u32) -> Self {
        Self { begin, end }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> u32 {
        self.end - self.begin
    }

    pub fn merge(lhs: SourceSpan, rhs: SourceSpan) -> Self {
        Self {
            begin: lhs.begin,
            end: rhs.begin,
        }
    }
}

impl From<Range<u32>> for SourceSpan {
    fn from(value: Range<u32>) -> Self {
        Self::new(value.start, value.end)
    }
}
