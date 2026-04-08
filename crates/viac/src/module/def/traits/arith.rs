use super::{ImplBuilder, TraitBuilder};
use crate::{
    module::{
        def::{DefContext, Intrin, error::Result},
        symbol::SymbolTable,
    },
    sema::{SemContext, Ty, TySubst},
};

pub fn register_builtin_arith(
    st: &mut SymbolTable,
    sem: &mut SemContext,
    def: &mut DefContext,
) -> Result<()> {
    let int = sem.intern_ty(Ty::Int);
    let float = sem.intern_ty(Ty::Float);

    let this = sem.intern_ty(Ty::Subst(TySubst::This));

    let add_trait = TraitBuilder::new(st, def, "Add")
        .method("add", &[this, this], this)?
        .finish()?;
    let add_trait = def.register_trait(add_trait)?;

    ImplBuilder::new(st, sem, def)
        .impl_intr(add_trait, "add", int, Intrin::IAdd)?
        .impl_intr(add_trait, "add", float, Intrin::FAdd)?;

    Ok(())
}
