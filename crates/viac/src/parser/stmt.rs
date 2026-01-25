/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Parser;
use super::decl::AllowImport;
use super::prelude::*;
use crate::ast::control;
use crate::ast::stmt::Stmt;

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
                        Ok(OpEq) | Ok(OpPlusEq) | Ok(OpMinusEq) | Ok(OpStarEq) | Ok(OpSlashEq)
                        | Ok(OpStarStarEq) | Ok(OpPercentEq) | Ok(OpAmpEq) | Ok(OpPipeEq) => {
                            let op = self.consume()?;
                            let rhs = self.parse_expr()?;
                            let first = expr.span;
                            let last = rhs.span;
                            Ok(Node {
                                node: Stmt::Control(
                                    control::Assign {
                                        op: op,
                                        lhs: expr.into(),
                                        rhs: rhs.into(),
                                    }
                                    .into(),
                                ),
                                span: span![first.begin, last.end],
                                attrs: vec![],
                            })
                        }
                        _ => Ok(Node {
                            node: Stmt::Expr(expr.node),
                            span: expr.span,
                            attrs: vec![],
                        }),
                    }
                }
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
