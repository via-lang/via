/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::context::Context;
use escape_string::escape;
use std::fmt;
use std::rc::Rc;
use viac_diags::builder::Builder;
use viac_diags::diag::{Diag, IntoDiag};
use viac_diags::diag::{DiagKind, Note};
use viac_lexer::token::Token;
use viac_source::Source;

#[derive(Debug)]
pub struct ExpectedList(pub Vec<&'static str>);

impl From<Vec<&'static str>> for ExpectedList {
    fn from(value: Vec<&'static str>) -> Self {
        ExpectedList(value)
    }
}

impl fmt::Display for ExpectedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedEndOfFile,
    UnexpectedToken { exp: ExpectedList, got: Token },
    UnterminatedStringLiteral { tok: Token },
}

#[derive(Debug)]
pub struct Error {
    pub ctxts: Vec<Context>,
    pub kind: ErrorKind,
}

impl IntoDiag for Error {
    fn into_diag(self, src: &Rc<Source>) -> Diag {
        let mut b = Builder::new(&src, DiagKind::Error);

        for ctxt in &self.ctxts {
            b.context(format!("while parsing {ctxt}"));
        }

        match self.kind {
            ErrorKind::UnexpectedEndOfFile => {
                let end_span = b.src.end_span();
                b.message("unexpected end of file".to_string())
                    .location(end_span)
            }
            ErrorKind::UnexpectedToken { exp, got } => {
                // TODO: This shit does not actually need to allocate,
                // but the borrow checker is being a bitch
                let slice = b.src.slice(got.span).to_string();
                let text = escape(slice.as_str());
                b.message(format!(
                    "unexpected token '{}'",
                    if text.len() > 20 {
                        "<truncated>"
                    } else {
                        text.as_ref()
                    }
                ))
                .location(got.span)
                .note(Note::Note(format!("expected {}", exp)))
            }
            ErrorKind::UnterminatedStringLiteral { tok } => b
                .message("unterminated string literal".to_string())
                .location(tok.span),
        };
        b.build()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
