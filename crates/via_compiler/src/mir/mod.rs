use crate::{
    db::{Db, SourceProgram},
    hir::lower_program_to_hir,
    mir::{builder::MirBuilder, function::Function},
};

pub mod builder;
pub mod expr;
pub mod function;
pub mod stat;
pub mod ty;
pub mod value;

#[salsa::tracked(debug)]
pub struct Mir<'db> {
    pub functions: Vec<Function<'db>>,
}

#[salsa::tracked]
pub fn lower_program_to_mir<'db>(db: &'db dyn Db, program: SourceProgram) -> Option<Mir<'db>> {
    lower_program_to_hir(db, program).map(|hir| MirBuilder::new(db, hir).lower())
}
