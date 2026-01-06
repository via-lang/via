/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::expr::ExprRef;
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct TypeRef(pub(super) u32);

#[derive(Debug)]
pub struct Type {
    pub span: Span,
    pub kind: TypeKind,
}

#[derive(Debug)]
pub enum TypeKind {
    Builtin {
        kind: BuiltinKind,
    },
    Optional {
        typ: TypeRef,
    },
    Array {
        typ: TypeRef,
    },
    Map {
        key: TypeRef,
        value: TypeRef,
    },
    Function {
        params: Vec<TypeRef>,
        result: TypeRef,
    },
    TypeOf {
        expr: ExprRef,
    },
}
