use salsa::Update;

use crate::db::{Db, Symbol};

use super::def::{Def, DefKind, ImplAssoc, Module, TraitAssoc, TraitImplAssoc, get_name};
use super::pat::{Pat, get_subpat};
use super::path::{Path, PathHead};
use super::scope::{LocalBinding, Scope, get_local_pat};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum ResolveData<'db> {
    Local {
        binding: LocalBinding<'db>,
        subpat: Pat<'db>,
    },
    Def(Def<'db>),
    ImplAssoc(ImplAssoc<'db>),
    TraitAssoc(TraitAssoc<'db>),
    TraitImplAssoc(TraitImplAssoc<'db>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum ResolveError {
    NotFound,
    Inaccessible,
}

pub type ResolveResult<'db> = Result<ResolveData<'db>, ResolveError>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub struct ResolveCtxt<'db> {
    pub current_module: Module<'db>,
    pub scope_ancestry: Vec<Scope<'db>>,
}

#[salsa::tracked]
pub fn resolve_local<'db>(
    db: &'db dyn Db,
    scope: Scope<'db>,
    name: Symbol<'db>,
) -> ResolveResult<'db> {
    for binding in scope.bindings(db).iter().rev().cloned() {
        let pat = get_local_pat(db, binding);
        if let Some(subpat) = get_subpat(db, pat, name) {
            return Ok(ResolveData::Local { binding, subpat });
        }
    }
    Err(ResolveError::NotFound)
}

#[salsa::tracked]
pub fn resolve_item<'db>(
    db: &'db dyn Db,
    module: Module<'db>,
    name: Symbol<'db>,
) -> ResolveResult<'db> {
    for child in module.children(db) {
        if get_name(db, child) == Some(name) {
            return Ok(ResolveData::Def(child));
        }
    }
    Err(ResolveError::NotFound)
}

#[salsa::tracked]
pub fn resolve<'db>(
    db: &'db dyn Db,
    ctxt: ResolveCtxt<'db>,
    name: Symbol<'db>,
) -> ResolveResult<'db> {
    for scope in ctxt.scope_ancestry.iter().rev() {
        if let Ok(data) = resolve_local(db, *scope, name) {
            return Ok(data);
        }
    }

    if let Ok(data) = resolve_item(db, ctxt.current_module, name) {
        return Ok(data);
    }

    Err(ResolveError::NotFound)
}

#[salsa::tracked]
pub fn resolve_path<'db>(
    db: &'db dyn Db,
    module_ancestry: Vec<Module<'db>>,
    path: Path<'db>,
) -> ResolveResult<'db> {
    let segments = path.segments(db);
    if segments.is_empty() {
        panic!("Path cannot be empty");
    }

    let mut current_module = match path.head(db) {
        None => match module_ancestry.last() {
            Some(&module) => module,
            None => return Err(ResolveError::NotFound),
        },
        Some(PathHead::Absolute) => match module_ancestry.first() {
            Some(&root) => root,
            None => return Err(ResolveError::NotFound),
        },
        Some(PathHead::Super) => {
            if module_ancestry.len() < 2 {
                return Err(ResolveError::NotFound);
            }
            module_ancestry[module_ancestry.len() - 2]
        }
    };

    let (last_segment, intermediate_segments) = segments.split_last().unwrap();

    for segment in intermediate_segments {
        let name = segment.ident(db);

        #[allow(clippy::all)]
        match resolve_item(db, current_module, name)? {
            ResolveData::Def(item) => {
                if let DefKind::Module(module) = item.kind(db) {
                    current_module = module;
                    continue;
                }
            }
            _ => {}
        }
        return Err(ResolveError::NotFound);
    }

    let final_symbol = last_segment.ident(db);
    resolve_item(db, current_module, final_symbol)
}
