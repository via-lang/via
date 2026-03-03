/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::expr::{Expr, ExprKind};

yes_or_no!(AllowPrefix);

impl Parser<'_> {
    pub(super) fn is_expr_start(&self) -> bool {
        matches!(
            self.peek().map(|t| t.kind),
            Ok(
                Ident { .. }
                | KwTrue // literal
                | KwFalse
                | KwNone
                | KwFn
                | Int { .. }
                | Float(_)
                | String { .. }
                | Minus // unary
                | Amp
                | Tilde
                | Bang
                | LParen // group or tuple
                | LBrace // map
                | LBracket // array
            )
        )
    }

    fn parse_expr_primary(&mut self) -> Result<Expr> {
        let token = self.peek()?;
        match token.kind {
            KwNone => {
                self.consume()?;
                Ok(Expr {
                    kind: ExprKind::None,
                    span: token.span,
                })
            }
            KwTrue => {
                self.consume()?;
                Ok(Expr {
                    kind: ExprKind::True,
                    span: token.span,
                })
            }
            KwFalse => {
                self.consume()?;
                Ok(Expr {
                    kind: ExprKind::False,
                    span: token.span,
                })
            }
            Int { value, base: _ } => {
                self.consume()?;
                Ok(Expr {
                    kind: ExprKind::Integer(value),
                    span: token.span,
                })
            }
            Float(value) => {
                self.consume()?;
                Ok(Expr {
                    kind: ExprKind::Float(value),
                    span: token.span,
                })
            }
            _ => Err(Error::UnexpectedToken(token.span)),
        }
    }

    fn parse_expr_postfix(&mut self) -> Result<Expr> {
        let mut expr = self.parse_expr_primary()?;
        loop {
            expr = match self.peek().map(|t| t.kind) {
                Ok(Dot) => {
                    self.consume()?;
                    let last = self.consume()?;
                    let span = SourceSpan::merge(expr.span, last.span);

                    match last.kind {
                        _ => todo!(),
                    }
                }
                _ => break Ok(expr),
            };
        }
    }

    fn parse_expr_binary(&mut self, min_prec: u8) -> Result<Expr> {
        let mut lhs = self.parse_expr_postfix()?;
        while let Ok(op) = self.peek() {
            let prec = match op.kind.prec() {
                Some(prec) if prec >= min_prec => prec,
                _ => break,
            };

            self.consume()?;
            let rhs = self.parse_expr_binary(prec + 1)?;

            // lhs = Expr::Value(
            //     expr::Binary {
            //         span: SourceSpan::new(first.begin, last.end),
            //         op,
            //         lhs: tree.insert(lhs),
            //         rhs: tree.insert(rhs),
            //     }
            //     .into(),
            // );
        }
        Ok(lhs)
    }

    pub(crate) fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_expr_binary(0)
    }
}
