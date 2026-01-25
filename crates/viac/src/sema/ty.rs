/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TyQuals: u8 {
        const None = 0;
        const Mutable = 1 << 1;
        const Reference = 1 << 2;
        const Option = 1 << 3;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TyId(u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TyKind {
    None,
    Bool,
    Int,
    Float,
    String,
    Array(TyId),
    Map { key: TyId, value: TyId },
    Function { result: TyId, params: Vec<TyId> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ty {
    pub kind: TyKind,
    pub quals: TyQuals,
}
