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
use crate::ast::attr::{self, Attr};

impl Parser {
    pub(crate) fn parse_attr(&mut self) -> Result<Node<Attr>> {
        self.with_context(Context::Attr, |p| {
            let first = expect_token!(p, Hash)?;
            let name = expect_token!(p, Ident)?;
            let span = span![first.span.begin, name.span.end];

            match p.src.slice(name.span) {
                "native" => Ok(Node::new(attr::Native {}.into(), span)),
                "inline" => Ok(Node::new(attr::Inline {}.into(), span)),
                "distinct" => Ok(Node::new(attr::Distinct {}.into(), span)),
                _ => p.error(ErrorKind::UnexpectedToken {
                    exp: vec!["native", "inline", "distinct"].into(),
                    got: name,
                }),
            }
        })
    }

    #[allow(dead_code)]
    pub(crate) fn parse_attrs(&mut self) -> Result<Vec<Node<Attr>>> {
        check_token!(self, Hash)
            .then(|| self.parse_attr().map(|a| vec![a]))
            .unwrap_or(Ok(Vec::new()))
    }
}
