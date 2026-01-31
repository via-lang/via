/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use assert_matches::assert_matches;

use super::super::{prelude::*, ty::AllowRaiseClause};
use crate::ast::ty::Ty;

pub fn parse_ty(src: &str) -> Result<Ty> {
    super::parse(src, |parser| {
        parser.parse_type(AllowRaiseClause::Yes).map(|t| t.node)
    })
}

#[test]
fn type_builtin() {
    assert_matches!(parse_ty("none"), Ok(Ty::Builtin(_)));
    assert_matches!(parse_ty("bool"), Ok(Ty::Builtin(_)));
    assert_matches!(parse_ty("int"), Ok(Ty::Builtin(_)));
    assert_matches!(parse_ty("float"), Ok(Ty::Builtin(_)));
    assert_matches!(parse_ty("string"), Ok(Ty::Builtin(_)));
}

#[test]
fn type_optional() {
    assert_matches!(parse_ty("int?"), Ok(Ty::Optional(o)) => {
        assert_matches!(*o.ty.node, Ty::Builtin(_));
    });
}
