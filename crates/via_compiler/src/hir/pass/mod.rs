pub mod zonk;

pub(super) mod prelude {
    pub use super::{
        super::{Hir, error::*, expr::Expr, stmt::Stmt},
        Pass,
    };
}

use prelude::*;

use crate::sema::SemContext;

pub trait Pass {
    fn run(&mut self, sem_ctxt: &mut SemContext, hir: &mut Hir) -> Result<()>;
}
