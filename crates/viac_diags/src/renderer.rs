/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::diag::Diag;
use crate::diag::HeaderInfo;
use std::error::Error;
use std::io::{Error as IOError, Result as IOResult, Write};
use std::rc::Rc;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use viac_source::Source;
use viac_source::span;
use viac_source::span::Span;

pub trait Renderer {
    type E: Error;
    fn render(&mut self, diag: &Diag) -> Result<(), <Self as Renderer>::E>;
}

#[derive(Debug)]
pub struct TermRenderer {
    src: Rc<Source>,
    out: StandardStream,
}

impl TermRenderer {
    pub fn new(src: &Rc<Source>, stream: Option<StandardStream>) -> Self {
        Self {
            src: src.clone(),
            out: stream.unwrap_or_else(|| StandardStream::stdout(ColorChoice::Auto)),
        }
    }

    fn render_context(&mut self, diag: &Diag) -> IOResult<()> {
        for ctxt in &diag.context {
            self.out.set_color(ColorSpec::new().set_dimmed(true))?;
            writeln!(self.out, "{ctxt}:")?;
            self.out.reset()?;
        }
        Ok(())
    }

    fn render_header(&mut self, diag: &Diag) -> IOResult<Color> {
        let HeaderInfo(color, text) = Into::<HeaderInfo>::into(diag.kind);

        self.out.set_color(
            ColorSpec::new()
                .set_fg(Some(color))
                .set_bold(true)
                .set_intense(true),
        )?;

        write!(self.out, "{text}:")?;
        self.out.reset()?;

        write!(self.out, " {}", diag.message)?;

        if let Some(span) = diag.location {
            self.out.set_color(ColorSpec::new().set_dimmed(true))?;
            write!(self.out, " at ")?;

            self.out.set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::Cyan))
                    .set_dimmed(false)
                    .set_bold(true),
            )?;

            let loc = self.src.line_col(span.begin);
            writeln!(
                self.out,
                "[{}:{}:{}]",
                // TODO: actually display file information here
                "<file error>",
                loc.line + 1,
                loc.column
            )?;
            self.out.reset()?;
        } else {
            writeln!(self.out, "")?;
        }
        Ok(color)
    }

    fn write_source_line(
        &mut self,
        width: usize,
        line_no: u32,
        highlight: Option<(Span, bool)>,
        color: Color,
    ) -> IOResult<()> {
        self.out.set_color(ColorSpec::new().set_dimmed(true))?;
        write!(self.out, " {:width$} | ", line_no + 1, width = width)?;

        let text = self.src.line(line_no);
        match highlight {
            None => {
                writeln!(self.out, "{text}")?;
            }
            Some((span, builtin_highlight)) => {
                let start = (span.begin as usize).min(text.len());
                let end = (span.end as usize).min(text.len()).max(start);

                let (pre, rest) = text.split_at(start);
                let (mid, post) = rest.split_at(end - start);

                write!(self.out, "{pre}")?;
                self.out.set_color(
                    ColorSpec::new()
                        .set_fg(Some(color))
                        .set_underline(builtin_highlight)
                        .set_dimmed(true)
                        .set_bold(true),
                )?;

                write!(self.out, "{mid}")?;
                self.out.set_color(ColorSpec::new().set_dimmed(true))?;

                writeln!(self.out, "{post}")?;
            }
        }

        self.out.reset()?;
        Ok(())
    }

    fn write_highlight(
        &mut self,
        width: usize,
        col_start: usize,
        len: usize,
        color: Color,
    ) -> IOResult<()> {
        self.out.set_color(ColorSpec::new().set_dimmed(true))?;

        write!(
            self.out,
            " {:width$} | {:col_start$}",
            "",
            "",
            width = width,
            col_start = col_start
        )?;

        self.out
            .set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
        writeln!(self.out, "{:^<len$}", "", len = len)?;

        self.out.reset()?;
        Ok(())
    }

    fn render_span(&mut self, span: Span, color: Color) -> IOResult<()> {
        let (begin, end) = self.src.span_line_col(span);
        let max_line = self.src.line_count();

        let mut start_line = begin.line.min(max_line);
        let mut end_line = end.line.min(max_line);

        if start_line > end_line {
            std::mem::swap(&mut start_line, &mut end_line);
        }

        let start_line = begin.line;
        let end_line = end.line;
        let width = (end_line + 1).ilog10() as usize + 1;

        let context_before = (start_line > 1).then(|| start_line - 1);
        let context_after = (context_before.is_none() && end_line + 1 <= self.src.line_count())
            .then(|| end_line + 1);

        if let Some(line_no) = context_before {
            self.write_source_line(width, line_no, None, color)?;
        }

        for line_no in start_line..=end_line {
            let text = self.src.line(line_no);
            let fancy = self.out.supports_color();
            let highlight = if start_line == end_line {
                Some((span![begin.column, end.column], fancy))
            } else if line_no == start_line {
                Some((span![begin.column, text.len() as u32], fancy))
            } else if line_no == end_line {
                Some((span![0, end.column], fancy))
            } else {
                Some((span![0, text.len() as u32], fancy))
            };

            self.write_source_line(width, line_no, highlight, color)?;

            if let Some((span, _)) = highlight
                && !fancy
            {
                self.write_highlight(
                    width,
                    span.begin as usize,
                    (span.end - span.begin).max(1) as usize,
                    color,
                )?;
            }
        }

        if let Some(line_no) = context_after {
            self.write_source_line(width, line_no, None, color)?;
        }
        Ok(())
    }
}

impl Renderer for TermRenderer {
    type E = IOError;
    fn render(&mut self, diag: &Diag) -> IOResult<()> {
        self.render_context(diag)?;

        let color = self.render_header(diag)?;
        if let Some(span) = diag.location {
            self.render_span(span, color)?;
        }

        self.out.reset()?;
        self.out.flush()?;
        Ok(())
    }
}
