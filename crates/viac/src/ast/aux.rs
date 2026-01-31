/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    node::{Marker, NodeRef, Nodes},
    stmt::Stmt,
    ty::Ty,
};
use crate::lexer::token::Token;

pub type Body = Nodes<Stmt>;

#[derive(Debug)]
pub struct Param {
    pub name: Token,
    pub ty: NodeRef<Ty>,
}

impl Marker for Param {}
impl PartialEq for Param {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}
