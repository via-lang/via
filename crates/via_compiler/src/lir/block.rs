use salsa::Update;

use super::instr::{Instr, Temp};

#[salsa::tracked(debug)]
pub struct Block<'db> {
    pub instrs: Vec<Instr<'db>>,
    pub terminator: Terminator<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum Terminator<'db> {
    Raise(Temp),
    Return(Option<Temp>),
    Jump(Block<'db>),
    Branch {
        cond: Temp,
        iftrue: Block<'db>,
        iffalse: Block<'db>,
    },
}
