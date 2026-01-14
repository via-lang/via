/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::expr::Expr;
use crate::extra::Body;
use crate::extra::Param;
use crate::macros::ast;
use crate::node::{Node, NodeRef};
use crate::ty::Ty;
use viac_lexer::token::Token;

ast! {
    pub enum Decl {
        Variable {
            symbol: Token,
            ty: Option<NodeRef<Ty>>,
            expr: NodeRef<Expr>,
        },
        Function {
            symbol: Token,
            params: Vec<Param>,
            result: Option<NodeRef<Ty>>,
            body: Node<Body>,
        },
        Use { symbol: Token },
        Type {
            symbol: Token,
            ty: NodeRef<Ty>,
        },
        Const {
            symbol: Token,
            expr: NodeRef<Expr>,
        },
        Struct {
            symbol: Token,
            body: Node<Body<Decl>>,
        },
        Import {
            path: Vec<Token>,
            alias: Option<Token>,
        },
    }
}
