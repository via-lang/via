/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{decl::AllowImport, prelude::*};
use crate::ast::{Tree, control, stmt::Stmt};

impl Parser<'_> {
    pub(super) fn parse_stmt(&mut self, tree: &mut Tree) -> Result<Stmt> {
        if let Ok(token) = self.peek() {
            match token.kind {
                KwBreak | KwContinue | KwReturn | KwRaise | KwWhile | KwFor | KwIf => {
                    self.parse_control(tree).map(Stmt::Control)
                }
                KwVar | KwFn | KwUse | KwType | KwConst | KwStruct | KwImport => {
                    self.parse_decl(tree, AllowImport::Yes).map(Stmt::Decl)
                }
                _ if self.is_expr_start() => {
                    let expr = self.parse_expr(tree)?;

                    match self.peek().map(|t| t.kind) {
                        Ok(Eq) | Ok(PlusEq) | Ok(MinusEq) | Ok(StarEq) | Ok(SlashEq)
                        | Ok(StarStarEq) | Ok(PercentEq) | Ok(AmpEq) | Ok(PipeEq) => {
                            let op = self.consume()?;
                            let rhs = self.parse_expr(tree)?;

                            let first = expr.span();
                            let last = rhs.span();

                            Ok(Stmt::Control(
                                control::Assign {
                                    span: SourceSpan::new(first.begin, last.end),
                                    op,
                                    lhs: tree.insert(expr),
                                    rhs: tree.insert(rhs),
                                }
                                .into(),
                            ))
                        }
                        _ => Ok(Stmt::Expr(expr.clone())),
                    }
                }
                _ => Err(Error::UnexpectedToken {
                    span: token.span.to_miette_span(),
                    expected: vec![].into(),
                    got: self.src.get_span(&token.span).to_owned(),
                }),
            }
        } else {
            Err(Error::UnexpectedEndOfFile {})
        }
    }
}
