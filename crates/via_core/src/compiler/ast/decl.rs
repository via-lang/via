/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{expr::ExprRef, stmt::StmtRef, typ::TypeRef};
use crate::compiler::lexer::token::Token;

#[derive(Debug)]
pub struct Variable {
    pub symbol: Token,
    pub typ: Option<TypeRef>,
    pub expr: ExprRef,
}

#[derive(Debug)]
pub struct Function {
    pub symbol: Token,
    pub params: Vec<(Token, TypeRef)>,
    pub result: Option<TypeRef>,
    pub body: Vec<StmtRef>,
}

#[derive(Debug)]
pub struct Use {
    pub symbol: Token,
}

#[derive(Debug)]
pub struct Type {
    pub symbol: Token,
    pub typ: TypeRef,
}

#[derive(Debug)]
pub struct Const {
    pub symbol: Token,
    pub expr: ExprRef,
}

#[derive(Debug)]
pub struct Struct {
    pub symbol: Token,
    pub fields: Vec<Decl>,
}

#[derive(Debug)]
pub enum Decl {
    Variable(Variable),
    Function(Function),
    Use(Use),
    Type(Type),
    Const(Const),
}
