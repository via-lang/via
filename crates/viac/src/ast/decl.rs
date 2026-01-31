/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    aux::{Body, Param},
    expr::Expr,
    macros::ast,
    node::{NodeRef, Nodes},
    ty::Ty,
};
use crate::lexer::token::Token;

ast! {
    Decl {
        Variable {
            symbol: Token,
            ty: Option<NodeRef<Ty>>,
            expr: NodeRef<Expr>,
        },
        Function {
            symbol: Token,
            params: Nodes<Param>,
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
            body: Nodes<Decl>,
        },
        Import {
            path: Vec<Token>,
            alias: Option<Token>,
        },
    }
}
