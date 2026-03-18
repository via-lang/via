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
    Const { value: ConstValue, out: ValueId },
    TraitCall {},
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
            _ => todo!(),
        }
    }
}
