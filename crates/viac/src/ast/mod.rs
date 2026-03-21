/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod expr;
pub mod stmt;
pub mod ty;

use via_macros::Arena;

use crate::node::{NodeId, NodeStore};

use expr::Expr;
use stmt::Stmt;
use ty::Ty;

#[derive(Arena, Debug, Default)]
pub struct Tree {
    #[arena]
    stmt: Vec<Stmt>,
    #[arena]
    expr: Vec<Expr>,
    #[arena]
    ty: Vec<Ty>,
    pub roots: Vec<NodeId<Stmt>>,
}
