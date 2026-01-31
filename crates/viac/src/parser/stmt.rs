/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{decl::AllowImport, prelude::*};
use crate::ast::{control, stmt::Stmt};

impl Parser {
    pub(super) fn parse_stmt(&mut self) -> Result<Node<Stmt>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwBreak | KwContinue | KwReturn | KwRaise | KwWhile | KwFor | KwIf => {
                    self.parse_control().map(|node| node.map(Stmt::Control))
                }
                KwVar | KwFn | KwUse | KwType | KwConst | KwStruct | KwImport => self
                    .parse_decl(AllowImport::Yes)
                    .map(|node| node.map(Stmt::Decl)),
                _ if self.is_expr_start() => {
                    let expr = self.parse_expr()?;
                    match self.peek().map(|t| t.kind) {
                        Ok(Eq) | Ok(PlusEq) | Ok(MinusEq) | Ok(StarEq) | Ok(SlashEq)
                        | Ok(StarStarEq) | Ok(PercentEq) | Ok(AmpEq) | Ok(PipeEq) => {
                            let op = self.consume()?;
                            let rhs = self.parse_expr()?;
                            let first = expr.span.clone();
                            let last = rhs.span.clone();
                            Ok(Node {
                                node: Stmt::Control(
                                    control::Assign {
                                        op,
                                        lhs: expr.into(),
                                        rhs: rhs.into(),
                                    }
                                    .into(),
                                ),
                                span: SourceSpan::new(first.begin, last.end),
                                attrs: None,
                            })
                        }
                        _ => Ok(Node {
                            node: Stmt::Expr(expr.node),
                            span: expr.span,
                            attrs: None,
                        }),
                    }
                }
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
