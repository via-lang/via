/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{decl::Variable, expr::ExprRef, stmt::StmtRef, typ::TypeRef};
use crate::compiler::lexer::token::Token;

#[derive(Debug)]
pub struct Break;

#[derive(Debug)]
pub struct Continue;

#[derive(Debug)]
pub struct Return {
    pub expr: Option<ExprRef>,
}

#[derive(Debug)]
pub struct Raise {
    pub expr: ExprRef,
}

#[derive(Debug)]
pub struct If {
    pub cond: ExprRef,
    pub body: Vec<StmtRef>,
    pub elifs: Vec<(ExprRef, Vec<StmtRef>)>,
    pub els: Option<Vec<StmtRef>>,
}

#[derive(Debug)]
pub struct While {
    pub cond: ExprRef,
    pub body: Vec<StmtRef>,
}

#[derive(Debug)]
pub struct WhileNot {
    pub cond: ExprRef,
    pub body: Vec<StmtRef>,
}

#[derive(Debug)]
pub struct For {
    pub init: Variable,
    pub cond: ExprRef,
    pub action: ExprRef,
    pub body: Vec<StmtRef>,
}

#[derive(Debug)]
pub struct ForEach {
    pub param: (Token, Option<TypeRef>),
    pub expr: ExprRef,
    pub body: Vec<StmtRef>,
}

#[derive(Debug)]
pub enum Control {
    Break(Break),
    Continue(Continue),
    Return(Return),
    Raise(Raise),
    If(If),
    While(While),
    WhileNot(WhileNot),
    For(For),
    ForEach(ForEach),
}
