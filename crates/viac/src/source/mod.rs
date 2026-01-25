/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod span;

use span::{Span, span};
use std::rc::Rc;

#[derive(Debug)]
pub struct Source {
    pub text: String,
    line_starts: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineCol {
    pub line: u32,
    pub column: u32,
}

impl Source {
    pub fn new(src: String) -> Rc<Self> {
        let mut line_starts = Vec::new();
        line_starts.push(0);

        for (i, b) in src.bytes().enumerate() {
            if b == b'\n' {
                line_starts.push((i + 1) as u32);
            }
        }

        Rc::new(Self {
            text: src,
            line_starts,
        })
    }

    pub fn end_span(&self) -> Span {
        let len = self.text.len() as u32;
        span![len, len]
    }

    pub fn slice(&self, span: Span) -> &str {
        let begin = span.begin as usize;
        let end = span.end as usize;
        &self.text[begin..end]
    }

    pub fn span_of(&self, sub: &str) -> Span {
        let src_start = self.text.as_ptr() as usize;
        let sub_start = sub.as_ptr() as usize;

        assert!(
            sub_start >= src_start && sub_start + sub.len() <= src_start + self.text.len(),
            "substring does not belong to source"
        );

        let begin = sub_start - src_start;
        let end = begin + sub.len();

        span![begin as u32, end as u32]
    }

    pub fn span_of_range(&self, begin: &str, end: &str) -> Span {
        span![self.span_of(begin).begin, self.span_of(end).end]
    }

    pub fn line_col(&self, pos: u32) -> LineCol {
        debug_assert!(pos as usize <= self.text.len());

        let line = match self.line_starts.binary_search(&pos) {
            Ok(i) => i as u32,
            Err(i) => (i - 1) as u32,
        };

        let column = pos - self.line_starts[line as usize];
        LineCol { line, column }
    }

    pub fn span_line_col(&self, span: Span) -> (LineCol, LineCol) {
        (self.line_col(span.begin), self.line_col(span.end))
    }

    pub fn line(&self, line: u32) -> &str {
        let start = self.line_starts[line as usize] as usize;
        let end = self
            .line_starts
            .get(line as usize + 1)
            .map(|&e| e as usize)
            .unwrap_or(self.text.len());

        self.text[start..end].trim_end_matches('\n')
    }

    pub fn lines(&self, start: u32, end: u32) -> &str {
        debug_assert!(start <= end);

        let begin = self.line_starts[start as usize] as usize;
        let end = self
            .line_starts
            .get(end as usize + 1)
            .map(|&e| e as usize)
            .unwrap_or(self.text.len());

        &self.text[begin..end]
    }

    pub fn span_lines(&self, span: Span) -> &str {
        let start_line = self.line_col(span.begin).line;
        let end_line = self.line_col(span.end).line;
        self.lines(start_line, end_line)
    }

    pub fn line_count(&self) -> u32 {
        self.line_starts.len() as u32 - 1
    }
}
