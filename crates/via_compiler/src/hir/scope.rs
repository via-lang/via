use itertools::Itertools;
use salsa::Update;

use crate::db::{Db, IntoSymbol};

use super::def::{DefKind, FnArg, FnImpl, Function};
use super::expr::ExprKind;
use super::pat::{Pat, PatKind};
use super::stat::{Body, Local, Stat, StatKind};
use super::ty::Ty;

/// Represents a locally scoped binding.
#[salsa::tracked(debug)]
pub struct LocalBinding<'db> {
    pub kind: LocalBindingKind<'db>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum LocalBindingKind<'db> {
    /// A local variable.
    Local(Local<'db>),

    /// A function argument.
    Argument(FnArg<'db>),

    /// A pattern introduced as a side effect by certain statements, such as `if let` and `for`.
    Pattern(Pat<'db>, Option<Ty<'db>>),
}

/// Queries the pattern of a local binding.
#[salsa::tracked]
pub fn get_local_pat<'db>(db: &'db dyn Db, local: LocalBinding<'db>) -> Pat<'db> {
    match local.kind(db) {
        LocalBindingKind::Local(local) => local.pat,
        LocalBindingKind::Argument(arg) => match arg {
            FnArg::Reciever(reciever) => Pat::new(
                db,
                PatKind::Binding {
                    reference: reciever.reference(db),
                    mutable: reciever.mutable(db),
                    name: "self".into_symbol(db),
                },
            ),
            FnArg::Typed(typed) => typed.pat(db),
        },
        LocalBindingKind::Pattern(pat, _) => pat,
    }
}

/// A lexical scope.
#[salsa::tracked(debug)]
pub struct Scope<'db> {
    #[returns(ref)]
    pub scopes: Vec<Scope<'db>>,
    #[returns(ref)]
    pub bindings: Vec<LocalBinding<'db>>,
}

/// Queries the scope of an HIR body.
#[salsa::tracked]
pub fn get_body_scope<'db>(
    db: &'db dyn Db,
    body: Body<'db>,
    mut bindings: Vec<LocalBinding<'db>>,
) -> Scope<'db> {
    let mut scopes = Vec::new();

    for stat in body.stats(db) {
        scopes.append(&mut get_stat_scope(db, *stat));

        if let Some(binding) = get_stat_binding(db, *stat) {
            bindings.push(binding);
        }
    }

    Scope::new(db, scopes, bindings)
}

/// Queries the scope of an HIR function.
#[salsa::tracked]
pub fn get_function_scope<'db>(db: &'db dyn Db, function: Function<'db>) -> Scope<'db> {
    let bindings = function
        .signature(db)
        .input(db)
        .iter()
        .map(|arg| LocalBinding::new(db, LocalBindingKind::Argument(*arg)))
        .collect_vec();

    if let FnImpl::User(body) = function.body(db) {
        get_body_scope(db, body, bindings)
    } else {
        Scope::new(db, Vec::new(), bindings)
    }
}

/// Queries the binding of a non-scoped statement.
#[salsa::tracked]
pub fn get_stat_binding<'db>(db: &'db dyn Db, stat: Stat<'db>) -> Option<LocalBinding<'db>> {
    let binding = match stat.kind(db) {
        StatKind::Local(local) => LocalBinding::new(db, LocalBindingKind::Local(*local)),
        _ => return None,
    };
    Some(binding)
}

/// Queries the scope of a statement.
#[salsa::tracked]
pub fn get_stat_scope<'db>(db: &'db dyn Db, stat: Stat<'db>) -> Vec<Scope<'db>> {
    let mut scopes = Vec::new();

    #[allow(clippy::all)]
    match stat.kind(db) {
        StatKind::Def(item) => match item.kind(db) {
            DefKind::Function(fun) => scopes.push(get_function_scope(db, fun)),
            _ => {}
        },

        StatKind::Expr(expr) => match expr.kind(db) {
            ExprKind::If {
                cond: _cond,
                then_body,
                else_body,
            } => {
                let scope = get_body_scope(db, *then_body, vec![]);
                scopes.push(scope);

                if let Some(else_body) = else_body {
                    let scope = get_body_scope(db, *else_body, vec![]);
                    scopes.push(scope);
                }
            }

            ExprKind::For { pat, body, .. } => {
                let scope = get_body_scope(
                    db,
                    *body,
                    vec![LocalBinding::new(db, LocalBindingKind::Pattern(*pat, None))],
                );
                scopes.push(scope);
            }

            _ => {}
        },

        _ => {}
    };

    scopes
}
