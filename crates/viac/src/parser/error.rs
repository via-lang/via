/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

#![allow(unused_assignments)]

use std::string::String;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::clinic::PrettyVec;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("unexpected end of file")]
    #[diagnostic(code(syn::unexp::eof))]
    UnexpectedEndOfFile {},

    #[error("unexpected token '{got}'")]
    #[diagnostic(code(syn::unexp::token))]
    UnexpectedToken {
        #[label("expected one of {expected} here")]
        span: SourceSpan,

        expected: PrettyVec<&'static str>,
        got: String,
    },

    #[error("unterminated string literal")]
    #[diagnostic(code(syn::unterm::str_lit))]
    UnterminatedStringLiteral {
        #[label("here")]
        string: SourceSpan,

        #[label("missing closing `\"` (quote) here")]
        quote: SourceSpan,
    },

    #[error("`raise` clause may not appear in this context")]
    #[diagnostic(
        code(syn::unexp::raise),
        help("express union types as `T | E` instead of `T raise E`")
    )]
    UnexpectedRaiseClause {
        #[label("here")]
        span: SourceSpan,
    },

    #[error("`self` parameter may not appear twice")]
    #[diagnostic(code(syn::unexp::self_parm), help("remove parameter"))]
    UnexpectedSelfParam {
        #[label("here")]
        span: SourceSpan,
    },
}
