use bitflags::bitflags;
use via_vm::{Executor, IntoNativeFn, NativeClosure, ValueId};

use crate::{
    def::{DefContext, FnImpl, Intrin, error::Result},
    sema::{SemContext, Ty, TySubst},
    symbol::SymbolTable,
};

use builder::{FnBuilder, ImplBuilder, TraitBuilder};

mod builder;

bitflags! {
    pub struct ExtraLib: u8 {
        const IO = 1 >> 1;
    }
}

fn print(e: &mut Executor, args: Vec<ValueId>) {
    let str = e.heap().get(*args.first().unwrap()).as_string();
    println!("{str}");
}

pub fn register(
    st: &mut SymbolTable,
    sem: &mut SemContext,
    def: &mut DefContext,
    extra: ExtraLib,
) -> Result<()> {
    use Ty::*;

    let parent = None;

    let this = Subst(TySubst::This);

    let add_trait = TraitBuilder::new(st, sem, def, "Add")
        .method("add", &[this, this], this)?
        .finish(parent.clone())?;

    let add_trait = def.register_trait(add_trait)?;

    ImplBuilder::new(st, sem, def)
        .impl_intr(add_trait, "add", Int, Intrin::IAdd)?
        .impl_intr(add_trait, "add", Float, Intrin::FAdd)?;

    if extra.contains(ExtraLib::IO) {
        let print_fn = FnBuilder::new(st, sem, def, "print")
            .returns(Unit)
            .parameter(String)
            .with_body(parent, FnImpl::Native(NativeClosure::new(print, &[])))
            .build();

        def.register_fn(print_fn)?;
    }

    Ok(())
}
