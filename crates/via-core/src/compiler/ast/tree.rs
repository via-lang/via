/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    expr::{Expr, ExprRef},
    stmt::{Stmt, StmtRef},
    typ::{Type, TypeRef},
};
use bumpalo::collections::Vec as BumpVec;

#[derive(Debug)]
pub struct Tree<'m> {
    expr_arena: BumpVec<'m, Expr>,
    type_arena: BumpVec<'m, Type>,
    stmt_arena: BumpVec<'m, Stmt>,
}

impl<'m> Tree<'m> {
    pub fn expr(&mut self, e: Expr) -> ExprRef {
        self.expr_arena.push(e);
        ExprRef {
            0: (self.expr_arena.len() - 1) as u32,
        }
    }

    pub fn typ(&mut self, t: Type) -> TypeRef {
        self.type_arena.push(t);
        TypeRef {
            0: (self.type_arena.len() - 1) as u32,
        }
    }

    pub fn stmt(&mut self, s: Stmt) -> StmtRef {
        self.stmt_arena.push(s);
        StmtRef {
            0: (self.stmt_arena.len() - 1) as u32,
        }
    }

    pub fn to_expr(&'m self, e: ExprRef) -> &'m Expr {
        &self.expr_arena[e.0 as usize]
    }

    pub fn to_type(&'m self, t: TypeRef) -> &'m Type {
        &self.type_arena[t.0 as usize]
    }

    pub fn to_stmt(&'m self, s: StmtRef) -> &'m Stmt {
        &self.stmt_arena[s.0 as usize]
    }
}
