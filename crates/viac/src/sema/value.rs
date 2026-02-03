/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use derive_more::Display;

#[derive(Display, Debug)]
pub enum ConstValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}
