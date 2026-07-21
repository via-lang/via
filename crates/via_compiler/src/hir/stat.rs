use salsa::Update;

use super::def::Def;
use super::expr::Expr;
use super::pat::Pat;
use super::ty::Ty;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Update)]
pub struct Local<'db> {
    pub pat: Pat<'db>,
    pub ty: Ty<'db>,
    pub expr: Expr<'db>,
}

#[salsa::tracked(debug)]
pub struct Body<'db> {
    #[returns(ref)]
    pub stats: Vec<Stat<'db>>,
    pub tail: Option<Expr<'db>>,
}

#[salsa::tracked(debug)]
pub struct Stat<'db> {
    #[returns(ref)]
    pub kind: StatKind<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum StatKind<'db> {
    Local(Local<'db>),
    Expr(Expr<'db>),
    Def(Def<'db>),
}
