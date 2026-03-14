/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::{
    clinic::{Diagnostic, Severity},
    source::SourceSpan,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile(SourceSpan),
    UnexpectedToken(SourceSpan),
    UnterminatedStringLiteral {
        literal: SourceSpan,
        quote: SourceSpan,
    },
}

impl Diagnostic for Error {
    fn severity(&self) -> Severity {
        Severity::Error
    }
}
