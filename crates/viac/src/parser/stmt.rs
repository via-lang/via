/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use bitflags::bitflags;

use super::{body::ExpectBraces, param::*, prelude::*, ty::AllowRaiseClause};
use crate::ast::{
    Tree,
    stmt::{self, Stmt},
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Allow: u8 {
        const Const = 1 << 0;
        const Type  = 1 << 1;
        const Fn    = 1 << 2;
        const Expr  = 1 << 3;
        const Def   = Self::Const.bits()
                    | Self::Type.bits()
                    | Self::Fn.bits();
    }
}

impl Parser<'_> {
    fn parse_stmt_const(&mut self, tree: &mut Tree) -> Result<stmt::DefineConst> {
        let first = expect_one!(self, KwConst)?;
        let name = expect_one!(self => Ident { .. })?;
        expect_one!(self, Col)?;

        let ty = self.parse_type(tree, AllowRaiseClause::No)?;
        expect_one!(self, Eq)?;

        let expr = self.parse_expr(tree)?;
        let semi = expect_one!(self, Semi)?;

        Ok(stmt::DefineConst {
            name,
            ty: tree.insert(ty),
            expr: tree.insert(expr),
            span: SourceSpan::merge(first.span, semi.span),
        })
    }

    fn parse_stmt_type(&mut self, tree: &mut Tree) -> Result<stmt::DefineType> {
        let first = expect_one!(self, KwType)?;
        let name = expect_one!(self => Ident { .. })?;
        expect_one!(self, Eq)?;

        let ty = self.parse_type(tree, AllowRaiseClause::No)?;
        let semi = expect_one!(self, Semi)?;

        Ok(stmt::DefineType {
            name,
            ty: tree.insert(ty),
            span: SourceSpan::merge(first.span, semi.span),
        })
    }

    fn parse_stmt_fn(&mut self, tree: &mut Tree) -> Result<stmt::DefineFn> {
        let first = expect_one!(self, KwFn)?;
        let name = expect_one!(self => Ident { .. })?;

        let params = self.parse_params(tree, OmitEmptyParams::No, AllowNamedParam::Yes)?;
        let result = check!(self, Arrow)
            .then(|| {
                self.consume()?;
                self.parse_return_ty(tree)
            })
            .transpose()?
            .map(|t| tree.insert(t));

        let body = self.parse_body(tree, ExpectBraces::Yes, Allow::all())?;
        let last = body.span;

        Ok(stmt::DefineFn {
            name,
            params,
            result,
            body,
            span: SourceSpan::merge(first.span, last),
        })
    }

    pub(super) fn parse_stmt(&mut self, tree: &mut Tree, alw: Allow) -> Result<Stmt> {
        let allowed = |other: Allow| !(alw & other).is_empty();
        let token = self.peek()?;

        match token.kind {
            KwFn if allowed(Allow::Fn) => self.parse_stmt_fn(tree).map(Into::into),
            KwType if allowed(Allow::Type) => self.parse_stmt_type(tree).map(Into::into),
            KwConst if allowed(Allow::Const) => self.parse_stmt_const(tree).map(Into::into),
            _ if allowed(Allow::Expr) && self.is_expr_start() => {
                let proto = self.parse_expr(tree)?;
                let span = proto.span();
                let expr = tree.insert(proto);

                Ok(if optional!(self, Semi) {
                    stmt::Discard { span, expr }.into()
                } else {
                    stmt::Consume { span, expr }.into()
                })
            }
            _ => Err(Error::UnexpectedToken {
                span: token.span.into(),
                expected: vec![].into(),
                got: self.src.get_span(&token.span).to_owned(),
            }),
        }
    }
}
