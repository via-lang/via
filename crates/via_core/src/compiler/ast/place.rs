/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::expr::ExprRef;
use crate::compiler::lexer::token::Token;

#[derive(Debug)]
pub struct Symbol {
    pub token: Token,
}

#[derive(Debug)]
pub struct Dynamic {
    pub expr: ExprRef,
    pub token: ExprRef,
}

#[derive(Debug)]
pub struct Static {
    pub expr: ExprRef,
    pub token: Token,
}

#[derive(Debug)]
pub struct Subscript {
    pub expr: ExprRef,
    pub index: ExprRef,
}

#[derive(Debug)]
pub enum Place {
    Symbol(Symbol),
    Dynamic(Dynamic),
    Static(Static),
    Subscript(Subscript),
}
