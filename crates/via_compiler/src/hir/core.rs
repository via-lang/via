use super::def::{Module, Visibility};
use crate::db::{Db, IntoSymbol};

#[salsa::tracked]
pub fn get_core_module<'db>(db: &'db dyn Db) -> Module<'db> {
    Module::new(db, Visibility::Pub, "core".into_symbol(db), Vec::new())
}
