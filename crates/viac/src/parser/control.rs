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
    control::{self, Control},
    stmt::Stmt,
};

impl Parser<'_> {
    fn parse_control_return(&mut self) -> Result<Node<control::Return>> {
        self.with_context(Context::ControlReturn, |parser| {
            let first = expect_one!(parser, KwReturn)?;
            let expr = parser
                .is_expr_start()
                .then(|| parser.parse_expr())
                .transpose()?;

            let last = match &expr {
                Some(e) => e.span.clone(),
                _ => first.span.clone(),
            };

            optional!(parser, Semi);

            Ok(Node {
                node: control::Return {
                    expr: expr.map(Into::into),
                },
                span: SourceSpan::new(first.span.begin, last.end),
                attrs: None,
            })
        })
    }

    fn parse_control_raise(&mut self) -> Result<Node<control::Raise>> {
        self.with_context(Context::ControlRaise, |parser| {
            let first = expect_one!(parser, KwRaise)?.span;
            let expr = parser.parse_expr()?;
            let last = expr.span.clone();

            optional!(parser, Semi);
            Ok(Node {
                node: control::Raise { expr: expr.into() },
                span: SourceSpan::new(first.begin, last.end),
                attrs: None,
            })
        })
    }

    fn parse_control_if(&mut self) -> Result<Node<control::If>> {
        self.push_context(Context::ControlIf);

        let first = expect_one!(self, KwIf)?.span;
        let cond = self.parse_expr()?;

        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let mut last = body.span.clone();
        let mut elseif = vec![];

        while check!(self, KwElse) && check!(self, KwIf, 1) {
            self.push_context(Context::ControlElseIf);
            self.consume()?;
            self.consume()?;

            let cond = self.parse_expr()?;
            self.pop_context();

            let body = self.parse_body(Self::parse_stmt)?;
            last = body.span.clone();
            elseif.push((cond, body));
        }

        let else_body = check!(self, KwElse)
            .then(|| -> Result<Nodes<Stmt>> {
                self.consume()?;
                let body = self.parse_body(Self::parse_stmt)?;
                last = body.span.clone();
                Ok(body)
            })
            .transpose()?;

        Ok(Node {
            node: control::If {
                cond: cond.into(),
                body,
                elseif,
                else_body,
            },
            span: SourceSpan::new(first.begin, last.end),
            attrs: None,
        })
    }

    fn parse_control_while(&mut self) -> Result<Node<control::While>> {
        self.push_context(Context::ControlWhile);
        let first = expect_one!(self, KwWhile)?.span;
        let cond = self.parse_expr()?;
        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let last = body.span.clone();

        Ok(Node {
            node: control::While {
                cond: cond.into(),
                body,
            },
            span: SourceSpan::new(first.begin, last.end),
            attrs: None,
        })
    }

    fn parse_control_for(&mut self) -> Result<Node<control::For>> {
        self.push_context(Context::ControlFor);

        let first = expect_one!(self, KwFor)?.span;
        let param = expect_one!(self, Ident)?;
        let ty = check!(self, Col)
            .then(|| {
                self.consume()?;
                self.parse_type(AllowRaiseClause::No)
            })
            .transpose()?;

        expect_one!(self, KwIn)?;

        let expr = self.parse_expr()?;
        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let last = body.span.clone();

        Ok(Node {
            node: control::For {
                param: (param, ty.map(Into::into)),
                expr: expr.into(),
                body,
            },
            span: SourceSpan::new(first.begin, last.end),
            attrs: None,
        })
    }

    pub(crate) fn parse_control(&mut self) -> Result<Node<Control>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwBreak => self.consume().map(|token| Node {
                    node: control::Break {}.into(),
                    span: token.span,
                    attrs: None,
                }),
                KwContinue => self.consume().map(|token| Node {
                    node: control::Continue {}.into(),
                    span: token.span,
                    attrs: None,
                }),
                KwReturn => self.parse_control_return().map(Node::recast),
                KwRaise => self.parse_control_raise().map(Node::recast),
                KwWhile => self.parse_control_while().map(Node::recast),
                KwFor => self.parse_control_for().map(Node::recast),
                KwIf => self.parse_control_if().map(Node::recast),
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
