use crate::{
    core::builder::{assoc, generic},
    def::{DefContext, Intrin, NsDef, error::Result},
    node::NodeId,
    sema::{SemContext, Ty, TySubst},
    symbol::{IntoSymbol, StringInterner},
};

use super::builder::{TraitBuilder, TraitImplBuilder};

use Ty::*;

const BINARY_OP: &[(&str, &str, &[(Ty, Ty, Ty, Intrin)])] = &[
    (
        "Add",
        "add",
        &[
            (Int, Int, Int, Intrin::IAdd),
            (Int, Float, Float, Intrin::IAddF),
            (Float, Float, Float, Intrin::FAdd),
            (Float, Int, Float, Intrin::FAddI),
        ],
    ),
    (
        "Sub",
        "sub",
        &[
            (Int, Int, Int, Intrin::ISub),
            (Int, Float, Float, Intrin::ISubF),
            (Float, Float, Float, Intrin::FSub),
            (Float, Int, Float, Intrin::FSubI),
        ],
    ),
    (
        "Mul",
        "mul",
        &[
            (Int, Int, Int, Intrin::IMul),
            (Int, Float, Float, Intrin::IMulF),
            (Float, Float, Float, Intrin::FMul),
            (Float, Int, Float, Intrin::FMulI),
        ],
    ),
    (
        "Div",
        "div",
        &[
            (Int, Int, Int, Intrin::IDiv),
            (Int, Float, Float, Intrin::IDivF),
            (Float, Float, Float, Intrin::FDiv),
            (Float, Int, Float, Intrin::FDivI),
        ],
    ),
    (
        "Pow",
        "pow",
        &[
            (Int, Int, Int, Intrin::IPow),
            (Int, Float, Float, Intrin::IPowF),
            (Float, Float, Float, Intrin::FPow),
            (Float, Int, Float, Intrin::FPowI),
        ],
    ),
    (
        "Rem",
        "rem",
        &[
            (Int, Int, Int, Intrin::IRem),
            (Float, Float, Float, Intrin::FRem),
        ],
    ),
];

pub fn open(
    interner: &mut StringInterner,
    sem_ctxt: &mut SemContext,
    def_ctxt: &mut DefContext,
    core: NodeId<NsDef>,
) -> Result<()> {
    let ops_ns = def_ctxt.alloc_ns_def(NsDef {
        symbol: "ops".into_symbol(interner),
        parent: Some(core.into()),
    });

    let this = Subst(TySubst::This);

    let rhs = "Rhs".into_symbol(interner);
    let output = "Output".into_symbol(interner);

    for &(trait_name, method_name, impls) in BINARY_OP {
        let trait_id = TraitBuilder::new(interner, sem_ctxt, def_ctxt, trait_name)
            .generic(rhs, [], Some(this))
            .assoc(output, [], Option::<NodeId<Ty>>::None)
            .method(method_name, &[this, generic!(rhs)], assoc!(output))?
            .register(Some(ops_ns))?;

        for &(ref lhs, ref rhs_ty, ref out_ty, ref intrin) in impls {
            let mut b = TraitImplBuilder::new(interner, sem_ctxt, def_ctxt, trait_id, *lhs)
                .assoc(output, *out_ty)
                .method_intrin(method_name, *intrin);

            if rhs_ty != lhs {
                b = b.generic(*rhs_ty);
            }

            b.finish()?;
        }
    }

    Ok(())
}
