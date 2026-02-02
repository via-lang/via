/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{prelude::*, ty::AllowRaiseClause};
use crate::ast::{
    Tree,
    aux::Nodes,
    control::{self, ControlId},
    stmt::StmtId,
};

impl Parser<'_> {
    fn parse_control_return(&mut self, tree: &mut Tree) -> Result<control::Return> {
        self.with_context(Context::ControlReturn, |parser| {
            let first = expect_one!(parser, KwReturn)?.span;
            let expr = parser
                .is_expr_start()
                .then(|| parser.parse_expr(tree))
                .transpose()?;

            let last = expr.map(|e| tree.get(e).span()).unwrap_or(first.clone());

            optional!(parser, Semi);

            Ok(control::Return {
                span: SourceSpan::merge(first, last),
                expr: expr.map(Into::into),
            })
        })
    }

    fn parse_control_raise(&mut self, tree: &mut Tree) -> Result<control::Raise> {
        self.with_context(Context::ControlRaise, |parser| {
            let first = expect_one!(parser, KwRaise)?.span;
            let expr = parser.parse_expr(tree)?;
            let last = tree.get(expr).span();

            optional!(parser, Semi);
            Ok(control::Raise {
                span: SourceSpan::new(first.begin, last.end),
                expr: expr.into(),
            })
        })
    }

    fn parse_control_if(&mut self, tree: &mut Tree) -> Result<control::If> {
        self.push_context(Context::ControlIf);

        let first = expect_one!(self, KwIf)?.span;
        let cond = self.parse_expr(tree)?;

        self.pop_context();

        let body = self.parse_body(tree, Self::parse_stmt)?;
        let mut last = body.span.clone();
        let mut elseif = vec![];

        while check!(self, KwElse) && check!(self, KwIf, 1) {
            self.push_context(Context::ControlElseIf);
            self.consume()?;
            self.consume()?;

            let cond = self.parse_expr(tree)?;
            self.pop_context();

            let body = self.parse_body(tree, Self::parse_stmt)?;
            last = body.span.clone();
            elseif.push((cond, body));
        }

        let else_body = check!(self, KwElse)
            .then(|| -> Result<Nodes<StmtId>> {
                self.consume()?;
                let body = self.parse_body(tree, Self::parse_stmt)?;
                last = body.span.clone();
                Ok(body)
            })
            .transpose()?;

        Ok(control::If {
            span: SourceSpan::new(first.begin, last.end),
            cond: cond.into(),
            body,
            elseif,
            else_body,
        })
    }

    fn parse_control_while(&mut self, tree: &mut Tree) -> Result<control::While> {
        self.push_context(Context::ControlWhile);

        let first = expect_one!(self, KwWhile)?.span;
        let cond = self.parse_expr(tree)?;

        self.pop_context();

        let body = self.parse_body(tree, Self::parse_stmt)?;
        let last = body.span.clone();

        Ok(control::While {
            span: SourceSpan::new(first.begin, last.end),
            cond: cond.into(),
            body,
        })
    }

    fn parse_control_for(&mut self, tree: &mut Tree) -> Result<control::For> {
        self.push_context(Context::ControlFor);

        let first = expect_one!(self, KwFor)?.span;
        let param = expect_one!(self, Ident)?;
        let ty = check!(self, Col)
            .then(|| {
                self.consume()?;
                self.parse_type(tree, AllowRaiseClause::No)
            })
            .transpose()?;

        expect_one!(self, KwIn)?;

        let expr = self.parse_expr(tree)?;

        self.pop_context();

        let body = self.parse_body(tree, Self::parse_stmt)?;
        let last = body.span.clone();

        Ok(control::For {
            span: SourceSpan::new(first.begin, last.end),
            param: (param, ty.map(Into::into)),
            expr: expr.into(),
            body,
        })
    }

    pub(crate) fn parse_control(&mut self, tree: &mut Tree) -> Result<ControlId> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwBreak => self
                    .consume()
                    .map(|token| control::Break { span: token.span }.into())
                    .map(|c| tree.insert(c)),
                KwContinue => self
                    .consume()
                    .map(|token| control::Continue { span: token.span }.into())
                    .map(|c| tree.insert(c)),
                KwReturn => self
                    .parse_control_return(tree)
                    .map(Into::into)
                    .map(|c| tree.insert(c)),
                KwRaise => self
                    .parse_control_raise(tree)
                    .map(Into::into)
                    .map(|c| tree.insert(c)),
                KwWhile => self
                    .parse_control_while(tree)
                    .map(Into::into)
                    .map(|c| tree.insert(c)),
                KwFor => self
                    .parse_control_for(tree)
                    .map(Into::into)
                    .map(|c| tree.insert(c)),
                KwIf => self
                    .parse_control_if(tree)
                    .map(Into::into)
                    .map(|c| tree.insert(c)),
                _ => Err(Error::UnexpectedToken {
                    src: self.src.clone(),
                    span: token.span.to_miette_span(),
                    expected: vec![].into(),
                    got: self.src.get_span(token.span).to_owned(),
                }),
            }
        } else {
            Err(Error::UnexpectedEndOfFile {
                src: self.src.clone(),
            })
        }
    }
}
