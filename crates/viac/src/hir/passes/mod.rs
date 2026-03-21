/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

// pub mod infer;
// pub mod typeck;

pub mod prelude {
    pub use super::{
        super::{Hir, HirBuilder, error::*, expr::Expr, stmt::Stmt},
        Pass,
    };
}

use prelude::*;

pub trait Pass<'a> {
    fn new(builder: &'a mut HirBuilder<'a, '_>, hir: &'a Hir) -> Self;
    fn run(&mut self) -> Result<()>;
}
