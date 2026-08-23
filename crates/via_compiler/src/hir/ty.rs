use derive_more::From;
use salsa::Update;

use super::expr::Expr;
use super::path::Path;

/// Represents a type.
#[salsa::interned(debug)]
pub struct Ty<'db> {
    #[returns(ref)]
    pub kind: TyKind<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum TyKind<'db> {
    /// A primitive type.
    Primitive(Primitive),

    /// Type [$T].
    Vector(Ty<'db>),

    /// Type [$T; const $N].
    Array { ty: Ty<'db>, size: Expr<'db> },

    /// Type #{ $T: $U }.
    Map { key: Ty<'db>, value: Ty<'db> },

    /// Type &mut $T.
    Reference { mutable: bool, ty: Ty<'db> },

    /// A thunk to be solved during MIR.
    Obligation(Obligation<'db>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum Primitive {
    Unit,
    Bool,
    Int,
    Float,
    String,
}

/// Represents a dependent type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum Obligation<'db> {
    /// Represents the `Self` type.
    This,

    /// Represents type `_`, with relational context.
    MetaVar(MetaVar),

    /// Represents an arbitrary path.
    Path(Path<'db>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Update, From)]
pub struct MetaVar(pub u32);
