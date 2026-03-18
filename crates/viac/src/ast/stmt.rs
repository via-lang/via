/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{expr::Expr, ty::Ty};
use crate::source::SourceSpan;

#[derive(Debug)]
pub enum StmtKind {
    Let {
        ident: String,
        ty: Option<Box<Ty>>,
        expr: Box<Expr>,
    },
    Discard(Expr),
    Consume(Expr),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: SourceSpan,
}
