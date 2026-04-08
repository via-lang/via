use crate::node::NodeId;

use super::{instr::Instr, term::Term};
use pretty::RcDoc;

impl NodeId<Block> {
    pub fn to_doc(&self) -> RcDoc<'_> {
        RcDoc::text(format!("#{}", self.index()))
    }
}

#[derive(Debug)]
pub struct Block {
    pub id: NodeId<Block>,
    pub instrs: Vec<Instr>,
    pub term: Term,
}

impl Block {
    pub fn new() -> Self {
        Self {
            id: NodeId::new(0),
            instrs: vec![],
            term: Term::Halt,
        }
    }

    pub fn to_doc(&self) -> RcDoc<'_> {
        let label = self.id.to_doc().append(":");
        let instrs = RcDoc::intersperse(self.instrs.iter().map(|i| i.to_doc()), RcDoc::hardline());

        let body = RcDoc::hardline()
            .append(instrs)
            .append(RcDoc::hardline())
            .append(self.term.to_doc())
            .nest(2);

        label.append(RcDoc::hardline()).append(body)
    }
}

impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}
