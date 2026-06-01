use crate::{
    def::{DefContext, NsDef, error::Result},
    sema::SemContext,
    symbol::{IntoSymbol, StringInterner},
};

mod builder;
mod ops;

pub fn open(
    interner: &mut StringInterner,
    sem_ctxt: &mut SemContext,
    def_ctxt: &mut DefContext,
) -> Result<()> {
    let core_ns = def_ctxt.alloc_ns_def(NsDef {
        symbol: "core".into_symbol(interner),
        parent: None,
    });

    ops::open(interner, sem_ctxt, def_ctxt, core_ns)?;

    Ok(())
}
