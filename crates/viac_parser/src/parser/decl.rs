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
use crate::macros::yes_or_no;
use via_macros::bug;
use viac_ast::decl::{self, Decl};
use viac_ast::node::IntoNode;

yes_or_no!(pub AllowImport);

impl<'a> Parser<'a> {
    pub(crate) fn parse_decl_variable(&mut self) -> Result<Node<decl::Variable>> {
        self.with_context(Context::DeclVariable, |p| {
            let first = expect_token!(p, KwVar)?.span;
            let symbol = expect_token!(p, Identifier(_))?;
            let ty = check_token!(p, Colon)
                .then(|| {
                    p.consume()?;
                    Ok(p.parse_type()?)
                })
                .transpose()?;

            expect_token!(p, OpEq)?;

            let expr = p.parse_expr()?;
            let last = expr.span;

            optional_token!(p, Semicolon);

            Ok(Node {
                node: decl::Variable {
                    symbol: symbol,
                    ty: ty.map(Into::into),
                    expr: expr.into(),
                },
                span: span![first.begin, last.end],
                attrs: vec![],
            })
        })
    }

    fn parse_decl_function(&mut self) -> Result<Node<decl::Function>> {
        self.push_context(Context::DeclFunction);

        let first = expect_token!(self, KwFn)?.span;
        let symbol = expect_token!(self, Identifier(_))?;
        let params = self.with_context(Context::ParamList, |p| {
            p.parse_list((ParenOpen, ParenClose), Self::parse_param)
        })?;

        let result = self.parse_return_ty()?;

        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let last = body.span;

        Ok(Node {
            node: decl::Function {
                symbol: symbol,
                params,
                result: result.map(Into::into),
                body,
            },
            span: span![first.begin, last.end],
            attrs: vec![],
        })
    }

    fn parse_decl_use(&mut self) -> Result<Node<decl::Use>> {
        todo!()
    }

    fn parse_decl_type(&mut self) -> Result<Node<decl::Type>> {
        self.with_context(Context::DeclType, |p| {
            let begin = expect_token!(p, KwType)?;
            let symbol = expect_token!(p, Identifier(_))?;
            expect_token!(p, OpEq)?;

            let ty = p.parse_type()?;
            let last = ty.span;

            optional_token!(p, Semicolon);

            Ok(Node {
                node: decl::Type {
                    symbol,
                    ty: ty.into(),
                },
                span: span![begin.span.begin, last.end],
                attrs: vec![],
            })
        })
    }

    fn parse_decl_const(&mut self) -> Result<Node<decl::Const>> {
        self.with_context(Context::DeclConst, |p| {
            let begin = expect_token!(p, KwConst)?;
            let symbol = expect_token!(p, Identifier(_))?;
            expect_token!(p, OpEq)?;

            let expr = p.parse_expr()?;
            let last = expr.span;

            optional_token!(p, Semicolon);

            Ok(Node {
                node: decl::Const {
                    symbol: symbol,
                    expr: expr.into(),
                },
                span: span![begin.span.begin, last.end],
                attrs: vec![],
            })
        })
    }

    fn parse_decl_struct(&mut self) -> Result<Node<decl::Struct>> {
        self.push_context(Context::DeclStruct);

        let first = expect_token!(self, KwStruct)?;
        let symbol = expect_token!(self, Identifier(_))?;

        self.pop_context();

        let body = self.parse_body(|p| p.parse_decl(AllowImport::No))?;
        let last = body.span;

        Ok(Node {
            node: decl::Struct { symbol, body },
            span: span![first.span.begin, last.end],
            attrs: vec![],
        })
    }

    fn parse_decl_import(&mut self) -> Result<Node<decl::Import>> {
        self.with_context(Context::DeclImport, |p| {
            let first = expect_token!(p, KwImport)?;
            let mut path = vec![expect_token!(p, Identifier(_))?];
            while check_token!(p, Period) {
                p.consume()?;
                let token = expect_token!(p, Identifier(_))?;
                path.push(token);
            }

            let alias = check_token!(p, KwAs)
                .then(|| {
                    p.consume()?;
                    Ok(expect_token!(p, Identifier(_))?)
                })
                .transpose()?;

            let last = alias.clone().map(|t| t.span).unwrap_or_else(|| {
                path.last()
                    .unwrap_or_else(|| bug!("misparsed import path"))
                    .span
            });

            Ok(Node {
                node: decl::Import { path, alias },
                span: span![first.span.begin, last.end],
                attrs: vec![],
            })
        })
    }

    pub(crate) fn parse_decl(&mut self, allow_import: AllowImport) -> Result<Node<Decl>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwVar => self.parse_decl_variable().map(IntoNode::into_node),
                KwFn => self.parse_decl_function().map(IntoNode::into_node),
                KwUse => self.parse_decl_use().map(IntoNode::into_node),
                KwType => self.parse_decl_type().map(IntoNode::into_node),
                KwConst => self.parse_decl_const().map(IntoNode::into_node),
                KwStruct => self.parse_decl_struct().map(IntoNode::into_node),
                KwImport if allow_import.into() => {
                    self.parse_decl_import().map(IntoNode::into_node)
                }
                _ => self.error(ErrorKind::UnexpectedToken {
                    expected: vec![],
                    got: token,
                }),
            }
        } else {
            self.error(ErrorKind::UnexpectedEndOfFile)
        }
    }
}
