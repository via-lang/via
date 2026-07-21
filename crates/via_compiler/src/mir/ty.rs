use salsa::Update;

use crate::db::Db;
use crate::hir::ty::{Primitive, Ty as HirTy, TyKind as HirTyKind};

use super::value::Value;

#[salsa::tracked(debug)]
pub struct Ty<'db> {
    pub data: TyData<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum TyData<'db> {
    Never,
    Primitive(Primitive),
    Vector(Ty<'db>),
    Array(Ty<'db>, Value<'db>),
    Map(Ty<'db>, Ty<'db>),
}

#[salsa::tracked]
pub fn equals_hir_type<'db>(db: &'db dyn Db, ty: Ty<'db>, hir_ty: HirTy<'db>) -> bool {
    match ty.data(db) {
        TyData::Primitive(primitive) => *hir_ty.kind(db) == HirTyKind::Primitive(primitive),
        TyData::Vector(inner) => {
            if let HirTyKind::Vector(vector) = *hir_ty.kind(db) {
                equals_hir_type(db, inner, vector)
            } else {
                false
            }
        }
        _ => false,
    }
}
