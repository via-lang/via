/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::attr::{self, Attr};

impl Parser<'_> {
    pub(crate) fn parse_attr(&mut self) -> Result<Node<Attr>> {
        self.with_context(Context::Attr, |parser| {
            let first = expect_one!(parser, Hash)?;
            let name = expect_one!(parser, Ident)?;
            let span = SourceSpan::merge(first.span, name.span.clone());
            let slice = parser.src.get_span(name.span);
            match slice {
                "native" => Ok(Node::new(attr::Native {}, span, None)),
                "inline" => Ok(Node::new(attr::Inline {}, span, None)),
                "distinct" => Ok(Node::new(attr::Distinct {}, span, None)),
                _ => Err(Error::UnexpectedToken {
                    src: parser.src.clone(),
                    span: span.to_miette_span(),
                    expected: vec!["native", "inline", "distinct"].into(),
                    got: slice.to_string(),
                }),
            }
        })
    }

    #[allow(dead_code)]
    pub(crate) fn parse_attrs(&mut self) -> Result<Vec<Node<Attr>>> {
        check!(self, Hash)
            .then(|| self.parse_attr().map(|a| vec![a]))
            .unwrap_or(Ok(Vec::new()))
    }
}
