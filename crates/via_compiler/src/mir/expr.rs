use salsa::Update;

use crate::{
    db::Db,
    mir::{
        function::Function,
        ty::{Ty, TyData},
        value::{Value, get_type_of_value},
    },
};

#[salsa::tracked(debug)]
pub struct Expr<'db> {
    pub data: ExprData<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum ExprData<'db> {
    Value(Value<'db>),
    Call(Function<'db>),
    Return(Option<Expr<'db>>),
}

#[salsa::tracked]
pub fn get_type_of_expr<'db>(db: &'db dyn Db, expr: Expr<'db>) -> Ty<'db> {
    match expr.data(db) {
        ExprData::Value(value) => get_type_of_value(db, value),
        ExprData::Call(call) => call.result(db),
        ExprData::Return(_) => Ty::new(db, TyData::Never),
    }
}
