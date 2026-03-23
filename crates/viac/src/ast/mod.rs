pub mod expr;
pub mod stmt;
pub mod ty;

use via_macros::Arena;

use crate::node::NodeId;

use expr::Expr;
use stmt::Stmt;
use ty::Ty;

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
