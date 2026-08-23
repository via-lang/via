use ordered_float::OrderedFloat;
use salsa::Update;

use crate::{
    db::Db,
    hir::ty::Primitive,
    mir::ty::{Ty, TyData},
};

#[salsa::tracked(debug)]
pub struct Value<'db> {
    #[returns(ref)]
    pub data: ValueData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum ValueData {
    Unit,
    Bool(bool),
    Int(i64),
    Float(OrderedFloat<f64>),
    String(String),
}

#[salsa::tracked]
pub fn get_type_of_value<'db>(db: &'db dyn Db, value: Value<'db>) -> Ty<'db> {
    let data = match value.data(db) {
        ValueData::Unit => TyData::Primitive(Primitive::Unit),
        ValueData::Bool(_) => TyData::Primitive(Primitive::Bool),
        ValueData::Int(_) => TyData::Primitive(Primitive::Int),
        ValueData::Float(_) => TyData::Primitive(Primitive::Float),
        ValueData::String(_) => TyData::Primitive(Primitive::String),
    };

    Ty::new(db, data)
}
