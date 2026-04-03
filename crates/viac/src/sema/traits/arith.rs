use super::{
    super::{context::SemContext, error::*, func::Intrinsic, ty::Ty},
    builder::{ImplBuilder, TraitBuilder},
};
use crate::module::symbol::SymbolTable;

pub fn register_builtin_arith(st: &mut SymbolTable, sem: &mut SemContext) -> Result<()> {
    let int = sem.intern_ty(Ty::Int);
    let float = sem.intern_ty(Ty::Float);

    let this = sem.intern_ty(Ty::This);

    let add_proto = TraitBuilder::new(st, sem, "Add")
        .method("add", &[this, this], this)?
        .finish()?;
    let add_proto = sem.register_trait(add_proto)?;

    ImplBuilder::new(st, sem)
        .impl_intr(add_proto, "add", int, Intrinsic::IAdd)?
        .impl_intr(add_proto, "add", float, Intrinsic::FAdd)?;

    Ok(())
}
