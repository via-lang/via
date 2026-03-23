use super::{expr::Expr, ty::Ty};
use crate::{node::NodeId, source::SourceSpan};

#[derive(Debug)]
pub enum StmtKind {
    Let {
        ident: String,
        ty: Option<NodeId<Ty>>,
        expr: NodeId<Expr>,
    },
    Discard(NodeId<Expr>),
    Consume(NodeId<Expr>),
}

#[derive(Debug)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: SourceSpan,
}
