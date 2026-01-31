/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use assert_matches::assert_matches;

use super::super::prelude::*;
use crate::ast::attr::Attr;

pub fn parse_attr(src: &str) -> Result<Attr> {
    super::parse(src, |parser| parser.parse_attr().map(|a| a.node))
}

#[test]
fn attr_primitive() {
    assert_matches!(parse_attr("#inline"), Ok(Attr::Inline(_)));
    assert_matches!(parse_attr("#native"), Ok(Attr::Native(_)));
}
