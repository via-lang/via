/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::module::symbol::SymbolId;
use crate::sema::ty::TyId;
use crate::sema::value::ConstValue;
use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValueId(u32);

bitflags! {
    #[derive(Debug, PartialEq, Eq)]
    pub struct RefQuals: u8 {
        const None = 0;
        const Mutable = 1 << 1;
        const Strong = 1 << 2;
    }
}

#[derive(Debug)]
pub enum Instr {
    Const {
        value: ConstValue,
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
