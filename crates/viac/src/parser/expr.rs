/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::{
    ast::expr::{Expr, ExprKind},
    sema::ops::{BinaryOp, UnaryOp},
};

yes_or_no!(enum AllowPrefix);

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

    fn parse_expr_primary(&mut self, allow_prefix: AllowPrefix) -> Result<Expr> {
        let token = self.consume()?;
        let span = token.span;

        match token.kind {
            KwNone => Ok(Expr {
                kind: ExprKind::None,
                span,
            }),
            KwTrue => Ok(Expr {
                kind: ExprKind::True,
                span,
            }),
            KwFalse => Ok(Expr {
                kind: ExprKind::False,
                span,
            }),
            Int { value, base: _ } => Ok(Expr {
                kind: ExprKind::Integer(value),
                span,
            }),
            Float(value) => Ok(Expr {
                kind: ExprKind::Float(value),
                span,
            }),
            Minus | Bang | Tilde if allow_prefix.into() => {
                let expr = self.parse_expr_primary(AllowPrefix::No)?;

                Ok(Expr {
                    kind: ExprKind::Unary {
                        op: match token.kind {
                            Minus => UnaryOp::Negate,
                            Bang => UnaryOp::Not,
                            Tilde => UnaryOp::BitNot,
                            _ => unreachable!(),
                        },
                        expr: Box::new(expr),
                    },
                    span,
                })
            }
            _ => Err(Error::UnexpectedToken(token.span)),
        }
    }

    fn parse_expr_postfix(&mut self) -> Result<Expr> {
        // TODO
        self.parse_expr_primary(AllowPrefix::Yes)
    }

    fn parse_expr_binary(&mut self, min_prec: u8) -> Result<Expr> {
        use BinaryOp::*;

        let mut lhs = self.parse_expr_postfix()?;

        while let Ok(op) = self.peek() {
            let prec = match op.kind.prec() {
                Some(prec) if prec >= min_prec => prec,
                _ => break,
            };

            let op = self.consume()?;
            let rhs = self.parse_expr_binary(prec + 1)?;

            lhs = Expr {
                span: SourceSpan::new(lhs.span.begin, rhs.span.end),
                kind: ExprKind::Binary {
                    op: match op.kind {
                        Plus => Add,
                        Minus => Sub,
                        Star => Mul,
                        Slash => Div,
                        Caret => Pow,
                        Percent => Mod,
                        Amp => BitAnd,
                        Pipe => BitOr,
                        LtLt => BitShl,
                        GtGt => BitShr,
                        AmpAmp => And,
                        PipePipe => Or,
                        _ => return Err(Error::UnexpectedToken(op.span)),
                    },
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                },
            };
        }
        Ok(lhs)
    }

    pub(crate) fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_expr_binary(0)
    }
}
