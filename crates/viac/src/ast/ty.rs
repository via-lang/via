/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::marker::PhantomData;

use crate::source::SourceSpan;

#[derive(Debug)]
pub enum TyKind {
    None,
    Bool,
    Int,
    Float,
}

#[derive(Debug)]
pub struct Ty {
    pub kind: TyKind,
    pub span: SourceSpan,
}
