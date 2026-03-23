use std::fmt;

use derive_more::From;

use crate::{counter::Id, sema::value::ConstValue};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TempId(u32);

impl Id for TempId {
    type Inner = u32;
    fn new(inner: Self::Inner) -> Self {
        Self(inner)
    }
}

impl fmt::Display for TempId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(u32);

impl Id for LocalId {
    type Inner = u32;
    fn new(inner: Self::Inner) -> Self {
        Self(inner)
    }
}

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
    IAdd {
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
            _ => todo!(),
        }
    }
}
