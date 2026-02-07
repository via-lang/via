/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt;

#[derive(Debug)]
pub enum ConstValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

impl fmt::Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(fp) => write!(f, "{fp}"),
            Self::String(s) => write!(f, "\"{s}\""),
        }
    }
}
