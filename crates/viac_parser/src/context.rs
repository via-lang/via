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
    ExprTuple,
    ExprGroup,
    ExprLambda,
    ExprBody,
    ExprPrimary,
    ExprPostfix,
    TypeMap,
    TypeLambda,
    TypeId,
    ControlReturn,
    ControlRaise,
    ControlIf,
    ControlElseIf,
    ControlElse,
    ControlWhile,
    ControlFor,
    DeclVariable,
    DeclFunction,
    DeclType,
    DeclConst,
    DeclStruct,
    DeclImport,
    Body,
    ReturnType,
    ParameterList,
    ArgumentList,
    Attr,
    AttrDistinct,
}
