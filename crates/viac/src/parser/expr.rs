use super::prelude::*;
use crate::{
    ast::{Expr, ExprKind},
    macros::ice_unreachable,
    sema::{BinaryOp, UnaryOp},
};

yes_or_no!(enum AllowPrefix);

impl<'a> Parser<'a> {
    pub(super) fn is_expr_start(&self) -> bool {
        matches!(
            self.peek().map(|t| t.kind),
            Ok(
                Ident { .. }
                | IntLit { .. }
                | NumLit(_)
                | StrLit { .. }
                | KwTrue // literal
                | KwFalse
                | KwFn
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

    fn parse_expr_primary(&mut self, tree: &mut Tree, allow_prefix: AllowPrefix) -> Result<Expr> {
        let token = self.consume()?;
        let span = token.span;

        let expr = match token.kind {
            LParen => match self.peek()?.kind {
                RParen => {
                    self.consume()?;
                    Expr {
                        kind: ExprKind::Unit,
                        span,
                    }
                }
                _ => {
                    let expr = self.parse_expr(tree)?;
                    expect_one!(self, RParen)?;
                    expr
                }
            },
            KwTrue => Expr {
                kind: ExprKind::True,
                span,
            },
            KwFalse => Expr {
                kind: ExprKind::False,
                span,
            },
            IntLit { value, base: _ } => Expr {
                kind: ExprKind::Integer(value),
                span,
            },
            NumLit(value) => Expr {
                kind: ExprKind::Float(value),
                span,
            },
            Minus | Bang | Tilde if allow_prefix.into() => {
                let expr = self.parse_expr_primary(tree, AllowPrefix::No)?;

                Expr {
                    kind: ExprKind::Unary {
                        op: match token.kind {
                            Minus => UnaryOp::Negate,
                            Bang => UnaryOp::LogNot,
                            Tilde => UnaryOp::BitNot,
                            _ => ice_unreachable!(),
                        },
                        expr: tree.alloc_expr(expr),
                    },
                    span,
                }
            }
            _ => return Err(Error::UnexpectedToken(token.span)),
        };

        Ok(expr)
    }

    fn parse_expr_postfix(&mut self, tree: &mut Tree) -> Result<Expr> {
        // TODO
        self.parse_expr_primary(tree, AllowPrefix::Yes)
    }

    fn parse_expr_binary(&mut self, tree: &mut Tree, min_prec: u8) -> Result<Expr> {
        use BinaryOp::*;

        let mut lhs = self.parse_expr_postfix(tree)?;

        while let Ok(op) = self.peek() {
            let prec = match op.kind.prec() {
                Some(prec) if prec >= min_prec => prec,
                _ => break,
            };

            let op = self.consume()?;
            let rhs = self.parse_expr_binary(tree, prec + 1)?;

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
                        AmpAmp => LogAnd,
                        PipePipe => LogOr,
                        _ => return Err(Error::UnexpectedToken(op.span)),
                    },
                    lhs: tree.alloc_expr(lhs),
                    rhs: tree.alloc_expr(rhs),
                },
            };
        }
        Ok(lhs)
    }

    pub(crate) fn parse_expr(&mut self, tree: &mut Tree) -> Result<Expr> {
        self.parse_expr_binary(tree, 0)
    }
}
