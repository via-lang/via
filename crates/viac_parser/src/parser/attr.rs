/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Parser;
use super::prelude::*;
use viac_ast::attr::{self, Attr};

impl<'a> Parser<'a> {
    pub(crate) fn parse_attr(&mut self) -> Result<Node<Attr>> {
        self.with_context(Context::Attr, |p| {
            let first = p.expect_consume(OpHash)?;
            let name = p.expect_consume(Identifier)?;
            let span = span![first.span.begin, name.span.end];

            match p.source.slice(name.span) {
                "native" => Ok(Node::new(attr::Native {}.into(), span)),
                "inline" => Ok(Node::new(attr::Inline {}.into(), span)),
                "distinct" => p.with_context(Context::AttrDistinct, |p| {
                    let first = p.expect_consume(ParenOpen)?;
                    let ty = p.parse_type()?;
                    let last = p.expect_consume(ParenClose)?;

                    Ok(Node {
                        node: attr::Distinct { ty: ty.into() }.into(),
                        span: span![first.span.begin, last.span.end],
                    })
                }),
                _ => p.error(ErrorKind::UnexpectedToken {
                    expected: vec![],
                    got: name,
                }),
            }
        })
    }
}
