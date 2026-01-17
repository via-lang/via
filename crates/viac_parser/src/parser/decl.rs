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
use via_macros::bug;
use viac_ast::decl::{self, Decl};
use viac_ast::node::IntoNode;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum AllowImport {
    Yes,
    No,
}

impl From<AllowImport> for bool {
    fn from(value: AllowImport) -> Self {
        value == AllowImport::Yes
    }
}

impl<'a> Parser<'a> {
    pub(crate) fn parse_decl_variable(&mut self) -> Result<Node<decl::Variable>> {
        self.with_context(Context::DeclVariable, |p| {
            let first = p.expect_consume(KwVar)?.span;
            let symbol = p.expect_consume(Identifier)?;
            let ty = p
                .check(Colon)
                .then(|| {
                    p.consume()?;
                    Ok(p.parse_type()?)
                })
                .transpose()?;

            p.expect_consume(OpEq)?;

            let expr = p.parse_expr()?;
            let last = expr.span;

            Ok(Node {
                node: decl::Variable {
                    symbol: symbol,
                    ty: ty.map(Into::into),
                    expr: expr.into(),
                },
                span: span![first.begin, last.end],
            })
        })
    }

    fn parse_decl_function(&mut self) -> Result<Node<decl::Function>> {
        self.push_context(Context::DeclFunction);

        let first = self.expect_consume(KwFn)?.span;
        let symbol = self.expect_consume(Identifier)?;
        let params = self.with_context(Context::ParamList, |p| {
            p.parse_list((ParenOpen, ParenClose), Self::parse_param)
        })?;

        let result = self
            .check(Arrow)
            .then(|| self.parse_return_ty())
            .transpose()?;

        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let last = body.span;

        Ok(Node {
            node: decl::Function {
                symbol: symbol,
                params,
                result: result.map(IntoNode::into_node).map(Into::into),
                body,
            },
            span: span![first.begin, last.end],
        })
    }

    fn parse_decl_use(&mut self) -> Result<Node<decl::Use>> {
        todo!()
    }

    fn parse_decl_type(&mut self) -> Result<Node<decl::Type>> {
        self.with_context(Context::DeclType, |p| {
            let begin = p.expect_consume(KwType)?;
            let symbol = p.expect_consume(Identifier)?;
            p.expect_consume(OpEq)?;

            let ty = p.parse_type()?;
            let last = ty.span;

            Ok(Node {
                node: decl::Type {
                    symbol,
                    ty: ty.into(),
                },
                span: span![begin.span.begin, last.end],
            })
        })
    }

    fn parse_decl_const(&mut self) -> Result<Node<decl::Const>> {
        self.with_context(Context::DeclConst, |p| {
            let begin = p.expect_consume(KwConst)?;
            let symbol = p.expect_consume(Identifier)?;
            p.expect_consume(OpEq)?;

            let expr = p.parse_expr()?;
            let last = expr.span;

            Ok(Node {
                node: decl::Const {
                    symbol: symbol,
                    expr: expr.into(),
                },
                span: span![begin.span.begin, last.end],
            })
        })
    }

    fn parse_decl_struct(&mut self) -> Result<Node<decl::Struct>> {
        self.push_context(Context::DeclStruct);

        let first = self.expect_consume(KwStruct)?;
        let symbol = self.expect_consume(Identifier)?;

        self.pop_context();

        let body = self.parse_body(|p| p.parse_decl(AllowImport::No))?;
        let last = body.span;

        Ok(Node {
            node: decl::Struct { symbol, body },
            span: span![first.span.begin, last.end],
        })
    }

    fn parse_decl_import(&mut self) -> Result<Node<decl::Import>> {
        self.with_context(Context::DeclImport, |p| {
            let first = p.expect_consume(KwImport)?;
            let mut path = vec![p.expect_consume(Identifier)?];
            while p.check(Period) {
                p.consume()?;
                let token = p.expect_consume(Identifier)?;
                path.push(token);
            }

            let alias = p
                .check(KwAs)
                .then(|| {
                    p.consume()?;
                    Ok(p.expect_consume(Identifier)?)
                })
                .transpose()?;

            let span = span![
                first.span.begin,
                alias
                    .unwrap_or(
                        path.last()
                            .unwrap_or_else(|| bug!("misparsed import path"))
                            .clone()
                    )
                    .span
                    .end
            ];

            Ok(Node {
                node: decl::Import { path, alias },
                span,
            })
        })
    }

    pub(crate) fn parse_decl(&mut self, import_policy: AllowImport) -> Result<Node<Decl>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwVar => self.parse_decl_variable().map(IntoNode::into_node),
                KwFn => self.parse_decl_function().map(IntoNode::into_node),
                KwUse => self.parse_decl_use().map(IntoNode::into_node),
                KwType => self.parse_decl_type().map(IntoNode::into_node),
                KwConst => self.parse_decl_const().map(IntoNode::into_node),
                KwStruct => self.parse_decl_struct().map(IntoNode::into_node),
                KwImport if import_policy.into() => {
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
