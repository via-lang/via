/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use array::Array;
use builtin::Builtin;
use function::Function;
use map::Map;

pub mod array;
pub mod builtin;
pub mod context;
pub mod function;
pub mod map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty<'cx> {
    Builtin(Builtin),
    Array(Array<'cx>),
    Map(Map<'cx>),
    Function(Function<'cx>),
}
