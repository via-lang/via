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
    body::Body,
    stmt::{Stmt, StmtId},
};

pub use super::stmt::Allow;

yes_or_no!(pub ExpectBraces);

impl Parser<'_> {
    pub(super) fn parse_body(
        &mut self,
        tree: &mut Tree,
        expect_braces: ExpectBraces,
        allow: Allow,
    ) -> Result<Body> {
        let first = if expect_braces.into() {
            expect_one!(self, LBrace)?.span
        } else {
            self.peek()?.span
        };

        let mut inner = vec![];
        let mut tail = None;

        let last = loop {
            let stmt = self.parse_stmt(tree, allow)?;

            if let Stmt::Consume(c) = stmt {
                tail = Some(c.expr);
            } else {
                let id = tree.insert::<StmtId>(stmt);
                inner.push(id);
            }

            match self.peek()?.kind {
                EndOfFile => {}
                RBrace if expect_braces.into() => {}
                _ => continue,
            }
            break self.consume()?.span;
        };

        Ok(Body {
            inner: inner.into_boxed_slice(),
            span: SourceSpan::merge(first, last),
            tail,
        })
    }
}
