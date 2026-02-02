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
    ast::{
        Tree,
        decl::{self, DeclId},
    },
    lexer::token::Token,
};

yes_or_no!(pub AllowImport);

impl Parser<'_> {
    pub(crate) fn parse_decl_variable(&mut self, tree: &mut Tree) -> Result<decl::Variable> {
        self.with_context(Context::DeclVariable, |parser| {
            let first = expect_one!(parser, KwVar)?.span;
            let symbol = expect_one!(parser, Ident)?;
            let ty = check!(parser, Col)
                .then(|| {
                    parser.consume()?;
                    parser.parse_type(tree, AllowRaiseClause::No)
                })
                .transpose()?;

            expect_one!(parser, Eq)?;

            let expr = parser.parse_expr(tree)?;
            let last = tree.get(expr).span();

            optional!(parser, Semi);

            Ok(decl::Variable {
                span: SourceSpan::new(first.begin, last.end),
                symbol,
                ty: ty.map(Into::into),
                expr: expr.into(),
            })
        })
    }

    fn parse_decl_function(&mut self, tree: &mut Tree) -> Result<decl::Function> {
        self.push_context(Context::DeclFunction);

        let first = expect_one!(self, KwFn)?.span;
        let symbol = expect_one!(self, Ident)?;
        let params = self.with_context(Context::ParamList, |parser| {
            parser.parse_list(tree, (LParen, RParen), Self::parse_param)
        })?;

        let result = optional!(self, Arrow)
            .then(|| self.parse_return_ty(tree))
            .transpose()?
            .map(Into::into);

        self.pop_context();

        let body = self.parse_body(tree, Self::parse_stmt)?;
        let last = body.span.clone();

        Ok(decl::Function {
            span: SourceSpan::new(first.begin, last.end),
            symbol,
            params: params.inner,
            result,
            body,
        })
    }

    fn parse_decl_use(&mut self, _tree: &mut Tree) -> Result<decl::Use> {
        todo!()
    }

    fn parse_decl_type(&mut self, tree: &mut Tree) -> Result<decl::Type> {
        self.with_context(Context::DeclType, |parser| {
            let begin = expect_one!(parser, KwType)?;
            let symbol = expect_one!(parser, Ident)?;
            expect_one!(parser, Eq)?;

            let ty = parser.parse_type(tree, AllowRaiseClause::No)?;
            let last = tree.get(ty).span();

            optional!(parser, Semi);

            Ok(decl::Type {
                span: SourceSpan::new(begin.span.begin, last.end),
                symbol,
                ty: ty.into(),
            })
        })
    }

    fn parse_decl_const(&mut self, tree: &mut Tree) -> Result<decl::Const> {
        self.with_context(Context::DeclConst, |parser| {
            let begin = expect_one!(parser, KwConst)?;
            let symbol = expect_one!(parser, Ident)?;
            expect_one!(parser, Eq)?;

            let expr = parser.parse_expr(tree)?;
            let last = tree.get(expr).span();

            optional!(parser, Semi);

            Ok(decl::Const {
                span: SourceSpan::new(begin.span.begin, last.end),
                symbol,
                expr: expr.into(),
            })
        })
    }

    fn parse_decl_struct(&mut self, tree: &mut Tree) -> Result<decl::Struct> {
        self.push_context(Context::DeclStruct);

        let first = expect_one!(self, KwStruct)?;
        let symbol = expect_one!(self, Ident)?;

        self.pop_context();

        let body = self.parse_body(tree, |parser, tree| {
            parser.parse_decl(tree, AllowImport::No)
        })?;
        let last = body.span.clone();

        Ok(decl::Struct {
            span: SourceSpan::new(first.span.begin, last.end),
            symbol,
            body,
        })
    }

    fn parse_decl_import(&mut self) -> Result<decl::Import> {
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

            Ok(decl::Import {
                span: SourceSpan::new(first.span.begin, last.end),
                path,
                alias,
            })
        })
    }

    pub(crate) fn parse_decl(
        &mut self,
        tree: &mut Tree,
        allow_import: AllowImport,
    ) -> Result<DeclId> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwVar => self
                    .parse_decl_variable(tree)
                    .map(Into::into)
                    .map(|d| tree.insert(d)),
                KwFn => self
                    .parse_decl_function(tree)
                    .map(Into::into)
                    .map(|d| tree.insert(d)),
                KwUse => self
                    .parse_decl_use(tree)
                    .map(Into::into)
                    .map(|d| tree.insert(d)),
                KwType => self
                    .parse_decl_type(tree)
                    .map(Into::into)
                    .map(|d| tree.insert(d)),
                KwConst => self
                    .parse_decl_const(tree)
                    .map(Into::into)
                    .map(|d| tree.insert(d)),
                KwStruct => self
                    .parse_decl_struct(tree)
                    .map(Into::into)
                    .map(|d| tree.insert(d)),
                KwImport if allow_import.into() => self
                    .parse_decl_import()
                    .map(Into::into)
                    .map(|d| tree.insert(d)),
                _ => {
                    return Err(Error::UnexpectedToken {
                        src: self.src.clone(),
                        span: token.span.to_miette_span(),
                        expected: vec![].into(),
                        got: self.src.get_span(token.span).to_owned(),
                    });
                }
            }
        } else {
            Err(Error::UnexpectedEndOfFile {
                src: self.src.clone(),
            })
        }
    }
}
