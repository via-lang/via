/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::expr::ExprRef;

#[derive(Debug)]
pub enum Attr {
    // Use
    Strong,
    // Struct
    Public,
    Private,
    ReadOnly,
    // Control flow
    Fallthrough,
    // Meta
    Multi(Vec<Attr>),
}
