/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::intern::Interned;

use constant::Constant;
use function::Function;
use record::Record;
use traits::Trait;
use ty::Ty;
use visibility::Visibility;

pub mod canonical_map;
pub mod constant;
pub mod context;
pub mod function;
pub mod record;
pub mod traits;
pub mod ty;
pub mod value;
pub mod visibility;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefKind<'cx> {
    Type(Interned<'cx, Ty<'cx>>),
    Trait(Interned<'cx, Trait<'cx>>),
    Record(Interned<'cx, Record<'cx>>),
    Function(Interned<'cx, Function<'cx>>),
    Constant(Interned<'cx, Constant<'cx>>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Def<'cx> {
    pub vis: Visibility,
    pub kind: DefKind<'cx>,
}
