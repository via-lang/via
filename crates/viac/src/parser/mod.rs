/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod attr;
pub mod context;
mod control;
mod decl;
pub mod error;
mod expr;
mod macros;
mod stmt;
mod ty;

pub(super) mod prelude {
    pub(super) use super::{
        Parser,
        context::Context,
        error::{Error, Result},
        macros::*,
    };
    pub(super) use crate::{
        lexer::token::TokenKind::{self, *},
        source::{SourceBuf, SourceSpan},
    };
}

use context::Context;
use prelude::*;

use crate::{
    ast::{
        Id, Tree,
        aux::{Nodes, Param, ParamId},
    },
    lexer::token::Token,
    module::compiler::{Compiler, state::Lexed},
};

pub struct Parser<'a> {
    src: SourceBuf,
    toks: &'a [Token],
    pos: usize,
    ctxts: Vec<Context>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &SourceBuf, toks: &'a [Token]) -> Self {
        Self {
            src: src.clone(),
            toks,
            pos: 0,
            ctxts: vec![],
        }
    }

    fn push_context(&mut self, ctx: Context) {
        self.ctxts.push(ctx);
    }

    fn pop_context(&mut self) {
        self.ctxts.pop();
    }

    fn with_context<T>(&mut self, ctx: Context, f: impl FnOnce(&mut Self) -> T) -> T {
        self.push_context(ctx);
        let result = f(self);
        self.pop_context();
        result
    }

    fn peek(&self) -> Result<Token> {
        self.toks
            .get(self.pos)
            .cloned()
            .ok_or_else(|| Error::UnexpectedEndOfFile {
                src: self.src.clone(),
            })
    }

    fn peek_ahead(&self, ahead: u32) -> Result<Token> {
        self.toks
            .get(self.pos + ahead as usize)
            .cloned()
            .ok_or_else(|| Error::UnexpectedEndOfFile {
                src: self.src.clone(),
            })
    }

    fn consume(&mut self) -> Result<Token> {
        self.peek().inspect(|_| self.pos += 1)
    }

    #[allow(private_bounds)]
    pub(super) fn parse_body<F, I>(&mut self, tree: &mut Tree, mut parse: F) -> Result<Nodes<I>>
    where
        F: FnMut(&mut Self, &mut Tree) -> Result<I>,
        I: Id,
    {
        let first = expect_one!(self, LBrace)?;
        let mut inner = vec![];

        while !check!(self, RBrace) {
            let id = parse(self, tree)?;
            inner.push(id);
        }

        let last = expect_one!(self, RBrace)?;
        Ok(Nodes {
            inner,
            span: SourceSpan::merge(first.span, last.span),
        })
    }

    pub(super) fn parse_list<F, I>(
        &mut self,
        tree: &mut Tree,
        brackets: (TokenKind, TokenKind),
        mut parse: F,
    ) -> Result<Nodes<I>>
    where
        F: FnMut(&mut Self, &mut Tree) -> Result<I>,
        I: Id,
    {
        let first = expect_one!(self, brackets.0)?;
        let mut inner = vec![];

        while !check!(self, brackets.1) {
            let id = parse(self, tree)?;
            inner.push(id);

            if !optional!(self, Comma) {
                break;
            }
        }

        let last = expect_one!(self, brackets.1)?;
        Ok(Nodes {
            inner,
            span: SourceSpan::merge(first.span, last.span),
        })
    }

    pub(super) fn parse_param(&mut self, tree: &mut Tree) -> Result<ParamId> {
        self.with_context(Context::Param, |parser| {
            let name = expect_one!(parser, Ident)?;
            let first = name.span.clone();

            expect_one!(parser, Col)?;

            let ty = parser.parse_param_ty(tree)?;
            let last = tree.get(ty).span();

            Ok(tree.insert(Param {
                span: SourceSpan::merge(first, last),
                name,
                ty: ty.into(),
            }))
        })
    }

    pub(crate) fn parse(&mut self) -> Result<Tree> {
        let mut tree = Tree::default();
        loop {
            if check!(self, EndOfFile) {
                break Ok(tree);
            }
            let stmt = self.parse_stmt(&mut tree)?;
            tree.stmts.push(stmt);
        }
    }
}

pub fn parse(c: &Compiler<Lexed>) -> Result<Tree> {
    Parser::new(c.source(), &c.stage().tt).parse()
}
