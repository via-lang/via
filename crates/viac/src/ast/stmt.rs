/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::source::SourceSpan;

use super::expr::Expr;

#[derive(Debug)]
pub enum StmtKind {
    Discard(Expr),
    Consume(Expr),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: SourceSpan,
}
