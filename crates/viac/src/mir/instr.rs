use derive_more::From;
use pretty::RcDoc;

use crate::{counter::Id, sema::ConstValue};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TempId(u32);

impl TempId {
    pub fn to_doc(&self) -> RcDoc<'_> {
        RcDoc::text(format!("v{}", self.0))
    }
}

impl Id for TempId {
    type Inner = u32;
    fn new(inner: Self::Inner) -> Self {
        Self(inner)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LocalId(u32);

impl LocalId {
    pub fn to_doc(&self) -> RcDoc<'_> {
        RcDoc::text(format!("l{}", self.0))
    }
}

impl Id for LocalId {
    type Inner = u32;
    fn new(inner: Self::Inner) -> Self {
        Self(inner)
    }
}

#[derive(From, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operand {
    Discard,
    Temp(TempId),
    Local(LocalId),
}

impl Operand {
    pub fn to_doc(&self) -> RcDoc<'_> {
        match self {
            Self::Discard => RcDoc::text("_"),
            Self::Temp(temp) => temp.to_doc(),
            Self::Local(local) => local.to_doc(),
        }
    }
}

#[derive(Debug)]
pub enum Instr {
    Local {
        id: Operand,
        out: LocalId,
    },
    Const {
        value: ConstValue,
        out: Operand,
    },
    IAdd {
        lhs: Operand,
        rhs: Operand,
        out: Operand,
    },
}

impl Instr {
    pub fn to_doc(&self) -> RcDoc<'_> {
        match self {
            Self::Local { id, out } => out.to_doc().append(" = ").append(id.to_doc()),
            Self::Const { value, out } => out.to_doc().append(" = const ").append(value.to_doc()),
            Self::IAdd { lhs, rhs, out } => out
                .to_doc()
                .append(" = iadd ")
                .append(lhs.to_doc())
                .append(", ")
                .append(rhs.to_doc()),
        }
    }
}
