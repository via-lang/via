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
use crate::sema::value::ConstValue;

#[repr(transparent)]
#[derive(From, Add, AddAssign, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TempId(u32);

impl Id for TempId {}

impl fmt::Display for TempId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[repr(transparent)]
#[derive(From, Add, AddAssign, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(u32);

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
    Negate {
        value: TempId,
        out: ValueId,
    },
    Not {
        in_: ValueId,
        out: ValueId,
    },
    BitNot {
        in_: ValueId,
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
