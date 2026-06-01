use crate::{ast::Expr, node::NodeId, source::SourceSpan};

#[derive(Debug)]
pub enum TyKind {
    Unit,
    Bool,
    Int,
    Float,
    Array { ty: NodeId<Ty>, size: NodeId<Expr> },
    Vector(NodeId<Ty>),
}

#[derive(Debug)]
pub struct Ty {
    pub kind: TyKind,
    pub span: SourceSpan,
}
