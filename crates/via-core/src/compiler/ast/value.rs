/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{attr::Attr, expr::ExprRef, place::Place, stmt::StmtRef, typ::TypeRef};
use crate::compiler::lexer::token::Token;
use bitflags::bitflags;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ReferenceFlags(u8);

bitflags! {
    impl ReferenceFlags: u8 {
        const None = 0b00;
        const Mutable = 0b10;
        const Strong = 0b01;
    }
}

#[derive(Debug)]
pub struct Constant {
    pub token: Token,
}

#[derive(Debug)]
pub struct Tuple {
    pub exprs: Vec<ExprRef>,
}

#[derive(Debug)]
pub struct Array {
    pub exprs: Vec<ExprRef>,
}

#[derive(Debug)]
pub struct Map {
    pub pairs: Vec<(ExprRef, ExprRef)>,
}

#[derive(Debug)]
pub struct Lambda {
    pub params: Vec<(Token, TypeRef)>,
    pub result: Option<ExprRef>,
    pub body: Vec<StmtRef>,
}

#[derive(Debug)]
pub struct Unary {
    pub op: Token,
    pub expr: ExprRef,
}

#[derive(Debug)]
pub struct Binary {
    pub op: Token,
    pub lhs: ExprRef,
    pub rhs: ExprRef,
}

#[derive(Debug)]
pub struct Reference {
    pub flags: ReferenceFlags,
    pub expr: ExprRef,
}

#[derive(Debug)]
pub struct Ternary {
    pub cond: ExprRef,
    pub iftrue: ExprRef,
    pub iffalse: ExprRef,
}

#[derive(Debug)]
pub struct Cast {
    pub expr: ExprRef,
    pub typ: TypeRef,
}

#[derive(Debug)]
pub enum Value {
    Constant(Constant),
    Tuple(Tuple),
    Array(Array),
    Map(Map),
    Lambda(Lambda),
    Unary(Unary),
    Binary(Binary),
    Reference(Reference),
    Ternary(Ternary),
    Cast(Cast),
    Attr(Attr),
    Read(Place),
}
