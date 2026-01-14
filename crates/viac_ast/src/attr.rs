/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::expr::Expr;
use crate::macros::ast;
use crate::node::NodeRef;
use crate::ty::Ty;

ast! {
    pub enum Attr {
        Native {},
        Inline {},
        Distinct { ty: NodeRef<Ty> },
        Assert { expr: NodeRef<Ty>, out: Option<NodeRef<Expr>> },
    }
}
