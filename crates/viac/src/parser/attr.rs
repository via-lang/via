/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::{
    Tree,
    attr::{self, AttrId},
};

impl Parser<'_> {
    pub(crate) fn parse_attr(&mut self, tree: &mut Tree) -> Result<AttrId> {
        self.with_context(Context::Attr, |parser| {
            let first = expect_one!(parser, Hash)?;
            let name = expect_one!(parser, Ident)?;

            let span = SourceSpan::merge(first.span, name.span.clone());
            let slice = parser.src.get_span(name.span);

            let attr = match slice {
                "native" => attr::Native { span }.into(),
                "inline" => attr::Inline { span }.into(),
                "distinct" => attr::Distinct { span }.into(),
                _ => {
                    return Err(Error::UnexpectedToken {
                        src: parser.src.clone(),
                        span: span.to_miette_span(),
                        expected: vec!["native", "inline", "distinct"].into(),
                        got: slice.to_string(),
                    });
                }
            };

            Ok(tree.insert(attr))
        })
    }
}
