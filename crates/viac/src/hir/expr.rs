/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::{ast::expr::ExprId, sema::ty::TyId};

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    Not,
    BitNot,
}

#[derive(Debug)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    And,
    Or,
    BitAnd,
    BitOr,
    BitShl,
    BitShr,
}

#[derive(Debug)]
pub enum ExprKind {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(std::string::String),
    Range {
        inclusive: bool,
        begin: Box<Expr>,
        end: Box<Expr>,
    },
    Tuple {},
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub ast: ExprId,
}

impl Expr {}
