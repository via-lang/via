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
use viac_source::span::Span;

pub trait Renderer {
    type Error: Error;

    fn default() -> Self;
    fn render(
        &mut self,
        src: Option<Rc<Source>>,
        diag: &Diag,
    ) -> Result<(), <Self as Renderer>::Error>;
}

#[derive(Debug)]
pub struct TermRenderer {
    stdout: StandardStream,
}

impl TermRenderer {
    pub fn new(choice: ColorChoice) -> Self {
        Self {
            stdout: StandardStream::stdout(choice),
        }
    }

    fn render_context(&mut self, diag: &Diag) -> IOResult<()> {
        for ctxt in &diag.context {
            self.stdout.set_color(ColorSpec::new().set_dimmed(true))?;
            writeln!(self.stdout, "{ctxt}:")?;
            self.stdout.reset()?;
        }
        Ok(())
    }

    fn render_header(&mut self, diag: &Diag) -> IOResult<Option<Color>> {
        let color = diag.kind.map(Into::<HeaderInfo>::into);

        if let Some(HeaderInfo(color, text)) = color {
            self.stdout.set_color(
                ColorSpec::new()
                    .set_fg(Some(color))
                    .set_bold(true)
                    .set_intense(true),
            )?;
            write!(self.stdout, "{text}")?;
            self.stdout.reset()?;
            writeln!(self.stdout, " {}", diag.message)?;
            Ok(Some(color))
        } else {
            writeln!(self.stdout, "{}", diag.message)?;
            Ok(None)
        }
    }

    fn write_source_line(
        &mut self,
        width: usize,
        line_no: u32,
        text: &str,
        highlight: Option<(usize, usize)>,
        color: Option<Color>,
    ) -> IOResult<()> {
        self.stdout.set_color(ColorSpec::new().set_dimmed(true))?;
        write!(self.stdout, " {line_no:width$} | ", width = width)?;

        match highlight {
            None => {
                writeln!(self.stdout, "{text}")?;
            }
            Some((start, end)) => {
                let start = start.min(text.len());
                let end = end.min(text.len()).max(start);

                let (pre, rest) = text.split_at(start);
                let (mid, post) = rest.split_at(end - start);

                write!(self.stdout, "{pre}")?;

                if let Some(color) = color {
                    self.stdout
                        .set_color(ColorSpec::new().set_fg(Some(color)).set_dimmed(true))?;
                }

                write!(self.stdout, "{mid}")?;
                self.stdout.set_color(ColorSpec::new().set_dimmed(true))?;
                writeln!(self.stdout, "{post}")?;
            }
        }

        self.stdout.reset()?;
        Ok(())
    }

    fn write_highlight(
        &mut self,
        width: usize,
        col_start: usize,
        len: usize,
        color: Option<Color>,
    ) -> IOResult<()> {
        self.stdout.set_color(ColorSpec::new().set_dimmed(true))?;

        write!(
            self.stdout,
            " {:width$} | {:col_start$}",
            "",
            "",
            width = width,
            col_start = col_start
        )?;

        self.stdout.reset()?;

        if let Some(color) = color {
            self.stdout
                .set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
        }

        writeln!(self.stdout, "{:^<len$}", "", len = len)?;

        self.stdout.reset()?;
        Ok(())
    }

    fn render_span(&mut self, source: &Source, span: Span, color: Option<Color>) -> IOResult<()> {
        let (begin, end) = source.span_line_col(span);
        let max_line = source.line_count();

        let mut start_line = begin.line.min(max_line);
        let mut end_line = end.line.min(max_line);

        if start_line > end_line {
            std::mem::swap(&mut start_line, &mut end_line);
        }

        let start_line = begin.line;
        let end_line = end.line;
        let width = end_line.ilog10() as usize + 1;

        let context_before = (start_line > 1).then(|| start_line - 1);
        let context_after =
            (context_before.is_none() && end_line + 1 <= source.line_count()).then(|| end_line + 1);

        if let Some(line_no) = context_before {
            self.write_source_line(width, line_no, source.line(line_no), None, None)?;
        }

        for line_no in start_line..=end_line {
            let text = source.line(line_no);
            let highlight = if start_line == end_line {
                Some((begin.column as usize, end.column as usize))
            } else if line_no == start_line {
                Some((begin.column as usize, text.len()))
            } else if line_no == end_line {
                Some((0, end.column as usize))
            } else {
                Some((0, text.len()))
            };

            self.write_source_line(width, line_no, text, highlight, color)?;
            if let Some((start, end)) = highlight {
                self.write_highlight(width, start, (end - start).max(1), color)?;
            }
        }

        if let Some(line_no) = context_after {
            self.write_source_line(width, line_no, source.line(line_no), None, None)?;
        }
        Ok(())
    }
}

impl Renderer for TermRenderer {
    type Error = IOError;

    fn default() -> Self {
        Self::new(ColorChoice::Auto)
    }

    fn render(&mut self, src: Option<Rc<Source>>, diag: &Diag) -> IOResult<()> {
        self.render_context(diag)?;

        let color = self.render_header(diag)?;
        if let Some(span) = diag.location {
            let source =
                src.expect("renderer must have source context to render diagnostic with location");
            self.render_span(source.as_ref(), span, color)?;
        }

        self.stdout.reset()?;
        Ok(())
    }
}
