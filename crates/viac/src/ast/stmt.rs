/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{control::Control, decl::Decl, expr::Expr, macros::ast};

ast! {
    Stmt {
        Decl(Decl),
        Control(Control),
        Expr(Expr),
    }
}
