/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{place::Place, value::Value};
use crate::compiler::source::Span;

#[derive(Debug)]
pub enum Expr {
    Place(Place),
    Value(Value),
}

impl Expr {
    pub fn span(&self) -> &Span {
        match self {
            Self::Place(place) => place.span(),
            Self::Value(value) => value.span(),
        }
    }
}
