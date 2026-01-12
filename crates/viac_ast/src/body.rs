/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::node::Node;
use crate::stmt::Stmt;
use viac_source::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub struct Body<T: Node = Stmt>(pub Span, pub Vec<T>);

impl Node for Body {
    fn span(&self) -> Span {
        self.0
    }
}
