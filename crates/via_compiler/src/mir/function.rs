use crate::db::Symbol;
use crate::mir::ty::Ty;

#[salsa::tracked(debug)]
pub struct Function<'db> {
    pub name: Symbol<'db>,
    pub args: Vec<Ty<'db>>,
    pub result: Ty<'db>,
}
