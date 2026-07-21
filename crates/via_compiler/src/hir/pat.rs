use salsa::Update;

use crate::db::{Db, Symbol};

/// A pattern.
#[salsa::tracked(debug)]
pub struct Pat<'db> {
    pub kind: PatKind<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum PatKind<'db> {
    Wildcard,
    Binding {
        reference: bool,
        mutable: bool,
        name: Symbol<'db>,
    },
}

/// Queries the subpattern of the given pattern with the given symbol.
#[salsa::tracked]
pub fn get_subpat<'db>(db: &'db dyn Db, pat: Pat<'db>, symbol: Symbol<'db>) -> Option<Pat<'db>> {
    match pat.kind(db) {
        PatKind::Binding { name, .. } if name == symbol => Some(pat),
        _ => None,
    }
}
