/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::node::{NodeId, NodeStore};

pub struct TypeCheck;

impl TypeCheck {
    fn check(&mut self, hir: &Hir, stmt: NodeId<Stmt>) -> Result<()> {
        match hir.get(stmt) {}
    }
}

impl Pass for TypeCheck {
    fn run(&mut self, builder: &mut HirBuilder, hir: &Hir) -> Result<()> {}
}
