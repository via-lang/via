/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

#![allow(unused_assignments)]

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::source::SourceBuf;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Diagnostic, Error, Debug)]
pub enum Error {
    #[error("expression result cannot be ignored")]
    #[diagnostic(
        code(sema::expr_ignored),
        help("ignoring this value may result in dead code or erroneous behavior")
    )]
    ExprIgnored {
        #[source_code]
        src: SourceBuf,

        #[label("this expression")]
        span: SourceSpan,
    },
}
