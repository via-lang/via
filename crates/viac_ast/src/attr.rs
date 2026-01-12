/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::node::Node;
use viac_source::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub enum Attr {
    // Type
    Strong(Span),
    // Struct
    Public(Span),
    Private(Span),
    ReadOnly(Span),
    // Control flow
    Fallthrough(Span),
}

impl Node for Attr {
    fn span(&self) -> Span {
        match self {
            Self::Strong(s) => *s,
            Self::Public(s) => *s,
            Self::Private(s) => *s,
            Self::ReadOnly(s) => *s,
            Self::Fallthrough(s) => *s,
        }
    }
}
