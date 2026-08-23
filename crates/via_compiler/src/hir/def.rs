use itertools::Itertools;
use salsa::Update;

use crate::db::{Db, Symbol};

use super::core::get_core_module;
use super::expr::Expr;
use super::pat::Pat;
use super::stat::Body;
use super::ty::Ty;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, Update)]
pub enum Visibility<'db> {
    #[default]
    Priv,
    Pub,
    Restricted(Module<'db>),
}

#[salsa::tracked(debug)]
pub struct Generics<'db> {
    pub params: Vec<GenericParam<'db>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum GenericParam<'db> {
    Type(TypeParam<'db>),
    Const(ConstParam<'db>),
}
#[salsa::tracked(debug)]
pub struct TypeParam<'db> {
    pub ident: Symbol<'db>,
    pub bounds: Vec<TraitBound<'db>>,
    pub default: Option<Ty<'db>>,
}

#[salsa::tracked(debug)]
pub struct ConstParam<'db> {
    pub ident: Symbol<'db>,
    pub ty: Ty<'db>,
    pub default: Option<Ty<'db>>,
}

#[salsa::tracked(debug)]
pub struct TraitBound<'db> {}

#[salsa::tracked(debug)]
pub struct Def<'db> {
    pub kind: DefKind<'db>,
}

/// Definitions that are associated with module trees.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Update)]
pub enum DefKind<'db> {
    Const(Const<'db>),
    Function(Function<'db>),
    Type(Type<'db>),
    Trait(Trait<'db>),
    Module(Module<'db>),
    Impl(Impl<'db>),
    TraitImpl(TraitImpl<'db>),
}

/// A constant definition.
#[salsa::tracked(debug)]
pub struct Const<'db> {
    pub vis: Visibility<'db>,
    pub name: Symbol<'db>,
    pub ty: Ty<'db>,
    pub init: Expr<'db>,
}

#[salsa::interned(debug)]
pub struct RecieverArg<'db> {
    pub reference: bool,
    pub mutable: bool,
    pub ty: Option<Ty<'db>>,
}

#[salsa::tracked(debug)]
pub struct TypedArg<'db> {
    pub pat: Pat<'db>,
    pub ty: Ty<'db>,
}

/// An argument in a function signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Update)]
pub enum FnArg<'db> {
    /// The self parameter of an associated method.
    Reciever(RecieverArg<'db>),
    Typed(TypedArg<'db>),
}

/// A function signature of a declaration or implementation.
#[salsa::interned(debug)]
pub struct Signature<'db> {
    pub ident: Symbol<'db>,
    pub input: Vec<FnArg<'db>>,
    pub output: Ty<'db>,
}

/// The implementation kind of a function.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum FnImpl<'db> {
    /// User-defined arbitrary HIR code.
    User(Body<'db>),
    // Special MIR intrinsic that usually lowers down to inline bytecode.
    // Intrinsic(lir::instr::Intrinsic),
}

/// A function definition.
#[salsa::tracked(debug)]
pub struct Function<'db> {
    pub vis: Visibility<'db>,
    pub name: Symbol<'db>,
    pub signature: Signature<'db>,
    pub body: FnImpl<'db>,
}

/// A type definition.
#[salsa::tracked(debug)]
pub struct Type<'db> {
    pub vis: Visibility<'db>,
    pub name: Symbol<'db>,
    pub ty: Ty<'db>,
}

/// An definition associated with a trait.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum TraitAssoc<'db> {
    Const(TraitAssocConst<'db>),
    Type(TraitAssocType<'db>),
    Function(TraitAssocFunction<'db>),
}

/// A constant associated with a trait.
#[salsa::tracked(debug)]
pub struct TraitAssocConst<'db> {
    pub name: Symbol<'db>,
    pub ty: Ty<'db>,
    pub default: Option<Expr<'db>>,
}

/// A type associated with a trait.
#[salsa::tracked(debug)]
pub struct TraitAssocType<'db> {
    pub name: Symbol<'db>,
    pub generics: Generics<'db>,
    pub bounds: Vec<TraitBound<'db>>,
    pub ty: Option<Ty<'db>>,
}

/// A function associated with a trait.
#[salsa::tracked(debug)]
pub struct TraitAssocFunction<'db> {
    pub signature: Signature<'db>,
    pub generics: Generics<'db>,
    pub default: Option<Body<'db>>,
}

/// A trait definition.
#[salsa::tracked(debug)]
pub struct Trait<'db> {
    pub vis: Visibility<'db>,
    pub name: Symbol<'db>,
    pub generics: Generics<'db>,
    pub assoc: Vec<TraitAssoc<'db>>,
}

/// A module definition.
#[salsa::tracked(debug)]
pub struct Module<'db> {
    pub vis: Visibility<'db>,
    pub name: Symbol<'db>,
    pub children: Vec<Def<'db>>,
}

/// A definition associated with an inherent implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum ImplAssoc<'db> {
    Const(Const<'db>),
    Type(Type<'db>),
    Function(Function<'db>),
}

/// An inherent implementation.
#[salsa::tracked(debug)]
pub struct Impl<'db> {
    pub generics: Generics<'db>,
    pub ty: Ty<'db>,
    pub assoc: Vec<ImplAssoc<'db>>,
}

/// A definition associated with a trait implementation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Update)]
pub enum TraitImplAssoc<'db> {
    Const(Const<'db>),
    Type(Type<'db>),
    Function(Function<'db>),
}

/// A trait implementation.
#[salsa::tracked(debug)]
pub struct TraitImpl<'db> {
    pub generics: Generics<'db>,
    pub trait_: Trait<'db>,
    pub ty: Ty<'db>,
    pub assoc: Vec<TraitImplAssoc<'db>>,
}

/// Queries the visibility of the given definition.
#[salsa::tracked]
pub fn get_vis<'db>(db: &'db dyn Db, def: Def<'db>) -> Visibility<'db> {
    match def.kind(db) {
        DefKind::Const(def) => def.vis(db),
        DefKind::Type(def) => def.vis(db),
        DefKind::Function(def) => def.vis(db),
        DefKind::Trait(def) => def.vis(db),
        DefKind::Module(def) => def.vis(db),
        DefKind::Impl(_) | DefKind::TraitImpl(_) => Visibility::Pub,
    }
}

/// Queries the name of the given definition.
#[salsa::tracked]
pub fn get_name<'db>(db: &'db dyn Db, def: Def<'db>) -> Option<Symbol<'db>> {
    let symbol = match def.kind(db) {
        DefKind::Const(def) => def.name(db),
        DefKind::Type(def) => def.name(db),
        DefKind::Function(def) => def.name(db),
        DefKind::Trait(def) => def.name(db),
        DefKind::Module(def) => def.name(db),
        _ => return None,
    };
    Some(symbol)
}

/// Queries the descendant definitions of the given module.
#[salsa::tracked]
pub fn get_descendants<'db>(db: &'db dyn Db, module: Module<'db>) -> Vec<Def<'db>> {
    let mut descs = Vec::new();

    for child in module.children(db) {
        descs.push(child);

        if let DefKind::Module(child_mod) = child.kind(db) {
            let inner_descs = get_descendants(db, child_mod);
            descs.extend_from_slice(inner_descs.as_slice())
        }
    }

    descs
}

/// Queries all available inherent implementations to the given type.
#[salsa::tracked]
pub fn get_inherent_impls<'db>(
    db: &'db dyn Db,
    module: Module<'db>,
    ty: Ty<'db>,
) -> Vec<Impl<'db>> {
    let candidates = {
        let core = get_core_module(db);
        get_descendants(db, core)
    };

    candidates
        .iter()
        .chain(module.children(db).iter())
        .filter_map(|def| match def.kind(db) {
            DefKind::Impl(inh_impl) if inh_impl.ty(db) == ty => Some(inh_impl),
            _ => None,
        })
        .collect_vec()
}

/// Queries all available implementations of the given trait.
#[salsa::tracked]
pub fn get_trait_impls<'db>(
    db: &'db dyn Db,
    module: Module<'db>,
    trait_: Trait<'db>,
) -> Vec<TraitImpl<'db>> {
    let candidates = {
        let core = get_core_module(db);
        get_descendants(db, core)
    };

    candidates
        .iter()
        .chain(module.children(db).iter())
        .filter_map(|def| match def.kind(db) {
            DefKind::TraitImpl(trait_impl) if trait_impl.trait_(db) == trait_ => Some(trait_impl),
            _ => None,
        })
        .collect_vec()
}
