/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum Context {
    Attr,

    ExprPrimary,
    ExprTuple,
    ExprGroup,
    ExprArray,
    ExprMap,
    ExprLambda,

    TypeRet,
    TypeArray,
    TypeMap,
    TypeFn,
    TypeUnion,
    TypeId,

    ControlReturn,
    ControlRaise,
    ControlIf,
    ControlElseIf,
    ControlWhile,
    ControlFor,

    DeclVariable,
    DeclFunction,
    DeclUse,
    DeclType,
    DeclConst,
    DeclStruct,
    DeclImport,

    Param,
    ParamList,
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Attr => "attribute",
                Self::ExprPrimary => "primary expression",
                Self::ExprTuple => "tuple expression",
                Self::ExprGroup => "group expression",
                Self::ExprArray => "array expression",
                Self::ExprMap => "map expression",
                Self::ExprLambda => "lambda expression",
                Self::TypeRet => "return type",
                Self::TypeUnion => "union type",
                Self::TypeArray => "array type",
                Self::TypeMap => "map type",
                Self::TypeFn => "function type",
                Self::TypeId => "typeid",
                Self::ControlReturn => "return statement",
                Self::ControlRaise => "raise statement",
                Self::ControlIf => "if statement",
                Self::ControlElseIf => "else-if statement",
                Self::ControlWhile => "while loop statement",
                Self::ControlFor => "for loop statement",
                Self::DeclVariable => "variable declaration",
                Self::DeclFunction => "function declaration",
                Self::DeclUse => "alias declaration",
                Self::DeclType => "type declaration",
                Self::DeclConst => "const declaration",
                Self::DeclStruct => "struct declaration",
                Self::DeclImport => "import declaration",
                Self::Param => "parameter declaration",
                Self::ParamList => "parameter list",
            }
        )
    }
}
