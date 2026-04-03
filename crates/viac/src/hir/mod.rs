pub mod builder;
pub mod error;
pub mod expr;
pub mod pass;
pub mod stmt;
pub mod ty;

use std::fmt;

use via_macros::Arena;

pub use builder::*;
pub use error::*;

use expr::Expr;
use stmt::Stmt;

use crate::node::NodeId;

#[derive(Arena, Default)]
pub struct Hir {
    #[allocator]
    expr: Vec<Expr>,
    #[allocator]
    stmt: Vec<Stmt>,
    pub roots: Vec<NodeId<Stmt>>,
}

impl fmt::Debug for Hir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Hir")
            .field(
                "roots",
                &self
                    .roots
                    .clone()
                    .iter()
                    .map(|root| &self[*root])
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}
