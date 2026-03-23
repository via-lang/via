use std::fmt;

use super::{block::BlockId, instr::TempId};

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
        block: BlockId,
    },
    Branch {
        cond: TempId,
        iftrue: BlockId,
        iffalse: BlockId,
    },
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => writeln!(f, "halt"),
            Self::Return { value } => {
                if let Some(value) = value {
                    writeln!(f, "ret {value}")
                } else {
                    writeln!(f, "ret")
                }
            }
            Self::Jump { block } => {
                writeln!(f, "jmp {block}")
            }
            Self::Branch {
                cond,
                iftrue,
                iffalse,
            } => {
                writeln!(f, "br {cond}, {iftrue}, {iffalse}")
            }
            _ => todo!(),
        }
    }
}
