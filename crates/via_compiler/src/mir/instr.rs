use derive_more::From;
use pretty::RcDoc;
use via_macros::Id;
use via_vm::Immediate;

use crate::sema::ConstValue;

#[derive(Id)]
#[id(inner = u32)]
pub struct TempId(u32);

impl TempId {
    pub fn to_doc(&self) -> RcDoc<'_> {
        RcDoc::text(format!("v{}", self.0))
    }
}

#[derive(Id)]
#[id(inner = Immediate)]
pub struct LocalId(Immediate);

impl LocalId {
    pub fn to_doc(&self) -> RcDoc<'_> {
        RcDoc::text(format!("l{}", self.0))
    }
}

#[derive(From, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operand {
    Temp(TempId),
    Local(LocalId),
}

impl Operand {
    pub fn to_doc(&self) -> RcDoc<'_> {
        match self {
            Self::Temp(temp) => temp.to_doc(),
            Self::Local(local) => local.to_doc(),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
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

impl Instruction {
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
