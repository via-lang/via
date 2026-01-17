/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

#[derive(Debug, Clone, Copy)]
pub enum Context {
    Attr,
    AttrDistinct,

    ExprPrimary,
    ExprTuple,
    ExprGroup,
    ExprArray,
    ExprMap,
    ExprLambda,

    TypeRet,
    TypeArray,
    TypeMap,
    TypeLambda,
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
