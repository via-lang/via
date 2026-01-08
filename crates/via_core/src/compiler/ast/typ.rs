/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::expr::Expr;
use crate::compiler::source::Span;
use strum::AsRefStr;

#[derive(AsRefStr, Debug, Clone, Copy, Eq, PartialEq)]
pub enum BuiltinKind {
    None,
    Bool,
    Int,
    Float,
    String,
}

#[derive(Debug)]
pub enum Type {
    Builtin {
        span: Span,
        kind: BuiltinKind,
    },
    Optional {
        span: Span,
        typ: Box<Type>,
    },
    Array {
        span: Span,
        typ: Box<Type>,
    },
    Map {
        span: Span,
        key: Box<Type>,
        value: Box<Type>,
    },
    Function {
        span: Span,
        params: Vec<Type>,
        result: Box<Type>,
    },
    TypeOf {
        span: Span,
        expr: Box<Expr>,
    },
}

impl Type {
    pub fn span(&self) -> &Span {
        match self {
            Self::Builtin { span, .. }
            | Self::Optional { span, .. }
            | Self::Array { span, .. }
            | Self::Map { span, .. }
            | Self::Function { span, .. }
            | Self::TypeOf { span, .. } => span,
        }
    }
}
