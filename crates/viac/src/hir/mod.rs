pub mod builder;
pub mod error;
pub mod expr;
pub mod pass;
pub mod stmt;
pub mod ty;

use via_macros::Arena;

pub use builder::*;
pub use error::*;

use expr::Expr;
use stmt::Stmt;

use crate::node::NodeId;

#[derive(Arena, Debug, Default)]
pub struct Hir {
    #[allocator]
    expr: Vec<Expr>,
    #[allocator]
    stmt: Vec<Stmt>,
    pub roots: Vec<NodeId<Stmt>>,
}
