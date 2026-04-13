use via_macros::Id;

#[derive(Id)]
#[id(inner = u32)]
pub struct MetaId(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TySubst {
    This,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ty {
    Unit,
    Bool,
    Int,
    Float,
    String,
    Meta(MetaId),
    Subst(TySubst),
}
