/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt;

use derive_more::{Add, AddAssign, Display, From};

use super::counter::Id;
use crate::{
    hir::block::BlockId,
    module::symbol::SymbolId,
    sema::{ty::TyId, value::ConstValue},
};

#[repr(transparent)]
#[derive(From, Add, AddAssign, Debug, Clone, Copy, PartialEq)]
pub struct TempId(usize);

impl Id for TempId {}
impl fmt::Display for TempId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[repr(transparent)]
#[derive(From, Add, AddAssign, Debug, Clone, Copy, PartialEq)]
pub struct LocalId(usize);

impl Id for LocalId {}
impl fmt::Display for LocalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "l{}", self.0)
    }
}

#[derive(Display, From, Debug, Clone, Copy, PartialEq)]
pub enum ValueId {
    Temp(TempId),
    Local(LocalId),
}

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
    Copy {
        value: ValueId,
        out: ValueId,
    },
    Closure {
        block: BlockId,
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

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut write_out = |out: Option<ValueId>| {
            if let Some(out) = out {
                write!(f, "{out} = ")
            } else {
                Ok(())
            }
        };

        match self {
            Self::Const { value, out } => {
                write_out(Some(*out))?;
                writeln!(f, "{value}")
            }
            Self::Closure { block, upvals, out } => {
                write_out(Some(*out))?;
                writeln!(f, "closure<{block}> env={upvals:?}")
            }
            _ => todo!(),
        }
    }
}
