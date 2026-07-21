use itertools::Itertools;

use crate::db::{Db, IntoSymbol, Symbol};
use crate::generator::Generator;
use crate::path;
use crate::syntax::{self, Ast, Root, SyntaxKind, SyntaxNode};

use super::Hir;
use super::def::{
    Const, Def, DefKind, FnArg, FnImpl, Function, Module, Signature, Type, TypedArg, Visibility,
};
use super::expr::{Expr, ExprKind};
use super::ops::{BinaryOp, UnaryOp};
use super::pat::{Pat, PatKind};
use super::path::{Path, PathHead, PathSegment};
use super::stat::{Body, Local, Stat, StatKind};
use super::ty::{MetaVar, Obligation, Primitive, Ty, TyKind};

pub struct HirBuilder<'db> {
    db: &'db dyn Db,
    root: Root,
    metavar_gen: Generator<MetaVar>,
}

impl<'db> HirBuilder<'db> {
    pub fn new(db: &'db dyn Db, ast: Ast<'db>) -> Self {
        let green = ast.root(db);
        let red = SyntaxNode::new_root(green);
        let root = Root::cast(red).expect("Expected ROOT");

        Self {
            db,
            root,
            metavar_gen: Generator::new(),
        }
    }

    fn lower_path(&mut self, path: syntax::Path) -> Path<'db> {
        Path::new(
            self.db,
            path.head().map(|head| match head.token().kind() {
                SyntaxKind::COLON_COLON => PathHead::Absolute,
                _ => unreachable!(),
            }),
            path.segments()
                .map(|segment| {
                    PathSegment::new(
                        self.db,
                        segment.ident().text().to_string().into_symbol(self.db),
                    )
                })
                .collect_vec(),
        )
    }

    fn lower_scope(&mut self, scope: syntax::Scope) -> Body<'db> {
        let mut hir_stats = Vec::new();
        let mut tail = None;

        let mut stats = scope.stats().peekable();

        while let Some(stat) = stats.next() {
            if stats.peek().is_none() {
                match stat {
                    syntax::Stat::Consume(expr) => {
                        tail = Some(self.lower_expr(expr.expr()));
                    }
                    _ => {
                        hir_stats.push(self.lower_stat(stat));
                    }
                }
            } else {
                hir_stats.push(self.lower_stat(stat));
            }
        }

        Body::new(self.db, hir_stats, tail)
    }

    fn lower_pat(&mut self, pat: syntax::Pat) -> Pat<'db> {
        use syntax::Pat::*;

        let inner = match pat {
            Wildcard(_) => PatKind::Wildcard,
            Ident(ident) => PatKind::Binding {
                reference: ident.refer().is_some(),
                mutable: ident.mutable().is_some(),
                name: ident.ident().text().to_string().into_symbol(self.db),
            },
            _ => unimplemented!(),
        };

        Pat::new(self.db, inner)
    }

    fn lower_ty(&mut self, ty: syntax::Ty) -> Ty<'db> {
        use syntax::Ty::*;

        let inner = match ty {
            Unit(_) => TyKind::Primitive(Primitive::Unit),
            Vector(vector) => TyKind::Vector(self.lower_ty(vector.inner())),
            Array(array) => TyKind::Array {
                ty: self.lower_ty(array.ty()),
                size: self.lower_expr(array.size()),
            },

            Map(map) => TyKind::Map {
                key: self.lower_ty(map.key()),
                value: self.lower_ty(map.value()),
            },

            Qual(qual) => {
                let path = self.lower_path(qual.path());
                if path == path!(self.db, Bool) {
                    TyKind::Primitive(Primitive::Bool)
                } else if path == path!(self.db, Int) {
                    TyKind::Primitive(Primitive::Int)
                } else if path == path!(self.db, Float) {
                    TyKind::Primitive(Primitive::Float)
                } else if path == path!(self.db, String) {
                    TyKind::Primitive(Primitive::String)
                } else {
                    TyKind::Obligation(Obligation::Path(path))
                }
            }
        };

        Ty::new(self.db, inner)
    }

    fn lower_expr(&mut self, expr: syntax::Expr) -> Expr<'db> {
        use syntax::Expr::*;

        match expr {
            Unit(_) => Expr::new(self.db, ExprKind::Unit),

            Bool(bool) => Expr::new(
                self.db,
                ExprKind::Bool(match bool.literal().kind() {
                    SyntaxKind::KW_TRUE => true,
                    SyntaxKind::KW_FALSE => false,
                    _ => unreachable!(),
                }),
            ),

            Int(int) => Expr::new(
                self.db,
                ExprKind::Int(
                    int.literal()
                        .text()
                        .parse::<i64>()
                        .expect("Invalid integer literal"),
                ),
            ),

            Float(float) => Expr::new(
                self.db,
                ExprKind::Float(
                    float
                        .literal()
                        .text()
                        .parse::<f64>()
                        .expect("Invalid float literal")
                        .into(),
                ),
            ),

            String(string) => Expr::new(self.db, ExprKind::String(string.literal().to_string())),

            Array(array) => Expr::new(
                self.db,
                ExprKind::Array(array.exprs().map(|e| self.lower_expr(e)).collect_vec()),
            ),

            Map(map) => Expr::new(
                self.db,
                ExprKind::Map(
                    map.pairs()
                        .map(|pair| (self.lower_expr(pair.key()), self.lower_expr(pair.value())))
                        .collect_vec(),
                ),
            ),

            Qual(qual) => Expr::new(self.db, ExprKind::Path(self.lower_path(qual.path()))),

            Unary(un) => Expr::new(
                self.db,
                ExprKind::Unary {
                    op: UnaryOp::from_syntax(un.op().kind()).expect("Invalid unary operator"),
                    expr: self.lower_expr(un.expr()),
                },
            ),

            Binary(bin) => {
                let lhs = self.lower_expr(bin.lhs());
                let rhs = self.lower_expr(bin.rhs());

                let inner = match bin.op().kind() {
                    SyntaxKind::EQ => ExprKind::Assign { lhs, rhs },

                    op @ (SyntaxKind::DOT_DOT | SyntaxKind::DOT_DOT_EQ) => ExprKind::Range {
                        inclusive: op == SyntaxKind::DOT_DOT_EQ,
                        lhs: Some(lhs),
                        rhs: Some(rhs),
                    },

                    _ => ExprKind::Binary {
                        op: BinaryOp::from_syntax(bin.op().kind())
                            .expect("Invalid binary operator"),
                        lhs,
                        rhs,
                    },
                };

                Expr::new(self.db, inner)
            }

            Index(index) => Expr::new(
                self.db,
                ExprKind::Index {
                    lhs: self.lower_expr(index.outer()),
                    rhs: self.lower_expr(index.inner()),
                },
            ),

            Call(call) => Expr::new(
                self.db,
                ExprKind::Call {
                    callee: self.lower_expr(call.callee()),
                    args: call.args().map(|arg| self.lower_expr(arg)).collect_vec(),
                },
            ),

            If(if_expr) => {
                let mut else_body = if_expr
                    .else_clause()
                    .map(|else_block| self.lower_scope(else_block.scope()));

                for branch in if_expr.else_if_clauses().collect_vec().into_iter().rev() {
                    let cond = self.lower_expr(branch.cond());
                    let then_body = self.lower_scope(branch.scope());

                    let if_expr = Expr::new(
                        self.db,
                        ExprKind::If {
                            cond,
                            then_body,
                            else_body,
                        },
                    );

                    else_body = Some(Body::new(self.db, Vec::new(), Some(if_expr)));
                }

                Expr::new(
                    self.db,
                    ExprKind::If {
                        cond: self.lower_expr(if_expr.cond()),
                        then_body: self.lower_scope(if_expr.scope()),
                        else_body,
                    },
                )
            }

            For(for_expr) => Expr::new(
                self.db,
                ExprKind::For {
                    pat: self.lower_pat(for_expr.pat()),
                    iter: self.lower_expr(for_expr.iter()),
                    body: self.lower_scope(for_expr.scope()),
                },
            ),

            Group(group) => self.lower_expr(group.inner()),
        }
    }

    fn lower_stat_let(&mut self, stat: syntax::StatLet) -> Stat<'db> {
        let pat = self.lower_pat(stat.pat());
        let ty = stat.ty().map(|ty| self.lower_ty(ty)).unwrap_or_else(|| {
            Ty::new(
                self.db,
                TyKind::Obligation(Obligation::MetaVar(self.metavar_gen.next_id())),
            )
        });

        let expr = stat
            .init()
            .map(|e| self.lower_expr(e))
            .expect("Optional initializers not yet implemented");

        Stat::new(self.db, StatKind::Local(Local { pat, ty, expr }))
    }

    fn lower_stat(&mut self, stat: syntax::Stat) -> Stat<'db> {
        use syntax::Stat::*;

        match stat {
            Let(stat_let) => self.lower_stat_let(stat_let),
            Consume(consume) => {
                let expr = self.lower_expr(consume.expr());
                Stat::new(self.db, StatKind::Expr(expr))
            }
            Discard(discard) => {
                let expr = self.lower_expr(discard.expr());
                Stat::new(self.db, StatKind::Expr(expr))
            }

            Def(_) => unimplemented!(),
        }
    }

    fn lower_def_const(&mut self, stat: syntax::DefConst) -> Def<'db> {
        let constant = Const::new(
            self.db,
            Visibility::Priv,
            stat.name().text().into_symbol(self.db),
            self.lower_ty(stat.ty()),
            self.lower_expr(stat.init()),
        );

        Def::new(self.db, DefKind::Const(constant))
    }

    fn lower_def_type(&mut self, stat: syntax::DefType) -> Def<'db> {
        let ty = Type::new(
            self.db,
            Visibility::Priv,
            stat.name().text().to_string().into_symbol(self.db),
            self.lower_ty(stat.ty()),
        );

        Def::new(self.db, DefKind::Type(ty))
    }

    fn lower_def_fn(&mut self, stat: syntax::DefFn) -> Def<'db> {
        let name = stat.name().text().to_string().into_symbol(self.db);
        let sig = Signature::new(
            self.db,
            name,
            stat.params()
                .map(|parameter| {
                    FnArg::Typed(TypedArg::new(
                        self.db,
                        self.lower_pat(parameter.pat()),
                        self.lower_ty(parameter.ty()),
                    ))
                })
                .collect_vec(),
            stat.result()
                .map(|r| self.lower_ty(r))
                .unwrap_or_else(|| Ty::new(self.db, TyKind::Primitive(Primitive::Unit))),
        );

        let body = self.lower_scope(stat.body());
        let imp = FnImpl::User(body);
        let fun = Function::new(self.db, Visibility::Priv, name, sig, imp);

        Def::new(self.db, DefKind::Function(fun))
    }

    fn lower_def(&mut self, def: syntax::Def) -> Def<'db> {
        use syntax::Def::*;

        match def {
            Const(def) => self.lower_def_const(def),
            Type(def) => self.lower_def_type(def),
            Fn(def) => self.lower_def_fn(def),
        }
    }

    pub fn build(mut self, name: Symbol<'db>) -> Hir<'db> {
        let mut defs = Vec::new();

        for ast_def in self.root.clone().defs() {
            let def = self.lower_def(ast_def);
            defs.push(def);
        }

        Hir::new(self.db, Module::new(self.db, Visibility::Pub, name, defs))
    }
}
