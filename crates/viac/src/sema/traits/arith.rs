use super::{
    super::{context::SemContext, error::*, func::Intrinsic, ty::Ty},
    builder::ImplBuilder,
};
use crate::module::symbol::SymbolTable;

pub fn register_builtin_arith_traits(st: &mut SymbolTable, sem: &mut SemContext) -> Result<()> {
    let int = sem.intern_ty(Ty::Int);
    let float = sem.intern_ty(Ty::Float);

    ImplBuilder::new(st, sem)
        .register("Add", "add", vec![int, int], int, Intrinsic::IAdd)?
        .register("Add", "add", vec![float, float], float, Intrinsic::FAdd)?;

    Ok(())
}
