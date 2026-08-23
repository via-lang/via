use crate::db::{Db, SourceProgram};

pub mod block;
pub mod builder;
pub mod instr;

#[salsa::tracked(debug)]
pub struct Lir<'db> {
    #[returns(ref)]
    pub blocks: Vec<block::Block<'db>>,
}

#[salsa::tracked]
pub fn lower_program_to_lir<'db>(_db: &'db dyn Db, _program: SourceProgram) -> Option<Lir<'db>> {
    todo!()
}
