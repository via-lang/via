/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::io::{self, Write};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use super::{
    diagnostic::{Report, Severity},
    renderer::Renderer,
};
use crate::{
    module::{Module, context::ModuleContext},
    source::{SourceBuf, SourceSpan},
};

#[derive(Debug)]
pub struct TermRenderer<'mcx> {
    mcx: &'mcx ModuleContext,
    out: StandardStream,
}

impl<'mcx> TermRenderer<'mcx> {
    pub fn new(mcx: &'mcx ModuleContext, out: Option<StandardStream>) -> Self {
        Self {
            mcx,
            out: out.unwrap_or(StandardStream::stdout(ColorChoice::Always)),
        }
    }

    fn render_header(&mut self, module: &dyn Module, report: &Report) -> io::Result<Color> {
        use Severity::*;

        let (color, header) = match report.severity {
            Info => (Color::Blue, "info"),
            Warning => (Color::Yellow, "warning"),
            Error => (Color::Red, "error"),
        };

        self.out.set_color(
            ColorSpec::new()
                .set_fg(Some(color))
                .set_bold(true)
                .set_intense(true),
        )?;

        write!(self.out, "{header}")?;

        if let Some(code) = report.code {
            write!(self.out, "[{code}]")?;
        }

        write!(self.out, ": ")?;

        self.out.reset()?;

        write!(self.out, "{}", report.message)?;

        if let Some(span) = report.span {
            self.out.set_color(ColorSpec::new().set_dimmed(true))?;

            write!(self.out, " at ")?;

            self.out.set_color(
                ColorSpec::new()
                    .set_fg(Some(Color::Cyan))
                    .set_dimmed(false)
                    .set_bold(true),
            )?;

            writeln!(self.out, "{}", module.get_trace(span))?;

            self.out.reset()?;
        } else {
            writeln!(self.out)?;
        }

        Ok(color)
    }

    fn write_source_line(
        &mut self,
        source: &SourceBuf,
        width: usize,
        line_no: u32,
        highlight: Option<(usize, usize)>,
        color: Option<Color>,
    ) -> io::Result<()> {
        self.out.set_color(ColorSpec::new().set_dimmed(true))?;

        write!(self.out, " {:width$} | ", line_no + 1, width = width)?;

        let text = source.get_line(line_no).expect("line should be valid here");

        match highlight {
            None => {
                writeln!(self.out, "{text}")?;
            }
            Some((start, end)) => {
                let start = start.min(text.len());
                let end = end.min(text.len()).max(start);

                let (pre, rest) = text.split_at(start);
                let (mid, post) = rest.split_at(end - start);

                write!(self.out, "{pre}")?;

                if let Some(color) = color {
                    self.out
                        .set_color(ColorSpec::new().set_fg(Some(color)).set_dimmed(true))?;
                }

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
        color: Option<Color>,
    ) -> io::Result<()> {
        self.out.set_color(ColorSpec::new().set_dimmed(true))?;

        write!(
            self.out,
            " {:width$} | {:col_start$}",
            "",
            "",
            width = width,
            col_start = col_start
        )?;

        self.out.reset()?;

        if let Some(color) = color {
            self.out
                .set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))?;
        }

        writeln!(self.out, "{:^<len$}", "", len = len)?;

        self.out.reset()?;
        Ok(())
    }

    fn render_span(
        &mut self,
        source: &SourceBuf,
        span: SourceSpan,
        color: Option<Color>,
    ) -> io::Result<()> {
        let max_line = source.get_line_count();
        let (begin, end) = (
            source.get_line_col(span.begin),
            source.get_line_col(span.end),
        );

        let mut start_line = begin.0.min(max_line);
        let mut end_line = end.0.min(max_line);

        if start_line > end_line {
            std::mem::swap(&mut start_line, &mut end_line);
        }

        let start_line = begin.0;
        let end_line = end.0;
        let width = (end_line + 1).ilog10() as usize + 1;

        let context_before = (start_line > 1).then(|| start_line - 1);
        let context_after = (context_before.is_none() && end_line < max_line).then(|| end_line + 1);

        if let Some(line_no) = context_before {
            self.write_source_line(source, width, line_no, None, None)?;
        }

        for line_no in start_line..=end_line {
            let text = source.get_line(line_no).expect("line should be valid here");

            let highlight = if start_line == end_line {
                Some((begin.1 as usize, end.1 as usize))
            } else if line_no == start_line {
                Some((begin.1 as usize, text.len()))
            } else if line_no == end_line {
                Some((0, end.1 as usize))
            } else {
                Some((0, text.len()))
            };

            self.write_source_line(source, width, line_no, highlight, color)?;

            if let Some((start, end)) = highlight {
                self.write_highlight(width, start, (end - start).max(1), color)?;
            }
        }

        if let Some(line_no) = context_after {
            self.write_source_line(source, width, line_no, None, None)?;
        }

        Ok(())
    }
}

impl<'mcx> Renderer for TermRenderer<'mcx> {
    type Error = std::io::Error;

    fn render(&mut self, report: Report) -> Result<(), Self::Error> {
        let module = self.mcx.get(report.origin).unwrap();
        let color = self.render_header(module, &report)?;

        if let Some(span) = report.span
            && let Some(source) = module.source()
        {
            self.render_span(source, span.clone(), Some(color))?;
        }

        self.out.reset()?;
        self.out.flush()?;
        Ok(())
    }
}
