use crate::counter::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MetaId(u32);

impl Id for MetaId {
    type Inner = u32;
    fn new(inner: Self::Inner) -> Self {
        Self(inner)
    }
}

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
