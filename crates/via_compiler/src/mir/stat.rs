use salsa::Update;

use crate::mir::expr::Expr;

#[salsa::tracked(debug)]
pub struct Stat<'db> {
    pub data: StatData<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum StatData<'db> {
    Expr(Expr<'db>),
}
