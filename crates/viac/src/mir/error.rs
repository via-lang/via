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

use crate::clinic::PrettyVec;

#[derive(Diagnostic, Error, Debug)]
pub enum Error {
    #[error("use of undefined symbol '{symbol}'")]
    #[diagnostic(code(sema::undef_sym), severity(Error))]
    UndefinedSymbol {
        #[label("here")]
        span: SourceSpan,
        symbol: String,
    },

    #[error("expression result cannot be ignored")]
    #[diagnostic(
        code(sema::expr_ignored),
        help("ignoring this value may result in dead code or erroneous behavior"),
        severity(Error)
    )]
    ExprIgnored {
        #[label("this expression")]
        span: SourceSpan,
    },

    #[error("unreachable statement")]
    #[diagnostic(
        code(sema::unreachable),
        help("statement is not reachable by control flow and will never execute"),
        severity(Warning)
    )]
    UnreachableStatement {
        #[label("this statement")]
        span: SourceSpan,
    },

    #[error("rogue control statement")]
    #[diagnostic(
        code(sema::rogue_ctrl),
        help("statement may only appear in {allowed}"),
        severity(Error)
    )]
    RogueControlStatement {
        #[label("this statement")]
        span: SourceSpan,
        allowed: PrettyVec<String>,
    },
}
