use pretty::RcDoc;

use super::{Block, instr::TempId};
use crate::{macros::ice_unimplemented, node::NodeId};

#[derive(Debug)]
pub enum Term {
    Halt,
    Raise {
        value: TempId,
    },
    Return {
        value: Option<TempId>,
    },
    Jump {
        block: NodeId<Block>,
    },
    Branch {
        cond: TempId,
        iftrue: NodeId<Block>,
        iffalse: NodeId<Block>,
    },
}

impl Term {
    pub fn to_doc(&self) -> RcDoc<'_> {
        match self {
            Self::Halt => RcDoc::text("halt"),
            Self::Return { value } => {
                let mut base = RcDoc::text("ret");
                if let Some(value) = value {
                    base = base.append(value.to_doc());
                }
                base
            }
            Self::Jump { block } => RcDoc::text("jmp").append(" ").append(block.to_doc()),
            Self::Branch {
                cond,
                iftrue,
                iffalse,
            } => RcDoc::text("br")
                .append(" ")
                .append(cond.to_doc())
                .append(" ? ")
                .append(iftrue.to_doc())
                .append(" ")
                .append(iffalse.to_doc()),
            _ => ice_unimplemented!(),
        }
    }
}
