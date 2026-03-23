use crate::{
    node::NodeId,
    sema::ops::{BinaryOp, UnaryOp},
    source::SourceSpan,
};

#[derive(Debug)]
pub enum PlaceKind {
    Symbol(String),
}

#[derive(Debug)]
pub struct Place {
    pub kind: PlaceKind,
    pub span: SourceSpan,
}

#[derive(Debug)]
pub enum ExprKind {
    None,
    True,
    False,
    Integer(u128),
    Float(f64),
    Unary {
        op: UnaryOp,
        expr: NodeId<Expr>,
    },
    Binary {
        op: BinaryOp,
        lhs: NodeId<Expr>,
        rhs: NodeId<Expr>,
    },
    Read(Place),
}

#[derive(Debug)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: SourceSpan,
}
