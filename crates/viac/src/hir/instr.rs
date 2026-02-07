/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt;

use derive_more::{Add, AddAssign, From};

use super::counter::Id;
use crate::{
    hir::block::BlockId,
    module::symbol::SymbolId,
    sema::{ty::TyId, value::ConstValue},
};

#[repr(transparent)]
#[derive(From, Add, AddAssign, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TempId(usize);

impl Id for TempId {}

impl fmt::Display for TempId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[repr(transparent)]
#[derive(From, Add, AddAssign, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(usize);

impl Id for LocalId {}

impl fmt::Display for LocalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "l{}", self.0)
    }
}

#[derive(From, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ValueId {
    Discard,
    Temp(TempId),
    Local(LocalId),
}

impl fmt::Display for ValueId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Discard => write!(f, "_"),
            Self::Temp(tmp) => write!(f, "{tmp}"),
            Self::Local(loc) => write!(f, "{loc}"),
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    Const {
        value: ConstValue,
        out: ValueId,
    },
    Range {
        inclusive: bool,
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Tuple {
        values: Vec<TempId>,
        out: ValueId,
    },
    Array {
        values: Vec<TempId>,
        out: ValueId,
    },
    Closure {
        block: BlockId,
        upvals: Vec<LocalId>,
        out: ValueId,
    },
    Copy {
        value: ValueId,
        out: ValueId,
    },
    Move {
        value: ValueId,
        out: ValueId,
    },
    Get {
        value: TempId,
        field: ValueId,
        out: ValueId,
    },
    GetStatic {
        value: TempId,
        field: SymbolId,
        out: ValueId,
    },
    GetDynamic {
        value: TempId,
        field: SymbolId,
        out: ValueId,
    },
    Call {
        callee: TempId,
        args: Vec<TempId>,
        out: Option<ValueId>,
    },
    Cast {
        value: TempId,
        ty: TyId,
        out: ValueId,
    },
    Negate {
        value: TempId,
        out: ValueId,
    },
    Add {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Sub {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Mul {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Div {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Pow {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Mod {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Not {
        in_: ValueId,
        out: ValueId,
    },
    And {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Or {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    BitNot {
        in_: ValueId,
        out: ValueId,
    },
    BitAnd {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    BitOr {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    BitXor {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Shl {
        lhs: TempId,
        rhs: TempId,
        out: ValueId,
    },
    Shr {
        lhs: TempId,
        rhs: TempId,
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

        fn stringify_vec<T: Id>(vec: &[T]) -> String {
            vec.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        }

        match self {
            Self::Const { value, out } => {
                write_out(Some(*out))?;
                writeln!(f, "{value}")
            }
            Self::Range {
                inclusive,
                lhs,
                rhs,
                out,
            } => {
                write_out(Some(*out))?;
                writeln!(
                    f,
                    "range {lhs}, {}{rhs}",
                    inclusive.then_some("=").unwrap_or_default()
                )
            }
            Self::Tuple { values, out } => {
                write_out(Some(*out))?;
                writeln!(f, "tuple {}", stringify_vec(values))
            }
            Self::Array { values, out } => {
                write_out(Some(*out))?;
                writeln!(f, "array {}", stringify_vec(values))
            }
            Self::Closure { block, upvals, out } => {
                write_out(Some(*out))?;
                writeln!(f, "closure{block} env=[{}]", stringify_vec(upvals))
            }
            Self::Copy { value, out } => {
                write_out(Some(*out))?;
                writeln!(f, "copy {value}")
            }
            Self::Move { value, out } => {
                write_out(Some(*out))?;
                writeln!(f, "move {value}")
            }
            Self::Get { value, field, out } => {
                write_out(Some(*out))?;
                writeln!(f, "{value}[{field}]")
            }
            Self::GetStatic { value, field, out } => {
                write_out(Some(*out))?;
                writeln!(f, "getstatic {value}, {field}")
            }
            Self::GetDynamic { value, field, out } => {
                write_out(Some(*out))?;
                writeln!(f, "getdyn {value}, {field}")
            }
            Self::Call { callee, args, out } => {
                write_out(*out)?;
                writeln!(f, "call {callee}({})", stringify_vec(args))
            }
            Self::Negate { value, out } => {
                write_out(Some(*out))?;
                writeln!(f, "-{value}")
            }
            Self::Add { lhs, rhs, out } => {
                write_out(Some(*out))?;
                writeln!(f, "{lhs} + {rhs}")
            }
            _ => todo!(),
        }
    }
}
