/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::node::{Ast, Node, NodeRef};
use super::stmt::Stmt;
use super::ty::Ty;
use crate::lexer::token::Token;
use crate::source::span::Span;

#[derive(Debug, PartialEq)]
pub struct NodeList<T: Ast = Stmt> {
    pub list: Vec<Node<T>>,
    pub span: Span,
}

pub type Body = NodeList<Stmt>;

impl<T: Ast> Ast for NodeList<T> {}

#[derive(Debug)]
pub struct Param {
    pub name: Token,
    pub ty: NodeRef<Ty>,
}

impl Ast for Param {}

impl PartialEq for Param {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}
