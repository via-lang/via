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
use super::ty::AllowEffect;
use crate::ast::control::{self, Control};
use crate::ast::node::IntoNode;

impl Parser {
    fn parse_control_return(&mut self) -> Result<Node<control::Return>> {
        self.with_context(Context::ControlReturn, |p| {
            let first = expect_token!(p, KwReturn)?;
            let expr = p.is_expr_start().then(|| p.parse_expr()).transpose()?;
            let last = match &expr {
                Some(e) => e.span,
                _ => first.span,
            };

            optional_token!(p, Semi);

            Ok(Node {
                node: control::Return {
                    expr: expr.map(Into::into),
                },
                span: span![first.span.begin, last.end],
                attrs: vec![],
            })
        })
    }

    fn parse_control_raise(&mut self) -> Result<Node<control::Raise>> {
        self.with_context(Context::ControlRaise, |p| {
            let first = expect_token!(p, KwRaise)?.span;
            let expr = p.parse_expr()?;
            let last = expr.span;

            optional_token!(p, Semi);

            Ok(Node {
                node: control::Raise { expr: expr.into() },
                span: span![first.begin, last.end],
                attrs: vec![],
            })
        })
    }

    fn parse_control_if(&mut self) -> Result<Node<control::If>> {
        self.push_context(Context::ControlIf);

        let first = expect_token!(self, KwIf)?.span;
        let cond = self.parse_expr()?;

        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let mut last = body.span;
        let mut elseif = vec![];

        while check_token!(self, KwElse) && check_token!(self, KwIf, 1) {
            self.push_context(Context::ControlElseIf);
            self.consume()?;
            self.consume()?;

            let cond = self.parse_expr()?;
            self.pop_context();

            let body = self.parse_body(Self::parse_stmt)?;
            last = body.span;
            elseif.push((cond, body));
        }

        let else_body = check_token!(self, KwElse)
            .then(|| {
                self.consume()?;
                let body = self.parse_body(Self::parse_stmt)?;
                last = body.span;
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
            span: span![first.begin, last.end],
            attrs: vec![],
        })
    }

    fn parse_control_while(&mut self) -> Result<Node<control::While>> {
        self.push_context(Context::ControlWhile);
        let first = expect_token!(self, KwWhile)?.span;
        let cond = self.parse_expr()?;
        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let last = body.span;

        Ok(Node {
            node: control::While {
                cond: cond.into(),
                body,
            },
            span: span![first.begin, last.end],
            attrs: vec![],
        })
    }

    fn parse_control_for(&mut self) -> Result<Node<control::For>> {
        self.push_context(Context::ControlFor);

        let first = expect_token!(self, KwFor)?.span;
        let param = expect_token!(self, Ident)?;
        let ty = check_token!(self, Col)
            .then(|| {
                self.consume()?;
                self.parse_type(AllowEffect::No)
            })
            .transpose()?;

        expect_token!(self, KwIn)?;

        let expr = self.parse_expr()?;
        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let last = body.span;

        Ok(Node {
            node: control::For {
                param: (param, ty.map(Into::into)),
                expr: expr.into(),
                body,
            },
            span: span![first.begin, last.end],
            attrs: vec![],
        })
    }

    pub(crate) fn parse_control(&mut self) -> Result<Node<Control>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwBreak => self.consume().map(|token| Node {
                    node: control::Break {}.into(),
                    span: token.span,
                    attrs: vec![],
                }),
                KwContinue => self.consume().map(|token| Node {
                    node: control::Continue {}.into(),
                    span: token.span,
                    attrs: vec![],
                }),
                KwReturn => self.parse_control_return().map(IntoNode::into_node),
                KwRaise => self.parse_control_raise().map(IntoNode::into_node),
                KwWhile => self.parse_control_while().map(IntoNode::into_node),
                KwFor => self.parse_control_for().map(IntoNode::into_node),
                KwIf => self.parse_control_if().map(IntoNode::into_node),
                _ => self.error(ErrorKind::UnexpectedToken {
                    exp: vec![].into(),
                    got: token,
                }),
            }
        } else {
            self.error(ErrorKind::UnexpectedEndOfFile)
        }
    }
}
