/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod builder;
pub mod error;
pub mod expr;
pub mod passes;
pub mod stmt;
pub mod ty;

use via_macros::Arena;

pub use builder::*;
pub use error::*;

use expr::Expr;
use stmt::Stmt;

use crate::node::{NodeId, NodeStore};

#[derive(Arena, Debug, Default)]
pub struct Hir {
    #[arena]
    expr: Vec<Expr>,
    #[arena]
    stmt: Vec<Stmt>,
    pub roots: Vec<NodeId<Stmt>>,
}
