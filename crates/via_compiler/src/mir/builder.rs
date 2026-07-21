use itertools::Itertools;

use crate::db::Db;
use crate::hir::Hir;
use crate::hir::def::{Def, DefKind, Function as HirFunction, Module, TraitImpl, get_trait_impls};
use crate::hir::expr::Expr as HirExpr;
use crate::hir::resolve::{ResolveData, resolve_path};
use crate::hir::stat::Stat as HirStat;
use crate::hir::ty::Ty as HirTy;
use crate::mir::ty::TyData;
use crate::mir::value::Value;
use crate::path;

use super::Mir;
use super::expr::{Expr, ExprData, get_type_of_expr};
use super::function::Function;
use super::stat::Stat;
use super::ty::Ty;
use super::value::ValueData;

#[salsa::tracked]
pub fn type_qualifies_for_trait_impl<'db>(
    db: &'db dyn Db,
    ty: Ty<'db>,
    trait_impl: TraitImpl<'db>,
) -> bool {
    use crate::hir::ty::TyKind::*;

    match trait_impl.ty(db).kind(db) {
        _ => todo!(),
    }
}

pub struct MirBuilder<'db> {
    db: &'db dyn Db,
    module: Module<'db>,
    module_tree: Vec<Module<'db>>,
    functions: Vec<Function<'db>>,
}

impl<'db> MirBuilder<'db> {
    pub fn new(db: &'db dyn Db, hir: Hir<'db>) -> Self {
        Self {
            db,
            module: *hir.root(db),
            module_tree: Vec::new(),
            functions: Vec::new(),
        }
    }

    fn lower_type(&mut self, ty: HirTy<'db>) -> Ty<'db> {
        use crate::hir::ty::Obligation::*;
        use crate::hir::ty::TyKind::*;

        match ty.kind(self.db) {
            Primitive(primitive) => Ty::new(self.db, TyData::Primitive(*primitive)),

            Obligation(obligation) => match *obligation {
                This => {}
            },
        }
    }

    fn lower_expr(&mut self, expr: HirExpr<'db>) -> Expr<'db> {
        use crate::hir::expr::ExprKind::*;

        let data = match expr.kind(self.db) {
            Unit => ValueData::Unit,
            Bool(bool) => ValueData::Bool(*bool),
            Int(int) => ValueData::Int(*int),
            Float(float) => ValueData::Float(*float),
            String(string) => ValueData::String(string.clone()),

            Unary { op, expr } => {
                let expr = self.lower_expr(*expr);
                let ty = get_type_of_expr(self.db, expr);

                let (trait_name, trait_method) = op.trait_info();
                let trait_path = match trait_name {
                    "Neg" => path!(self.db, ::core::ops::Neg),
                    "Not" => path!(self.db, ::core::ops::Not),
                    "BitNot" => path!(self.db, ::core::ops::BitNot),
                    _ => unreachable!(),
                };

                let ResolveData::Def(def) =
                    resolve_path(self.db, self.module_tree.clone(), trait_path)
                        .expect("Built-in trait not found")
                else {
                    unreachable!("Path of built-in trait does not correspond to a definition")
                };

                let DefKind::Trait(trait_) = def.kind(self.db) else {
                    unreachable!("Path of built-in trait does not correspond to a trait definition")
                };

                let (trait_impl,) =
                    get_trait_impls(self.db, *self.module_tree.last().unwrap(), trait_)
                        .iter()
                        .cloned()
                        .filter(|trait_impl| {
                            type_qualifies_for_trait_impl(self.db, ty, *trait_impl)
                        })
                        .collect_tuple()
                        .expect("Ambiguous trait implementation");

                todo!()
            }

            _ => todo!(),
        };

        Expr::new(self.db, ExprData::Value(Value::new(self.db, data)))
    }

    fn lower_stat(&mut self, stat: HirStat<'db>) -> Stat<'db> {
        match stat.kind(self.db) {}
    }

    fn lower_def(&mut self, def: Def<'db>) {
        use DefKind::*;

        match def.kind(self.db) {
            Module(module) => self.lower_module(module),
            Function(function) => {}
            _ => {}
        }
    }

    fn lower_module(&mut self, module: Module<'db>) {
        self.module_tree.push(module);

        module
            .children(self.db)
            .iter()
            .for_each(|def| self.lower_def(*def));

        self.module_tree.pop();
    }

    fn lower_function(&mut self, function: HirFunction<'db>) {}

    pub fn lower(mut self) -> Mir<'db> {
        self.lower_module(self.module);
        Mir::new(self.db, self.functions)
    }
}
