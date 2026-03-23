pub mod typeck;

pub mod prelude {
    pub use super::{
        super::{Hir, HirBuilder, error::*, expr::Expr, stmt::Stmt},
        Pass,
    };
}

use prelude::*;

use crate::sema::context::SemContext;

pub trait Pass {
    fn run(&mut self, sem: &mut SemContext, hir: &mut Hir) -> Result<()>;
}
