/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::macros::ast;
use crate::place::Place;
use crate::value::Value;

ast! {
    pub enum Expr {
        Place(Place),
        Value(Value),
    }
}
