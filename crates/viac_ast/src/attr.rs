/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::macros::ast;
use crate::node::Node;
use viac_source::span::Span;

ast! {
    pub enum Attr {
        Native {},
        Inline {},
        Strong {},
        Public {},
        Private {},
        ReadOnly {},
        Fallthrough {},
    }
}
