mod expr;
mod stmt;
mod ty;

use via_macros::Arena;

use crate::node::NodeId;

pub use {expr::*, stmt::*, ty::*};

#[derive(Arena, Debug, Default)]
pub struct Tree {
    #[allocator]
    stmt: Vec<Stmt>,
    #[allocator]
    expr: Vec<Expr>,
    #[allocator]
    ty: Vec<Ty>,
    pub roots: Vec<NodeId<Stmt>>,
}
