/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use derive_more::{Add, AddAssign, From};

use super::counter::Id;
use crate::{
    module::symbol::SymbolId,
    sema::{ty::TyId, value::ConstValue},
};

#[repr(transparent)]
#[derive(From, Add, AddAssign, Clone, Copy, Debug)]
pub struct ValueId(usize);

impl Id for ValueId {}

#[repr(transparent)]
#[derive(From, Add, AddAssign, Clone, Copy, Debug)]
pub struct LocalId(usize);

impl Id for LocalId {}

#[derive(Debug)]
pub enum Instr {
    Const {
        value: ConstValue,
        out: ValueId,
    },
    Range {
        inclusive: bool,
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Tuple {
        values: Vec<ValueId>,
        out: ValueId,
    },
    Array {
        values: Vec<ValueId>,
        out: ValueId,
    },
    Bind {
        value: ValueId,
        out: LocalId,
    },
    Copy {
        value: ValueId,
        out: ValueId,
    },
    Closure {
        symbol: Option<SymbolId>,
        upvals: Vec<ValueId>,
        out: ValueId,
    },
    Static {
        value: ValueId,
        index: SymbolId,
        out: ValueId,
    },
    Dynamic {
        value: ValueId,
        index: SymbolId,
        out: ValueId,
    },
    Access {
        value: ValueId,
        index: ValueId,
        out: ValueId,
    },
    Call {
        callee: ValueId,
        args: Vec<ValueId>,
        out: Option<ValueId>,
    },
    Cast {
        value: ValueId,
        ty: TyId,
        out: ValueId,
    },
    Negate {
        in_: ValueId,
        out: ValueId,
    },
    Add {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Sub {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Mul {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Div {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Pow {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Mod {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Not {
        in_: ValueId,
        out: ValueId,
    },
    And {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Or {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    BitNot {
        in_: ValueId,
        out: ValueId,
    },
    BitAnd {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    BitOr {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    BitXor {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Shl {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
    Shr {
        lhs: ValueId,
        rhs: ValueId,
        out: ValueId,
    },
}
