use super::{
    super::{context::SemContext, error::*, func::Intrinsic, ty::Ty},
    builder::ImplBuilder,
};
use crate::module::symbol::SymbolTable;

pub fn register_builtin_arith_traits(st: &mut SymbolTable, sem: &mut SemContext) -> Result<()> {
    let int = sem.intern_ty(Ty::Int);
    let float = sem.intern_ty(Ty::Float);

    ImplBuilder::new(st, sem)
        .register_basic_intr("Add", "add", int, Intrinsic::IAdd)?
        .register_basic_intr("Add", "add", float, Intrinsic::FAdd)?;

    Ok(())
}
