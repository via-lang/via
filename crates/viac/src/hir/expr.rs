/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::sema::ops::*;

#[derive(Debug)]
pub enum ExprKind {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
}
