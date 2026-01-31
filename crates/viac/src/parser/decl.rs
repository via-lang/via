/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use via_macros::bug;

use super::{prelude::*, ty::AllowRaiseClause};
use crate::{
    ast::decl::{self, Decl},
    lexer::token::Token,
};

yes_or_no!(pub AllowImport);

impl Parser {
    pub(crate) fn parse_decl_variable(&mut self) -> Result<Node<decl::Variable>> {
        self.with_context(Context::DeclVariable, |parser| {
            let first = expect_one!(parser, KwVar)?.span;
            let symbol = expect_one!(parser, Ident)?;
            let ty = check!(parser, Col)
                .then(|| {
                    parser.consume()?;
                    parser.parse_type(AllowRaiseClause::No)
                })
                .transpose()?;

            expect_one!(parser, Eq)?;

            let expr = parser.parse_expr()?;
            let last = expr.span.clone();

            optional!(parser, Semi);

            Ok(Node {
                node: decl::Variable {
                    symbol,
                    ty: ty.map(Into::into),
                    expr: expr.into(),
                },
                span: SourceSpan::new(first.begin, last.end),
                attrs: None,
            })
        })
    }

    fn parse_decl_function(&mut self) -> Result<Node<decl::Function>> {
        self.push_context(Context::DeclFunction);

        let first = expect_one!(self, KwFn)?.span;
        let symbol = expect_one!(self, Ident)?;
        let params = self.with_context(Context::ParamList, |parser| {
            parser.parse_list((LParen, RParen), Self::parse_param)
        })?;

        let result = optional!(self, Arrow)
            .then(|| self.parse_return_ty())
            .transpose()?
            .map(Into::into);

        self.pop_context();

        let body = self.parse_body(Self::parse_stmt)?;
        let last = body.span.clone();

        Ok(Node {
            node: decl::Function {
                symbol,
                params,
                result,
                body,
            },
            span: SourceSpan::new(first.begin, last.end),
            attrs: None,
        })
    }

    fn parse_decl_use(&mut self) -> Result<Node<decl::Use>> {
        todo!()
    }

    fn parse_decl_type(&mut self) -> Result<Node<decl::Type>> {
        self.with_context(Context::DeclType, |parser| {
            let begin = expect_one!(parser, KwType)?;
            let symbol = expect_one!(parser, Ident)?;
            expect_one!(parser, Eq)?;

            let ty = parser.parse_type(AllowRaiseClause::No)?;
            let last = ty.span.clone();

            optional!(parser, Semi);

            Ok(Node {
                node: decl::Type {
                    symbol,
                    ty: ty.into(),
                },
                span: SourceSpan::new(begin.span.begin, last.end),
                attrs: None,
            })
        })
    }

    fn parse_decl_const(&mut self) -> Result<Node<decl::Const>> {
        self.with_context(Context::DeclConst, |parser| {
            let begin = expect_one!(parser, KwConst)?;
            let symbol = expect_one!(parser, Ident)?;
            expect_one!(parser, Eq)?;

            let expr = parser.parse_expr()?;
            let last = expr.span.clone();

            optional!(parser, Semi);

            Ok(Node {
                node: decl::Const {
                    symbol,
                    expr: expr.into(),
                },
                span: SourceSpan::new(begin.span.begin, last.end),
                attrs: None,
            })
        })
    }

    fn parse_decl_struct(&mut self) -> Result<Node<decl::Struct>> {
        self.push_context(Context::DeclStruct);

        let first = expect_one!(self, KwStruct)?;
        let symbol = expect_one!(self, Ident)?;

        self.pop_context();

        let body = self.parse_body(|parser| parser.parse_decl(AllowImport::No))?;
        let last = body.span.clone();

        Ok(Node {
            node: decl::Struct { symbol, body },
            span: SourceSpan::new(first.span.begin, last.end),
            attrs: None,
        })
    }

    fn parse_decl_import(&mut self) -> Result<Node<decl::Import>> {
        self.with_context(Context::DeclImport, |parser| {
            let first = expect_one!(parser, KwImport)?;
            let mut path = vec![expect_one!(parser, Ident)?];
            while check!(parser, Dot) {
                parser.consume()?;
                let token = expect_one!(parser, Ident)?;
                path.push(token);
            }

            let alias = check!(parser, KwAs)
                .then(|| -> Result<Token> {
                    parser.consume()?;
                    Ok(expect_one!(parser, Ident)?)
                })
                .transpose()?;

            let last = alias.clone().map(|t| t.span).unwrap_or_else(|| {
                path.last()
                    .unwrap_or_else(|| bug!("misparsed import path"))
                    .span
                    .clone()
            });

            Ok(Node {
                node: decl::Import { path, alias },
                span: SourceSpan::new(first.span.begin, last.end),
                attrs: None,
            })
        })
    }

    pub(crate) fn parse_decl(&mut self, allow_import: AllowImport) -> Result<Node<Decl>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwVar => self.parse_decl_variable().map(Node::recast),
                KwFn => self.parse_decl_function().map(Node::recast),
                KwUse => self.parse_decl_use().map(Node::recast),
                KwType => self.parse_decl_type().map(Node::recast),
                KwConst => self.parse_decl_const().map(Node::recast),
                KwStruct => self.parse_decl_struct().map(Node::recast),
                KwImport if allow_import.into() => self.parse_decl_import().map(Node::recast),
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
