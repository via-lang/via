/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::source::SourceSpan;

#[derive(Debug)]
pub enum PlaceKind {
    Symbol(String),
}

#[derive(Debug)]
pub struct Place {
    pub kind: PlaceKind,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum ExprKind {
    None,
    True,
    False,
    Integer(u128),
    Float(f64),
    Read(Place),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: SourceSpan,
}
