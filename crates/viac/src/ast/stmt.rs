/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::control::Control;
use super::decl::Decl;
use super::expr::Expr;
use super::macros::ast;

ast! {
    pub enum Stmt {
        Decl(Decl),
        Control(Control),
        Expr(Expr),
    }
}
