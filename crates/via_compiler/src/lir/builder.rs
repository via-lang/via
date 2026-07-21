use crate::db::Db;
use crate::hir::Hir;
use crate::hir::def::Module;
use crate::hir::pat::PatKind;
use crate::hir::scope::Scope;
use crate::hir::stat::{Stat, StatKind};

use super::Lir;

pub struct MirBuilder<'db> {
    db: &'db dyn Db,
    current_module: Module<'db>,
    scope_ancestry: Vec<Scope<'db>>,
}

impl<'db> MirBuilder<'db> {
    pub fn new(db: &'db dyn Db, hir: Hir<'db>) -> Self {
        Self {
            db,
            current_module: *hir.root(db),
            scope_ancestry: Vec::new(),
        }
    }

    fn lower_stat(&mut self, stat: Stat<'db>) {
        match stat.kind(self.db) {
            StatKind::Def(_) => {}
            StatKind::Local(local) => match local.pat.kind(self.db) {
                PatKind::Wildcard => {}
                PatKind::Binding { name: _name, .. } => {}
            },
            _ => unimplemented!(),
        }
    }

    pub fn build(mut self) -> Lir<'db> {
        let blocks = Vec::new();

        Lir::new(self.db, blocks)
    }
}
