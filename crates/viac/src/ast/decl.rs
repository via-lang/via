/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::expr::Expr;
use super::extra::Body;
use super::extra::NodeList;
use super::extra::Param;
use super::macros::ast;
use super::node::NodeRef;
use super::ty::Ty;
use crate::lexer::token::Token;

ast! {
    pub enum Decl {
        Variable {
            symbol: Token,
            ty: Option<NodeRef<Ty>>,
            expr: NodeRef<Expr>,
        },
        Function {
            symbol: Token,
            params: NodeList<Param>,
            result: Option<NodeRef<Ty>>,
            body: Body,
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
            body: NodeList<Decl>,
        },
        Import {
            path: Vec<Token>,
            alias: Option<Token>,
        },
    }
}
