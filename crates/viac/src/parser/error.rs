/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::context::Context;
use crate::diags::builder::Builder;
use crate::diags::{Diagnostic, IntoDiagnostic, Note, Severity};
use crate::lexer::token::Token;
use crate::source::{Source, span::Span};
use escape_string::escape;
use std::fmt;
use std::rc::Rc;
use via_proc_macros::DiagCode;

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

#[derive(DiagCode, Debug)]
#[diag(prefix = "E", start = 0)]
pub enum ErrorKind {
    UnexpectedEndOfFile,
    UnexpectedToken { exp: ExpectedList, got: Token },
    UnterminatedStringLiteral { tok: Token },
    DisallowedEffect { tok: Token },
    MultiplePostfixTry { tok: Token },
    MultiplePostfixAwait { tok: Token },
    MultiplePostfixOptional { tok: Token },
    SyntacticallyUnreachable { span: Span },
}

#[derive(Debug)]
pub struct Error {
    pub ctxts: Vec<Context>,
    pub kind: ErrorKind,
}

impl IntoDiagnostic for Error {
    fn into_diagnostic(self, src: &Rc<Source>) -> Diagnostic {
        let mut b = Builder::new(src, Severity::Error);
        b.code(self.kind.code());

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
                .note(Note::Help(
                    "terminate string by inserting `\"` where it is supposed to end".to_string(),
                ))
                .location(tok.span),
            ErrorKind::DisallowedEffect { tok } => b
                .message("`raise` clause may only appear in function return types".to_string())
                .note(Note::Help("remove the `raise` clause".to_string()))
                .location(tok.span),
            ErrorKind::MultiplePostfixTry { tok } => b
                .message("multiple postfix `?` (try) operators are not allowed".to_string())
                .location(tok.span)
                .note(Note::Help("remove the latter `?`".to_string())),
            ErrorKind::MultiplePostfixAwait { tok } => b
                .message("multiple postfix `await` operators are not allowed".to_string())
                .location(tok.span)
                .note(Note::Help("remove the latter `.await`".to_string())),
            ErrorKind::MultiplePostfixOptional { tok } => b
                .message("multiple postfix `?` qualifiers are not allowed".to_string())
                .note(Note::Help("remove the latter `?`".to_string()))
                .location(tok.span),
            ErrorKind::SyntacticallyUnreachable { span } => b
                .message("syntactically unreachable statement".to_string())
                .note(Note::Note(
                    "statement is located below unconditionally branching control statement"
                        .to_string(),
                ))
                .note(Note::Help("remove statement".to_string()))
                .location(span),
        };
        b.build()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
