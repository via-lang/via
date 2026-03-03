/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod expr;
pub mod stmt;
pub mod ty;

use stmt::Stmt;

#[derive(Debug)]
pub struct Tree {
    pub inner: Box<[Stmt]>,
}
