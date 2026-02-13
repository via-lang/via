/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::{ops::Range, slice::SliceIndex, sync::Arc};

use miette::{NamedSource, SourceCode};

#[derive(Debug, Clone)]
pub struct SourceBuf {
    inner: NamedSource<Arc<String>>,
}

impl<'a> SourceBuf {
    pub fn new(name: impl Into<String>, code: impl Into<String>) -> Self {
        Self {
            inner: NamedSource::new(name.into(), Arc::new(code.into())),
        }
    }

    pub fn as_str(&'a self) -> &'a str {
        self.inner.inner().as_str()
    }

    pub fn get<I>(&'a self, i: I) -> &'a str
    where
        I: SliceIndex<str, Output = str>,
    {
        self.inner
            .inner()
            .get(i)
            .expect("SourceBuf: attempt to read out-of-range slice index")
    }

    pub fn get_span(&'a self, span: &SourceSpan) -> &'a str {
        self.get(span.begin..span.end)
    }
}

impl SourceCode for SourceBuf {
    fn read_span<'a>(
        &'a self,
        span: &miette::SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        self.inner
            .read_span(span, context_lines_before, context_lines_after)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    pub begin: usize,
    pub end: usize,
}

impl SourceSpan {
    pub fn new(begin: usize, end: usize) -> Self {
        Self { begin, end }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.end - self.begin
    }

    pub fn merge(lhs: SourceSpan, rhs: SourceSpan) -> Self {
        Self {
            begin: lhs.begin,
            end: rhs.begin,
        }
    }
}

impl From<SourceSpan> for miette::SourceSpan {
    fn from(value: SourceSpan) -> Self {
        Self::new(
            miette::SourceOffset::from(value.begin),
            value.end - value.begin,
        )
    }
}

impl From<Range<usize>> for SourceSpan {
    fn from(value: Range<usize>) -> Self {
        Self::new(value.start, value.end)
    }
}
